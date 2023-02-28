use crate::{
    error::GfxSdkError::*,
    ssl::{
        instructions::{swap_account_metas, SSLInstructionContext},
        FEE_COLLECTOR,
    },
};
use anchor_lang::AccountDeserialize;
use anchor_spl::{associated_token::get_associated_token_address, token::TokenAccount};
use anyhow::{anyhow, Error};
use fehler::{throw, throws};
use gfx_ssl_interface::{sorted, PDAIdentifier, Pair, SSL};
use jupiter::jupiter_override::{Swap, SwapLeg};
use jupiter_core::amm::{Amm, KeyedAccount, Quote, QuoteParams, SwapLegAndAccountMetas, SwapParams};
use lazy_static::lazy_static;
use pyth_sdk_solana::state::{load_price_account, PriceAccount};
use rust_decimal::Decimal;
use solana_program::pubkey::Pubkey;
use solana_sdk::pubkey;
use std::{
    collections::{HashMap, HashSet},
    ffi::CStr,
    fmt::Debug,
    mem,
};
use solana_program::clock::Clock;
use solana_program::sysvar::SysvarId;

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
            // (
            //     pubkey!("7dHbWXmci3dT8UFYWYZweBLXgycu7Y3iL6trKn1Y7ARj"),
            //     "STSOL",
            // ),
            // (
            //     pubkey!("9n4nbM75f5Ui33ZbPYXn59EwSgE8CGsHtAeTH5YFeJ9E"),
            //     "BTC",
            // ),
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
            ("USDT", pubkey!("3vxLXJqLqF3JG5TCbYycbKWRBbCJQLxQmBGCkyqEEefL")),
            ("ORCA", pubkey!("4ivThkX8uRxBpHsdWSqyXYihzKF3zpRGAUCqyuagnLoV")),
            ("USDC", pubkey!("Gnt27xtC473ZT2Mw5u8wZ68Z3gULkSTb5DuxJy7eJotD")),
            // ("STSOL", pubkey!("Bt1hEbY62aMriY1SyQqbeZbm8VmSbQVGBFzSzMuVNWzN")),
            // ("BTC", pubkey!("GVXRSBjFk6e6J3NbVPXohDJetcTjaeeuykUpbQF8UoMU")),
        ]
        .into_iter()
        .for_each(|(label, pubkey)| {
            paths.insert(label, pubkey);
        });
        paths
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
        slot: u64,
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
#[derive(Debug, Clone, Default)]
pub struct GfxAmm {
    // Related keys
    ssl_a_mint: Pubkey,
    ssl_a_pubkey: Pubkey,
    ssl_b_mint: Pubkey,
    ssl_b_pubkey: Pubkey,
    pub pair_pubkey: Pubkey,
    ssl_a_vault_a: Pubkey,
    ssl_a_vault_b: Pubkey,
    ssl_b_vault_a: Pubkey,
    ssl_b_vault_b: Pubkey,
    oracle_addresses: HashSet<Pubkey>, // Fetched from a lookup table during construction.

    // Accounts' data
    ssl_a_data: Option<[u8; mem::size_of::<SSL>() + DISCRIMINANT]>,
    ssl_b_data: Option<[u8; mem::size_of::<SSL>() + DISCRIMINANT]>,
    pair_data: Option<[u8; mem::size_of::<Pair>() + DISCRIMINANT]>,
    pair: Option<Pair>, // deserialized for the fee_rate
    ssl_a_vault_a_balance: Option<u64>,
    ssl_a_vault_b_balance: Option<u64>,
    ssl_b_vault_a_balance: Option<u64>,
    ssl_b_vault_b_balance: Option<u64>,
    // Indexed by Pubkey of the [PriceAccount].
    oracles: HashMap<Pubkey, PriceAccount>,
    slot: u64,
}

