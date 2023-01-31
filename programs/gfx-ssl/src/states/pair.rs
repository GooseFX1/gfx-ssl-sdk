use crate::svec4::StackVec4;
use crate::utils::PDAIdentifier;
use anchor_lang::prelude::*;
use crate::svec5::StackVec5;

impl PDAIdentifier for Pair {
    const IDENT: &'static [u8] = b"GFX-SSL-Pair";

    #[inline(always)]
    fn program_id() -> &'static Pubkey {
        &crate::ID
    }
}

#[account]
#[derive(Copy, Default)]
#[cfg_attr(feature = "no-entrypoint", derive(Debug))]
#[repr(C, align(8))]
pub struct Oracle {
    pub path: StackVec4<(Pubkey, bool)>,
    pub padding: [u64; 8],
}

#[allow(non_snake_case)]
#[account]
#[cfg_attr(feature = "no-entrypoint", derive(Debug))]
#[derive(Default)]
pub struct Pair {
    pub controller: Pubkey, // for indexing purpose
    pub mints: (Pubkey, Pubkey),
    pub bump: u8,
    pub _pad0: [u8; 7],
    pub oracles: StackVec5<Oracle>,
    // configs
    pub A: u8,              // parameter A for poorman's curve
    pub fee_rate: (u8, u8), // in BP
    pub _pad1: [u8; 5],
    pub max_delay: u64,
    pub confidence: u64,
    pub _unused4: [u8; 32],
    pub excessive_confiscate_rate: u16, // the percentage to confiscate if a trade gets better price than the oracle price

    pub fee_collector: Pubkey, // the pubkey for the platform fee collector
    pub platform_fee_rate: (u16, u16),

    pub _unused3: [u8; 2],
    pub surpluses: (u64, u64), // surpluses.0 is the surplus of mints.0, which is owned by the ssl of mints.1
    pub volumes: (u128, u128),

    pub _unused0: [u64; 10],
    pub enable_rebalance_swap: bool,
    pub _pad3: [u8; 7],
    pub _pad4: [u64; 18],
}

const _: [u8; 1528] = [0; std::mem::size_of::<Pair>()];
