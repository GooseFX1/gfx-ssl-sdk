use crate::states::SwapCache;
use crate::svec::StackVec;
use crate::utils::PDAIdentifier;
use anchor_lang::prelude::*;

impl PDAIdentifier for Pair {
    const IDENT: &'static [u8] = b"GFX-SSL-Pair";

    #[inline(always)]
    fn program_id() -> &'static Pubkey {
        &crate::ID
    }
}

#[derive(Copy, Clone, Default)]
#[cfg_attr(feature = "no-entrypoint", derive(Debug))]
#[repr(C, align(8))]
pub struct Oracle {
    pub path: StackVec<(Pubkey, bool), 4>,
    pub padding: [u64; 8],
}

#[allow(non_snake_case)]
#[account(zero_copy)]
#[derive(Default)]
#[cfg_attr(feature = "no-entrypoint", derive(Debug))]
pub struct Pair {
    pub controller: Pubkey, // for indexing purpose
    pub mints: (Pubkey, Pubkey),
    pub bump: u8,
    pub oracles: StackVec<Oracle, 5>,
    // configs
    pub A: u8,              // parameter A for poorman's curve
    pub fee_rate: (u8, u8), // in BP
    pub max_delay: u64,
    pub confidence: u64,
    pub balancer: Pubkey,               // the pubkey of the balancer
    pub excessive_confiscate_rate: u16, // the percentage to confiscate if a trade gets better price than the oracle price

    pub fee_collector: Pubkey, // the pubkey for the platform fee collector
    pub platform_fee_rate: (u16, u16),

    pub rebalance_rebates: (u8, u8),
    pub surpluses: (u64, u64),

    pub volumes: (u128, u128),
    // inter-ix communication
    pub swap_cache: SwapCache,
    pub enable_rebalance_swap: bool,
    pub _pad: [u8; 7],
    pub _pad2: [u64; 18],
}

const _: [u8; 1528] = [0; std::mem::size_of::<Pair>()];
