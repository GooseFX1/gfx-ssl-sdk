use std::collections::HashMap;
use std::ffi::CStr;
use std::fmt::Debug;
use std::mem;
use anchor_lang::__private::bytemuck::{cast_slice, from_bytes, Pod, PodCastError, try_cast_slice};
use anyhow::anyhow;
use jupiter_core::amm::{Amm, Quote, QuoteParams, SwapLegAndAccountMetas, SwapParams};
use rust_decimal::Decimal;
use anchor_lang::AccountDeserialize;
use anchor_spl::associated_token::get_associated_token_address;
use anchor_spl::token::TokenAccount;
use jupiter::jupiter_override::{Swap, SwapLeg};
use lazy_static::lazy_static;
use pyth_sdk_solana::state::{load_price_account, PriceAccount};
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::pubkey;
use gfx_ssl_sdk::{Pair, PDAIdentifier, skey, SSL};
use crate::ssl::FEE_COLLECTOR;
use crate::ssl::instructions::{SSLInstructionContext, swap_account_metas};
use crate::ssl::state::get_pair_blocking;

const DISCRIMINANT: usize = 8;

// TODO Put the correct value here, maybe make an enum for this.
const CONTROLLER: Pubkey = pubkey!("11111111111111111111111111111111");

lazy_static! {
    pub static ref MINTS: HashMap<Pubkey, &'static str> = {
        let mut mints = HashMap::new();
        vec![
            (pubkey!("So11111111111111111111111111111111111111112"), "WSOL"),
            (pubkey!("7vfCXTUXx5WJV5JADk17DUJ4ksgau7utNKj4b963voxs"), "ETH"),
            (pubkey!("mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So"), "MSOL"),
            (pubkey!("SRMuApVNdxXokk5GT7XD5cUUgXMBCoAz2LHeuAoKWRt"), "SRM"),
            (pubkey!("7i5KKsX2weiTkry7jA4ZwSuXGhs5eJBEjY8vVxR4pfRx"), "GMT"),
            (pubkey!("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB"), "USDT"),
            (pubkey!("orcaEKTdK7LKz57vaAYr9QeNsVEPfiu6QeMU1kektZE"), "ORCA"),
            (pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"), "USDC"),
            (pubkey!("7dHbWXmci3dT8UFYWYZweBLXgycu7Y3iL6trKn1Y7ARj"), "STSOL"),
            (pubkey!("9n4nbM75f5Ui33ZbPYXn59EwSgE8CGsHtAeTH5YFeJ9E"), "BTC"),
            (pubkey!("6LNeTYMqtNm1pBFN8PfhQaoLyegAH8GD32WmHU9erXKN"), "APT"),
        ].into_iter()
        .for_each(|(mint, name)| {
            mints.insert(mint, name);
        });
        mints
    };
    pub static ref RESERVE_MINTS: Vec<Pubkey> = {
        MINTS.keys().map(|p| *p).collect()
    };
}

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

#[repr(C)]
pub struct OracleEntry([u8; 32], PriceAccount);

extern "C" {
    fn quote(
        ssl_in: &[u8; mem::size_of::<SSL>() + DISCRIMINANT],
        ssl_out: &[u8; mem::size_of::<SSL>() + DISCRIMINANT],
        pair: &[u8; mem::size_of::<Pair>() + DISCRIMINANT],
        liability_in: u64,
        liability_out: u64,
        swapped_liability_in: u64,
        swapped_liability_out: u64,
        oracles: *const OracleEntry,
        num_oracles: usize,
        amount_in: u64,
    ) -> QuoteResult;
}

pub fn pair_from_mints_blocking(base: Pubkey, quote: Pubkey, client: &RpcClient) -> crate::error::Result<Pair> {
    let pair_pubkey = Pair::get_address(
        &[
            CONTROLLER.as_ref(),
            skey::<_, true>(&base, &quote).as_ref(),
            skey::<_, false>(&base, &quote).as_ref(),
        ]
    );
    get_pair_blocking(&pair_pubkey, client)
}

