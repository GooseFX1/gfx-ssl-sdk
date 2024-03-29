use crate::utils::PDAIdentifier;
use anchor_lang::prelude::*;

impl PDAIdentifier for StakingAccount {
    const IDENT: &'static [u8] = b"GFX-STAKINGACCOUNT";

    #[inline(always)]
    fn program_id() -> &'static Pubkey {
        &crate::ID
    }
}

#[account(zero_copy)]
#[derive(Default)]
#[cfg_attr(feature = "no-entrypoint", derive(Debug))]
pub struct StakingAccount {
    pub controller: Pubkey, // for indexing purpose
    pub bump: u8,
    pub _pad0: [u8; 7],
    pub share: u64,
    pub amount_staked: u64,

    pub _pad1: [u64; 32],
}

const _: [u8; 312] = [0; std::mem::size_of::<StakingAccount>()];
