use crate::ssl::instructions::{swap_account_metas, SSLInstructionContext};
use crate::ssl::FEE_COLLECTOR;
use anchor_lang::AccountDeserialize;
use anchor_spl::associated_token::get_associated_token_address;
use anchor_spl::token::TokenAccount;
use anyhow::anyhow;
use gfx_ssl_interface::{skey, PDAIdentifier, Pair, SSL};
use jupiter::jupiter_override::{Swap, SwapLeg};
use jupiter_core::amm::{Amm, Quote, QuoteParams, SwapLegAndAccountMetas, SwapParams};
use lazy_static::lazy_static;
use pyth_sdk_solana::state::{load_price_account, PriceAccount};
use rust_decimal::Decimal;
use solana_program::pubkey::Pubkey;
use solana_sdk::pubkey;
use std::collections::HashMap;
use std::ffi::CStr;
use std::fmt::Debug;
use std::mem;

const DISCRIMINANT: usize = 8;

/// The current mainnet value
const CONTROLLER: Pubkey = pubkey!("8CxKnuJeoeQXFwiG6XiGY2akBjvJA5k3bE52BfnuEmNQ");

lazy_static! {
    /// The tokens currently offered on SSL mainnet,
    /// paired to their human-readable names.
    pub static ref MINTS: HashMap<Pubkey, &'static str> = {
        let mut mints = HashMap::new();
        vec![
            (
                pubkey!("So11111111111111111111111111111111111111112"),
                "WSOL",
            ),
            (
                pubkey!("7vfCXTUXx5WJV5JADk17DUJ4ksgau7utNKj4b963voxs"),
                "ETH",
            ),
            (
                pubkey!("mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So"),
                "MSOL",
            ),
            (
                pubkey!("SRMuApVNdxXokk5GT7XD5cUUgXMBCoAz2LHeuAoKWRt"),
                "SRM",
            ),
            (
                pubkey!("7i5KKsX2weiTkry7jA4ZwSuXGhs5eJBEjY8vVxR4pfRx"),
                "GMT",
            ),
            (
                pubkey!("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB"),
                "USDT",
            ),
            (
                pubkey!("orcaEKTdK7LKz57vaAYr9QeNsVEPfiu6QeMU1kektZE"),
                "ORCA",
            ),
            (
                pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
                "USDC",
            ),
            (
                pubkey!("7dHbWXmci3dT8UFYWYZweBLXgycu7Y3iL6trKn1Y7ARj"),
                "STSOL",
            ),
            (
                pubkey!("9n4nbM75f5Ui33ZbPYXn59EwSgE8CGsHtAeTH5YFeJ9E"),
                "BTC",
            ),
            (
                pubkey!("6LNeTYMqtNm1pBFN8PfhQaoLyegAH8GD32WmHU9erXKN"),
                "APT",
            ),
        ]
        .into_iter()
        .for_each(|(mint, name)| {
            mints.insert(mint, name);
        });
        mints
    };
    /// The tokens currently offered on SSL mainnet.
    pub static ref RESERVE_MINTS: Vec<Pubkey> = MINTS.keys().map(|p| *p).collect();

    /// Oracle addresses for various tokens against USD,
    /// indexed by their human readable names.
    pub static ref ORACLE_USD_ADDRESSES: HashMap<&'static str, Pubkey> = {
        let mut paths = HashMap::new();
        vec![
            ("WSOL", pubkey!("H6ARHf6YXhGYeQfUzQNGk6rDNnLBQKrenN712K4AQJEG")),
            ("ETH", pubkey!("JBu1AL4obBcCMqKBBxhpWCNUt136ijcuMZLFvTP7iWdB")),
            ("MSOL", pubkey!("E4v1BBgoso9s64TQvmyownAVJbhbEPGyzA3qn4n46qj9")),
            ("SRM", pubkey!("3NBReDRTLKMQEKiLD5tGcx4kXbTf88b7f2xLS9UuGjym")),
            ("GMT", pubkey!("DZYZkJcFJThN9nZy4nK3hrHra1LaWeiyoZ9SMdLFEFpY")),
            ("USDT", pubkey!("3vxLXJqLqF3JG5TCbYycbKWRBbCJQLxQmBGCkyqEEefL")),
            ("ORCA", pubkey!("4ivThkX8uRxBpHsdWSqyXYihzKF3zpRGAUCqyuagnLoV")),
            ("USDC", pubkey!("Gnt27xtC473ZT2Mw5u8wZ68Z3gULkSTb5DuxJy7eJotD")),
            ("STSOL", pubkey!("Bt1hEbY62aMriY1SyQqbeZbm8VmSbQVGBFzSzMuVNWzN")),
            ("BTC", pubkey!("GVXRSBjFk6e6J3NbVPXohDJetcTjaeeuykUpbQF8UoMU")),
            ("APT", pubkey!("FNNvb1AFDnDVPkocEri8mWbJ1952HQZtFLuwPiUjSJQ")),
        ]
        .into_iter()
        .for_each(|(label, pubkey)| {
            paths.insert(label, pubkey);
        });
        paths
    };
}

