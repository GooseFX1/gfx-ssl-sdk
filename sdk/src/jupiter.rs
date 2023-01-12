use std::collections::HashMap;
use std::ffi::CStr;
use std::fmt::Debug;
use std::mem;
use anyhow::anyhow;
use jupiter_core::amm::{Amm, Quote, QuoteParams, SwapLegAndAccountMetas, SwapParams};
use rust_decimal::Decimal;
use anchor_lang::AccountDeserialize;
use jupiter::jupiter_override::{Swap, SwapLeg};
use jupiter::jupiter_override::SwapLeg::Swap;
use solana_program::pubkey::Pubkey;
use gfx_ssl_sdk::{Pair, SSL};

const DISCRIMINANT: usize = 8;

#[repr(C)]
pub struct SwapResult {
    pub amount_in: u64,
    pub fee_paid: u64,
    pub amount_out: u64,
    pub price_impact: f64,
    pub swap_price: f64,
    pub insta_price: f64,
    pub oracle_price: f64,
    pub iter: u32,
}

#[allow(dead_code)]
#[repr(C)]
enum QuoteResult {
    Ok(SwapResult),
    Error(*mut i8),
}

extern "C" {
    fn quote(
        ssl_in: &[u8; mem::size_of::<SSL>() + DISCRIMINANT],
        ssl_out: &[u8; mem::size_of::<SSL>() + DISCRIMINANT],
        pair: &[u8; mem::size_of::<Pair>() + DISCRIMINANT],
        liability_in: u64,
        liability_out: u64,
        swapped_liability_in: u64,
        swapped_liability_out: u64,
        amount_in: u64,
    ) -> QuoteResult;
}

#[derive(Debug, Clone)]
pub struct GfxAmm {
    /// "XXX/YYY" where:
    /// XXX = liability RT
    /// YYY = swapped liability RT
    label: String,
    liability: SSL,
    liability_data: [u8; mem::size_of::<SSL>() + DISCRIMINANT],
    liability_pubkey: Pubkey,
    swapped_liability: SSL,
    swapped_liability_data: [u8; mem::size_of::<SSL>() + DISCRIMINANT],
    swapped_liability_pubkey: Pubkey,
    pair: Pair,
    pair_data: [u8; mem::size_of::<Pair>() + DISCRIMINANT],
    pair_pubkey: Pubkey,
    // TODO There are more accounts to save here -- vaults for SSL in and SSL out
}

impl GfxAmm {
    // TODO don't distinguish liability / swapped_liability,
    //   only do so when you get input to the quote function
    pub fn new(
        label: String,
        liability_pubkey: Pubkey,
        liability_data: Vec<u8>,
        swapped_liability_pubkey: Pubkey,
        swapped_liability_data: Vec<u8>,
        pair_pubkey: Pubkey,
        pair_data: Vec<u8>,
    ) -> anyhow::Result<Self> {
        let liability = SSL::try_deserialize(&mut liability_data.as_slice())
            .map_err(|_| anyhow!("Invalid account data for SSL"))?;
        let liability_data: [u8; mem::size_of::<SSL>() + DISCRIMINANT] = liability_data
            .try_into().map_err(|_| anyhow!("Invalid data size for SSL account"))?;
        let swapped_liability = SSL::try_deserialize(&mut swapped_liability_data.as_slice())
            .map_err(|_| anyhow!("Invalid account data for SSL"))?;
        let swapped_liability_data: [u8; mem::size_of::<SSL>() + DISCRIMINANT] = swapped_liability_data
            .try_into().map_err(|_| anyhow!("Invalid data size for SSL account"))?;
        let pair = Pair::try_deserialize(&mut pair_data.as_slice())
            .map_err(|_| anyhow!("Invalid account data for Pair"))?;
        let pair_data: [u8; mem::size_of::<Pair>() + DISCRIMINANT] = pair_data
            .try_into().map_err(|_| anyhow!("Invalid data size for Pair account"))?;
        if pair.mints != (liability_pubkey, swapped_liability_pubkey) {
            return Err(anyhow!("Invalid pubkeys"));
        }
        Ok(Self {
            label,
            liability,
            liability_data,
            liability_pubkey,
            swapped_liability,
            swapped_liability_data,
            swapped_liability_pubkey,
            pair,
            pair_data,
            pair_pubkey,
        })
    }
}

impl Amm for GfxAmm {
    fn label(&self) -> String {
        // TODO Lookup table to convert Mint pubkey to Symbol
        //   manually maintained in this SDK repo.
        self.label.clone()
    }

    fn key(&self) -> Pubkey {
        self.pair_pubkey.clone()
    }