#[derive(Debug, Copy, Clone)]
enum AmmAccountState {
    /// If there have been no account state updates via a call to
    /// [Amm::update], we cannot provide any quotes.
    Empty,
    /// After the first [Amm::update], we know which oracles we need to fetch,
    /// but we haven't fetched them yet. A second [Amm::update] is required to crank
    /// the [GfxAmm] into a ready state.
    NeedOracleData,
    Ok,
}

/// Struct that implements the `jupiter_core::amm::Amm` trait.
///
/// This struct requires two calls to [Amm::get_accounts_to_update] and [Amm::update],
/// as some of the accounts required cannot be known for a given pair until
/// other account state is fetched first.
///
/// ```rust
/// use solana_program::pubkey::Pubkey;
/// use solana_sdk::pubkey;
/// use gfx_ssl_sdk_rust::jupiter::GfxAmm;
///
/// let base: Pubkey = pubkey!("GFX1ZjR2P15tmrSwow6FjyDYcEkoFb4p4gJCpLBjaxHD");
/// let quote: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
/// let gfx_amm = GfxAmm::new(base, quote);
/// ```
///
#[derive(Debug, Clone)]
pub struct GfxAmm {
    /// "XXX/YYY" where:
    /// XXX = liability RT
    /// YYY = swapped liability RT
    label: String,
    /// This object's state must be cranked twice before you can pull quotes from it.
    /// This enum keeps track of whether that's occurred.
    account_state: AmmAccountState,
    ssl_a: Option<SSL>,
    ssl_a_mint: Pubkey,
    ssl_a_data: [u8; mem::size_of::<SSL>() + DISCRIMINANT],
    ssl_a_pubkey: Pubkey,
    ssl_b: Option<SSL>,
    ssl_b_mint: Pubkey,
    ssl_b_data: [u8; mem::size_of::<SSL>() + DISCRIMINANT],
    ssl_b_pubkey: Pubkey,
    pair: Option<Pair>,
    pair_data: [u8; mem::size_of::<Pair>() + DISCRIMINANT],
    pair_pubkey: Pubkey,
    ssl_a_vault_a: Pubkey,
    ssl_a_vault_a_balance: u64,
    ssl_a_vault_b: Pubkey,
    ssl_a_vault_b_balance: u64,
    ssl_b_vault_a: Pubkey,
    ssl_b_vault_a_balance: u64,
    ssl_b_vault_b: Pubkey,
    ssl_b_vault_b_balance: u64,
    oracles: HashMap<Pubkey, PriceAccount>,
}

impl GfxAmm {
    pub fn new(base_mint: Pubkey, quote_mint: Pubkey) -> anyhow::Result<Self> {
        // Arrange them in order first
        let (ssl_a_mint, ssl_b_mint) = if base_mint > quote_mint {
            (quote_mint, base_mint)
        } else {
            (base_mint, quote_mint)
        };

        // Get label, and ensure these pairs are offered.
        let label_front = MINTS.get(&ssl_a_mint)
            .ok_or(anyhow!("This mint is not offered {}", ssl_a_mint))?;
        let label_back = MINTS.get(&ssl_b_mint)
            .ok_or(anyhow!("This mint is not offered {}", ssl_b_mint))?;
        let label = format!("{}/{}", label_front, label_back);

        // Calculate PDAs of GFX accounts
        let ssl_a_pubkey = SSL::get_address(
            &[
                CONTROLLER.as_ref(),
                ssl_a_mint.as_ref(),
            ]
        );
        let ssl_b_pubkey = SSL::get_address(
            &[
                CONTROLLER.as_ref(),
                ssl_b_mint.as_ref(),
            ]
        );
        let pair_pubkey = Pair::get_address(
            &[
                CONTROLLER.as_ref(),
                skey::<_, true>(&ssl_a_mint, &ssl_b_mint).as_ref(),
                skey::<_, false>(&ssl_a_mint, &ssl_b_mint).as_ref(),
            ]
        );
        let ssl_a_vault_a = get_associated_token_address(
            &ssl_a_pubkey,
            &ssl_a_mint,
        );
        let ssl_a_vault_b = get_associated_token_address(
            &ssl_a_pubkey,
            &ssl_b_mint,
        );
        let ssl_b_vault_a = get_associated_token_address(
            &ssl_b_pubkey,
            &ssl_a_mint,
        );
        let ssl_b_vault_b = get_associated_token_address(
            &ssl_b_pubkey,
            &ssl_b_mint,
        );

        Ok(Self {
            label,
            account_state: AmmAccountState::Empty,
            ssl_a: None,
            ssl_a_mint,
            ssl_a_data: [0; DISCRIMINANT + mem::size_of::<SSL>()],
            ssl_a_pubkey,
            ssl_b: None,
            ssl_b_mint,
            ssl_b_data: [0; DISCRIMINANT + mem::size_of::<SSL>()],
            ssl_b_pubkey,
            pair: None,
            pair_data: [0; DISCRIMINANT + mem::size_of::<Pair>()],
            pair_pubkey,
            ssl_a_vault_a,
            ssl_a_vault_a_balance: 0,
            ssl_a_vault_b,
            ssl_a_vault_b_balance: 0,
            ssl_b_vault_a,
            ssl_b_vault_a_balance: 0,
            ssl_b_vault_b,
            ssl_b_vault_b_balance: 0,
            oracles: Default::default(),
        })
    }
}