/// The tagged data for the "Ok" variant of [QuoteResult].
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

/// The output type for the FFI call used in the implementation of[Amm::quote]
/// for [GfxAmm].
#[allow(dead_code)]
#[repr(C)]
enum QuoteResult {
    Ok(SwapResult),
    Error(*mut i8),
}

/// A safe type to pass across the FFI boundary,
/// we pass a collection of these oracle addresses and account data.
#[derive(Debug)]
#[repr(C)]
pub struct OracleEntry([u8; 32], PriceAccount);

extern "C" {
    /// A function in a pre-compiled dylib that does
    /// the heavy lifting to derive an accurate swap quote.
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

/// Struct that implements the `jupiter_core::amm::Amm` trait.
///
/// ```rust
/// use solana_program::pubkey::Pubkey;
/// use solana_sdk::pubkey;
/// use gfx_ssl_sdk::jupiter::GfxAmm;
///
/// let base: Pubkey = pubkey!("GFX1ZjR2P15tmrSwow6FjyDYcEkoFb4p4gJCpLBjaxHD");
/// let quote: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
/// let gfx_amm = GfxAmm::new(base, quote);
/// ```
///
#[derive(Debug, Clone)]
pub struct GfxAmm {
    label: String,
    /// This object's state must be cranked twice before you can pull quotes from it.
    /// This enum keeps track of whether that's occurred.
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
    // Indexed by Pubkey of the [PriceAccount].
    oracles: HashMap<Pubkey, PriceAccount>,
    oracle_addresses: Vec<Pubkey>,
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
        let label_front = MINTS
            .get(&ssl_a_mint)
            .ok_or(anyhow!("This mint is not offered {}", ssl_a_mint))?;
        let label_back = MINTS
            .get(&ssl_b_mint)
            .ok_or(anyhow!("This mint is not offered {}", ssl_b_mint))?;
        let label = format!("{}/{}", &label_front, &label_back);

        // Oracle addresses
        let mut oracle_addresses = Vec::new();
        oracle_addresses.push(
          *ORACLE_USD_ADDRESSES.get(label_front).unwrap(),
        );
        oracle_addresses.push(
            *ORACLE_USD_ADDRESSES.get(label_back).unwrap(),
        );

        // Calculate PDAs of GFX accounts
        let ssl_a_pubkey = SSL::get_address(&[CONTROLLER.as_ref(), ssl_a_mint.as_ref()]);
        let ssl_b_pubkey = SSL::get_address(&[CONTROLLER.as_ref(), ssl_b_mint.as_ref()]);
        let pair_pubkey = Pair::get_address(&[
            CONTROLLER.as_ref(),
            skey::<_, true>(&ssl_a_mint, &ssl_b_mint).as_ref(),
            skey::<_, false>(&ssl_a_mint, &ssl_b_mint).as_ref(),
        ]);
        let ssl_a_vault_a = get_associated_token_address(&ssl_a_pubkey, &ssl_a_mint);
        let ssl_a_vault_b = get_associated_token_address(&ssl_a_pubkey, &ssl_b_mint);
        let ssl_b_vault_a = get_associated_token_address(&ssl_b_pubkey, &ssl_a_mint);
        let ssl_b_vault_b = get_associated_token_address(&ssl_b_pubkey, &ssl_b_mint);

        Ok(Self {
            label,
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
            oracle_addresses,
        })
    }
}

impl Amm for GfxAmm {
    /// Human-readable name for the Amm pair.
    fn label(&self) -> String {
        self.label.clone()
    }