    fn get_reserve_mints(&self) -> Vec<Pubkey> {
        vec![
            self.liability.mint,
            self.swapped_liability.mint,
        ]
    }

    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        vec![
            self.liability_pubkey.clone(),
            self.swapped_liability_pubkey.clone(),
            self.pair_pubkey.clone(),
            // TODO SSL Vaults
            // TODO Also all the oracles stored in the pair
        ]
    }

    fn update(&mut self, accounts_map: &HashMap<Pubkey, Vec<u8>>) -> anyhow::Result<()> {
        for (pubkey, data) in accounts_map {
            if *pubkey == self.liability_pubkey {
                let data: [u8; mem::size_of::<SSL>() + DISCRIMINANT] = data.clone().try_into()
                    .map_err(|_| anyhow!("Invalid data size for SSL"))?;
                self.liability_data = data;
                self.liability = SSL::try_deserialize(&mut data.as_slice())?;
            } else if *pubkey == self.swapped_liability_pubkey {
                let data: [u8; mem::size_of::<SSL>() + DISCRIMINANT] = data.clone().try_into()
                    .map_err(|_| anyhow!("Invalid data size for SSL"))?;
                self.swapped_liability_data = data;
                self.swapped_liability = SSL::try_deserialize(&mut data.as_slice())?;
            } else if *pubkey == self.pair_pubkey {
                let data: [u8; mem::size_of::<Pair>() + DISCRIMINANT] = data.clone().try_into()
                    .map_err(|_| anyhow!("Invalid data size for Pair"))?;
                self.pair_data = data;
                self.pair = Pair::try_deserialize(&mut data.as_slice())?;
            } else {
                return Err(anyhow!("cannot update data on unknown key"));
            }
        }
        Ok(())
    }

    // TODO Review whether this logic is correct
    fn quote(&self, quote_params: &QuoteParams) -> anyhow::Result<Quote> {
        let mut ssl_in = &self.liability_data;
        let mut ssl_out = &self.swapped_liability_data;
        let mut is_reversed = false;
        if quote_params.input_mint == self.swapped_liability.mint {
            is_reversed = true;
            ssl_in = &self.swapped_liability_data;
            ssl_out = &self.liability_data;
        }
        // TODO Explicitly check that both mints match (ssl_in == input_mint)
        // TODO No zeroes, instead it should be token vault balances for SSL in vault,
        //   SSL out vault
        match unsafe {
            quote(
                ssl_in,
                ssl_out,
                &self.pair_data,
                if !is_reversed { quote_params.in_amount } else { 0 },
                0,
                if is_reversed { quote_params.in_amount } else { 0 },
                0,
                quote_params.in_amount,
            )
        } {
            QuoteResult::Ok(swap_result) => {
                // To divide u64 and convert to Decimal type,
                // we'll first convert to i128, then to Decimal, then divide.
                let fee_paid: i128 = swap_result.fee_paid.into();
                let fee_paid = Decimal::from_i128_with_scale(fee_paid, 0);
                let amount_in: i128 = swap_result.amount_in.into();
                let amount_in = Decimal::from_i128_with_scale(amount_in, 0);
                // TODO How to convert this to a Decimal?
                // TODO Explicitly get the correct index based on input_mint
                let fee_pct = self.pair.fee_rate.0;
                // TODO turn fee_pct into Decimal, scale = 4
                // 40bps = 0.0040
                let fee_pct: Decimal = fee_paid / amount_in;
                // Price impact can be directly converted to decimal.
                let price_impact_pct = Decimal::from_f64_retain(swap_result.price_impact)
                    .ok_or(anyhow!("Invalid price impact pct decimal"))?;

                // Here, fee_mint is always the input mint
                let fee_mint = if !is_reversed {
                    self.liability_pubkey.clone()
                } else {
                    self.swapped_liability_pubkey.clone()
                };
                let quote = Quote {
                    not_enough_liquidity: false,
                    min_in_amount: None,
                    min_out_amount: None,
                    in_amount: swap_result.amount_in,
                    out_amount: swap_result.amount_out,
                    fee_amount: swap_result.fee_paid,
                    fee_mint,
                    fee_pct,
                    price_impact_pct,
                };
                Ok(quote)
            }
            QuoteResult::Error(err) => {
                unsafe {
                    let c_str = CStr::from_ptr(err);
                    let rust_str = c_str.to_str().expect("bad string encoding");
                    Err(anyhow!("{}", rust_str))
                }
            }
        }
    }

    fn get_swap_leg_and_account_metas(&self, swap_params: &SwapParams) -> anyhow::Result<SwapLegAndAccountMetas> {
        Ok(SwapLegAndAccountMetas {
            swap_leg: SwapLeg::Swap {
                swap: Swap::GooseFX
            },
            account_metas: gfx_ssl_sdk::accounts::Swap { ... }.to_account_metas(),
        })
    }

    fn clone_amm(&self) -> Box<dyn Amm + Send + Sync> {
        Box::new(self.clone())
    }
}