impl Amm for GfxAmm {
    fn label(&self) -> String {
        self.label.clone()
    }

    fn key(&self) -> Pubkey {
        self.pair_pubkey
    }

    fn get_reserve_mints(&self) -> Vec<Pubkey> {
        RESERVE_MINTS.clone()
    }

    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        let mut accounts = vec![
            self.ssl_a_pubkey,
            self.ssl_b_pubkey,
            self.pair_pubkey,
            self.ssl_a_vault_a,
            self.ssl_a_vault_b,
            self.ssl_b_vault_a,
            self.ssl_b_vault_b,
        ];
        // TODO Should we store a list of oracles on all updates instead of this?
        if let Some(pair) = &self.pair {
            accounts.extend::<Vec<_>>(
                pair.oracles
                    .iter()
                    .map(|o| o.path
                            .iter()
                            .map(|p| p.0)
                    )
                    .flatten()
                    .collect()
            );
        }
        accounts
    }

    fn update(&mut self, accounts_map: &HashMap<Pubkey, Vec<u8>>) -> anyhow::Result<()> {
        let update_token_account = |amount: &mut u64, data: &mut &[u8]| {
            let token_account = TokenAccount::try_deserialize(data)?;
            *amount = token_account.amount;
            Ok::<_, anyhow::Error>(())
        };
        for (pubkey, data) in accounts_map {
            // TODO Code readability?
            if *pubkey == self.ssl_a_pubkey {
                let data: [u8; mem::size_of::<SSL>() + DISCRIMINANT] = data.clone().try_into()
                    .map_err(|_| anyhow!("Invalid data size for SSL"))?;
                self.ssl_a_data = data;
                self.ssl_a = Some(SSL::try_deserialize(&mut data.as_slice())?);
            } else if *pubkey == self.ssl_b_pubkey {
                let data: [u8; mem::size_of::<SSL>() + DISCRIMINANT] = data.clone().try_into()
                    .map_err(|_| anyhow!("Invalid data size for SSL"))?;
                self.ssl_b_data = data;
                self.ssl_b = Some(SSL::try_deserialize(&mut data.as_slice())?);
            } else if *pubkey == self.pair_pubkey {
                let data: [u8; mem::size_of::<Pair>() + DISCRIMINANT] = data.clone().try_into()
                    .map_err(|_| anyhow!("Invalid data size for Pair"))?;
                self.pair_data = data;
                self.pair = Some(Pair::try_deserialize(&mut data.as_slice())?);
            } else if *pubkey == self.ssl_a_vault_a {
                update_token_account(&mut self.ssl_a_vault_a_balance, &mut data.as_slice())?;
            } else if *pubkey == self.ssl_a_vault_b {
                update_token_account(&mut self.ssl_a_vault_b_balance, &mut data.as_slice())?;
            } else if *pubkey == self.ssl_b_vault_a {
                update_token_account(&mut self.ssl_b_vault_a_balance, &mut data.as_slice())?;
            } else if *pubkey == self.ssl_b_vault_b {
                update_token_account(&mut self.ssl_b_vault_b_balance, &mut data.as_slice())?;
            } else {
                // Assume it's an oracle
                // TODO This assumption is tenuous, should maybe make this safer
                //    by saving a list of oracles. Depends on whether you think
                //    we should fail loudly in the case where they pass in
                //    an account that we do not need to update.
                let price_account = load_price_account(&mut data.as_slice())
                     .map_err(|_| anyhow!("Invalid oracle data"))?;
                self.oracles.insert(*pubkey, *price_account);
            }
        }
        match self.account_state {
            AmmAccountState::Empty => {
                self.account_state = AmmAccountState::NeedOracleData;
            }
            AmmAccountState::NeedOracleData => {
                self.account_state = AmmAccountState::Ok;
            }
            AmmAccountState::Ok => {}
        }
        Ok(())
    }

    // TODO Review whether this logic is correct
    fn quote(&self, quote_params: &QuoteParams) -> anyhow::Result<Quote> {
        match self.account_state {
            AmmAccountState::Ok => {},
            _ => {
                return Err(anyhow!("Cannot quote until account state is populated"));
            }
        }
        // Orient each side of the pair as "in" our "out".
        // Keep a boolean flag that helps keep track of whether to flip
        // other arguments later in this function
        let mut is_reversed = false;
        if quote_params.input_mint == self.ssl_b_mint && quote_params.output_mint == self.ssl_a_mint {
            is_reversed = true;
        } else if quote_params.input_mint != self.ssl_a_mint || quote_params.output_mint != self.ssl_b_mint {
            return Err(anyhow!("Invalid quote params, input and output mints do not match this Amm pair"));
        }

        let (
            ssl_in,
            ssl_out,
            liability_in,
            liability_out,
            swapped_liability_in,
            swapped_liability_out,
        ) = if is_reversed {
            (
                self.ssl_a_data,
                self.ssl_b_data,
                self.ssl_a_vault_a_balance,
                self.ssl_a_vault_b_balance,
                self.ssl_b_vault_a_balance,
                self.ssl_b_vault_b_balance,
            )
        } else {
            (
                self.ssl_b_data,
                self.ssl_a_data,
                self.ssl_b_vault_b_balance,
                self.ssl_b_vault_a_balance,
                self.ssl_a_vault_b_balance,
                self.ssl_a_vault_a_balance,
            )
        };

        let oracles = self.oracles
            .iter()
            .map(|(pubkey, act)| {
                let mut pubkey_arr: [u8; 32] = Default::default();
                pubkey_arr.copy_from_slice(pubkey.as_ref());
                OracleEntry(pubkey_arr, *act)
            })
            .collect::<Vec<_>>()
            .as_slice()
            .as_ptr();
        match unsafe {
            quote(
                &ssl_in,
                &ssl_out,
                &self.pair_data,
                liability_in,
                liability_out,
                swapped_liability_in,
                swapped_liability_out,
                oracles,
                self.oracles.len(),
                quote_params.in_amount,
            )
        } {
            QuoteResult::Ok(swap_result) => {
                let fee_pct = if !is_reversed {
                    self.pair.unwrap().fee_rate.0
                } else {
                    self.pair.unwrap().fee_rate.1
                };
                let fee_pct = Decimal::new(fee_pct.into(), 4);
                let price_impact_pct = Decimal::from_f64_retain(swap_result.price_impact)
                    .ok_or(anyhow!("Invalid price impact pct decimal"))?;

                // Here, fee_mint is always the input mint
                let fee_mint = if !is_reversed {
                    self.ssl_a_mint
                } else {
                    self.ssl_b_mint
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
            account_metas: swap_account_metas(
                &SSLInstructionContext::new(
                    CONTROLLER,
                    Default::default(), // Doesn't matter for this instruction, uses in/out
                    swap_params.user_transfer_authority,
                ),
                &swap_params.source_mint,
                &swap_params.destination_mint,
                &FEE_COLLECTOR,
            ),
        })
    }

    fn clone_amm(&self) -> Box<dyn Amm + Send + Sync> {
        Box::new(self.clone())
    }
}