    /// Get a pubkey to represent the Amm as a whole.
    fn key(&self) -> Pubkey {
        self.pair_pubkey
    }

    /// Fetches mints offered by GFX for swap.
    fn get_reserve_mints(&self) -> Vec<Pubkey> {
        RESERVE_MINTS.clone()
    }

    /// Fetch all accounts required for providing accurate quotes
    /// and swap instructions.
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
        accounts.extend(&self.oracle_addresses);
        accounts
    }

    /// Update account state
    fn update(&mut self, accounts_map: &HashMap<Pubkey, Vec<u8>>) -> anyhow::Result<()> {
        let update_token_account = |amount: &mut u64, data: &mut &[u8]| {
            let token_account = TokenAccount::try_deserialize(data)?;
            *amount = token_account.amount;
            Ok::<_, anyhow::Error>(())
        };
        for (pubkey, data) in accounts_map {
            if *pubkey == self.ssl_a_pubkey {
                let data: [u8; mem::size_of::<SSL>() + DISCRIMINANT] = data
                    .clone()
                    .try_into()
                    .map_err(|_| anyhow!("Invalid data size for SSL"))?;
                self.ssl_a_data = data;
                self.ssl_a = Some(SSL::try_deserialize(&mut data.as_slice())?);
            } else if *pubkey == self.ssl_b_pubkey {
                let data: [u8; mem::size_of::<SSL>() + DISCRIMINANT] = data
                    .clone()
                    .try_into()
                    .map_err(|_| anyhow!("Invalid data size for SSL"))?;
                self.ssl_b_data = data;
                self.ssl_b = Some(SSL::try_deserialize(&mut data.as_slice())?);
            } else if *pubkey == self.pair_pubkey {
                let data: [u8; mem::size_of::<Pair>() + DISCRIMINANT] = data
                    .clone()
                    .try_into()
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
                let price_account = load_price_account(&mut data.as_slice())
                    .map_err(|_| anyhow!("Invalid oracle data"))?;
                self.oracles.insert(*pubkey, *price_account);
            }
        }
        Ok(())
    }

    /// Get a swap quote
    fn quote(&self, quote_params: &QuoteParams) -> anyhow::Result<Quote> {
        if self.pair.is_none() {
            return Err(anyhow!("Account state is not initialized"));
        }
        // Orient each side of the pair as "in" our "out".
        // Keep a boolean flag that helps keep track of whether to flip
        // other arguments later in this function
        let mut is_reversed = false;
        if quote_params.input_mint == self.ssl_b_mint && quote_params.output_mint == self.ssl_a_mint
        {
            is_reversed = true;
        } else if quote_params.input_mint != self.ssl_a_mint
            || quote_params.output_mint != self.ssl_b_mint
        {
            return Err(anyhow!(
                "Invalid quote params, input and output mints do not match this Amm pair"
            ));
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

        let mut oracles: Vec<OracleEntry> = self
            .oracles
            .iter()
            .map(|(pubkey, act)| {
                let mut pubkey_arr: [u8; 32] = Default::default();
                pubkey_arr.copy_from_slice(pubkey.as_ref());
                OracleEntry(pubkey_arr, *act)
            })
            .collect();

        if is_reversed {
            oracles.reverse();
        }

        match unsafe {
            quote(
                &ssl_in,
                &ssl_out,
                &self.pair_data,
                liability_in,
                liability_out,
                swapped_liability_in,
                swapped_liability_out,
                oracles.as_ptr(),
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
            QuoteResult::Error(err) => unsafe {
                let c_str = CStr::from_ptr(err);
                let rust_str = c_str.to_str().expect("bad string encoding");
                Err(anyhow!("{}", rust_str))
            },
        }
    }

    /// Get account metas for a GFX swap instruction,
    /// and additional metadata such as the variant of [SwapLeg],
    /// and the platform facilitating the swap (GFX in this case).
    fn get_swap_leg_and_account_metas(
        &self,
        swap_params: &SwapParams,
    ) -> anyhow::Result<SwapLegAndAccountMetas> {
        Ok(SwapLegAndAccountMetas {
            swap_leg: SwapLeg::Swap {
                swap: Swap::GooseFX,
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

    /// Clone this object in a [Box].
    fn clone_amm(&self) -> Box<dyn Amm + Send + Sync> {
        Box::new(self.clone())
    }
}