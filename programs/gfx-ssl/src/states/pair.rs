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
    _pad0: [u8; 7],
    pub oracles: StackVec<Oracle, 5>,
    // configs
    pub A: u8,              // parameter A for poorman's curve
    pub fee_rate: (u8, u8), // in BP
    _pad1: [u8; 5],
    pub max_delay: u64,
    pub confidence: u64,
    pub balancer: Pubkey,               // the pubkey of the balancer
    pub excessive_confiscate_rate: u16, // the percentage to confiscate if a trade gets better price than the oracle price

    pub fee_collector: Pubkey, // the pubkey for the platform fee collector
    pub platform_fee_rate: (u16, u16),

    pub _unused3: [u8; 2],
    pub surpluses: (u64, u64), // surpluses.0 is the surplus of mints.0, which is owned by the ssl of mints.1
    pub volumes: (u128, u128),

    _unused0: [u64; 10],
    pub enable_rebalance_swap: bool,
    _pad3: [u8; 7],
    _pad4: [u64; 18],
}

const _: [u8; 1528] = [0; std::mem::size_of::<Pair>()];