impl GfxAmm {
    #[throws(Error)]
    pub fn from_keyed_account(act: KeyedAccount) -> Self {
        let data = act.account.data;
        let data: [u8; mem::size_of::<Pair>() + DISCRIMINANT] = data.clone().try_into().map_err(|_| {
            InvalidAccountSize(
                act.key,
                mem::size_of::<Pair>() + DISCRIMINANT,
                data.len(),
            )
        })?;
        let pair_data = Some(data);
        let pair: Pair = Pair::try_deserialize(&mut data.as_slice())?;
        let (ssl_a_mint, ssl_b_mint) = pair.mints;
        let mut oracle_addresses = HashSet::new();
        let oracles = pair.oracles;
        for oracle in oracles.iter() {
            for (key, _) in oracle.path.iter() {
                if *key != Pubkey::default() {
                    oracle_addresses.insert(*key);
                }
            }
        }
        let pair = Some(pair);

        // Calculate PDAs of GFX accounts
        let ssl_a_pubkey = SSL::get_address(&[CONTROLLER.as_ref(), ssl_a_mint.as_ref()]);
        let ssl_b_pubkey = SSL::get_address(&[CONTROLLER.as_ref(), ssl_b_mint.as_ref()]);
        let pair_pubkey = Pair::get_address(&[
            CONTROLLER.as_ref(),
            ssl_a_mint.as_ref(),
            ssl_b_mint.as_ref(),
        ]);
        let ssl_a_vault_a = get_associated_token_address(&ssl_a_pubkey, &ssl_a_mint);
        let ssl_a_vault_b = get_associated_token_address(&ssl_a_pubkey, &ssl_b_mint);
        let ssl_b_vault_a = get_associated_token_address(&ssl_b_pubkey, &ssl_a_mint);
        let ssl_b_vault_b = get_associated_token_address(&ssl_b_pubkey, &ssl_b_mint);

        Self {
            ssl_a_mint,
            ssl_a_pubkey,
            ssl_b_mint,
            ssl_b_pubkey,
            pair_pubkey,
            ssl_a_vault_a,
            ssl_a_vault_b,
            ssl_b_vault_a,
            ssl_b_vault_b,
            oracle_addresses,
            pair,
            pair_data,
            ..Default::default()
        }
    }

    #[throws(Error)]
    pub fn new(mint_1: Pubkey, mint_2: Pubkey) -> Self {
        // Arrange them in order first
        let ssl_a_mint = sorted::<_, 0>(&mint_1, &mint_2);
        let ssl_b_mint = sorted::<_, 1>(&mint_1, &mint_2);

        // Calculate PDAs of GFX accounts
        let ssl_a_pubkey = SSL::get_address(&[CONTROLLER.as_ref(), ssl_a_mint.as_ref()]);
        let ssl_b_pubkey = SSL::get_address(&[CONTROLLER.as_ref(), ssl_b_mint.as_ref()]);
        let pair_pubkey = Pair::get_address(&[
            CONTROLLER.as_ref(),
            ssl_a_mint.as_ref(),
            ssl_b_mint.as_ref(),
        ]);
        let ssl_a_vault_a = get_associated_token_address(&ssl_a_pubkey, &ssl_a_mint);
        let ssl_a_vault_b = get_associated_token_address(&ssl_a_pubkey, &ssl_b_mint);
        let ssl_b_vault_a = get_associated_token_address(&ssl_b_pubkey, &ssl_a_mint);
        let ssl_b_vault_b = get_associated_token_address(&ssl_b_pubkey, &ssl_b_mint);

        Self {
            ssl_a_mint,
            ssl_a_pubkey,
            ssl_b_mint,
            ssl_b_pubkey,
            pair_pubkey,
            ssl_a_vault_a,
            ssl_a_vault_b,
            ssl_b_vault_a,
            ssl_b_vault_b,
            ..Default::default()
        }
    }
}

impl Amm for GfxAmm {
    /// Human-readable name for the Amm pair.
    fn label(&self) -> String {
        "GooseFX".to_string()
    }

    /// Get a pubkey to represent the Amm as a whole.
    fn key(&self) -> Pubkey {
        self.pair_pubkey
    }

    /// Returns mints offered by this Amm for swap.
    fn get_reserve_mints(&self) -> Vec<Pubkey> {
        vec![self.ssl_a_mint, self.ssl_b_mint]
    }

    /// Returns pubkeys of all the accounts required
    /// for providing accurate quotes and swap instructions.
    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        let mut accounts = vec![
            self.ssl_a_pubkey,
            self.ssl_b_pubkey,
            self.pair_pubkey,
            self.ssl_a_vault_a,
            self.ssl_a_vault_b,
            self.ssl_b_vault_a,
            self.ssl_b_vault_b,
            Clock::id(),
        ];
        accounts.extend(&self.oracle_addresses);
        accounts
    }

    /// Update the account state contained in self.
    #[throws(Error)]
    fn update(&mut self, accounts_map: &HashMap<Pubkey, Vec<u8>>) {
        let update_token_account = |amount: &mut Option<u64>, data: &mut &[u8]| {
            let token_account = TokenAccount::try_deserialize(data)?;
            *amount = Some(token_account.amount);
            Ok::<_, Error>(())
        };
        for pubkey in self.get_accounts_to_update() {
            let data = accounts_map.get(&pubkey)
                .ok_or(AccountNotFound(pubkey))?;
            if pubkey == self.ssl_a_pubkey {
                let data = data.clone().try_into().map_err(|_| {
                    InvalidAccountSize(
                        self.ssl_a_pubkey,
                        mem::size_of::<SSL>() + DISCRIMINANT,
                        data.len(),
                    )
                })?;
                self.ssl_a_data = Some(data);
            } else if pubkey == self.ssl_b_pubkey {
                let data = data.clone().try_into().map_err(|_| {
                    InvalidAccountSize(
                        self.ssl_b_pubkey,
                        mem::size_of::<SSL>() + DISCRIMINANT,
                        data.len(),
                    )
                })?;
                self.ssl_b_data = Some(data);
            } else if pubkey == self.pair_pubkey {
                let data = data.clone().try_into().map_err(|_| {
                    InvalidAccountSize(
                        self.pair_pubkey,
                        mem::size_of::<Pair>() + DISCRIMINANT,
                        data.len(),
                    )
                })?;
                self.pair_data = Some(data);
                let pair: Pair = Pair::try_deserialize(&mut data.as_slice())?;
                let oracles = pair.oracles;
                for oracle in oracles.iter() {
                    for (key, _) in oracle.path.iter() {
                        if *key != Pubkey::default() {
                            self.oracle_addresses.insert(*key);
                        }
                    }
                }
                self.pair = Some(pair);
            } else if pubkey == self.ssl_a_vault_a {
                update_token_account(&mut self.ssl_a_vault_a_balance, &mut data.as_slice())?;
            } else if pubkey == self.ssl_a_vault_b {
                update_token_account(&mut self.ssl_a_vault_b_balance, &mut data.as_slice())?;
            } else if pubkey == self.ssl_b_vault_a {
                update_token_account(&mut self.ssl_b_vault_a_balance, &mut data.as_slice())?;
            } else if pubkey == self.ssl_b_vault_b {
                update_token_account(&mut self.ssl_b_vault_b_balance, &mut data.as_slice())?;
            } else if pubkey == Clock::id() {
                let clock: Clock = bincode::deserialize(&mut data.as_slice())?;
                self.slot = clock.slot;
            } else {
                // Assume it's an oracle
                let price_account = load_price_account(&mut data.as_slice())?;
                self.oracles.insert(pubkey, *price_account);
            }
        }
    }

    /// Get a GooseFX SSL swap quote
    #[throws(Error)]
    fn quote(&self, quote_params: &QuoteParams) -> Quote {
        if self.ssl_a_data.is_none()
            || self.ssl_b_data.is_none()
            || self.pair_data.is_none()
            || self.ssl_a_vault_a_balance.is_none()
            || self.ssl_a_vault_b_balance.is_none()
            || self.ssl_b_vault_a_balance.is_none()
            || self.ssl_b_vault_b_balance.is_none()
        {
            throw!(RequiredAccountNoUpdate);
        }
        if self.oracles.is_empty() {
            throw!(OraclesNeedUpdate);
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
            throw!(UnexpectedMints);
        }

        let (
            ssl_in,
            ssl_out,
            liability_in,
            swapped_liability_in,
            swapped_liability_out,
            liability_out,
        ) = if !is_reversed {
            (
                self.ssl_a_data.unwrap(),
                self.ssl_b_data.unwrap(),
                self.ssl_a_vault_a_balance.unwrap(),
                self.ssl_a_vault_b_balance.unwrap(),
                self.ssl_b_vault_a_balance.unwrap(),
                self.ssl_b_vault_b_balance.unwrap(),
            )
        } else {
            (
                self.ssl_b_data.unwrap(),
                self.ssl_a_data.unwrap(),
                self.ssl_b_vault_b_balance.unwrap(),
                self.ssl_b_vault_a_balance.unwrap(),
                self.ssl_a_vault_b_balance.unwrap(),
                self.ssl_a_vault_a_balance.unwrap(),
            )
        };

        let oracles: Vec<OracleEntry> = self
            .oracles
            .iter()
            .map(|(pubkey, act)| OracleEntry(pubkey.as_ref().try_into().unwrap(), *act))
            .collect();

        match unsafe {
            quote(
                &ssl_in,
                &ssl_out,
                &self.pair_data.unwrap(),
                liability_in,
                liability_out,
                swapped_liability_in,
                swapped_liability_out,
                oracles.as_ptr(),
                self.oracles.len(),
                self.slot,
                quote_params.in_amount,
            )
        } {
            QuoteResult::Ok(swap_result) => {
                let fee_pct = if !is_reversed {
                    self.pair.as_ref().unwrap().fee_rate.0
                } else {
                    self.pair.as_ref().unwrap().fee_rate.1
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
                quote
            }
            QuoteResult::Error(err) => unsafe {
                let c_str = CStr::from_ptr(err);
                let rust_str = c_str.to_str().expect("bad string encoding");
                throw!(anyhow!("{}", rust_str))
            },
        }
    }

    /// Get account metas for a GFX swap instruction,
    /// and marker denoting a [SwapLeg::Swap], and a [Swap::GooseFX].
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
