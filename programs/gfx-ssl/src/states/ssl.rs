use crate::utils::PDAIdentifier;
use anchor_lang::prelude::*;
use std::mem;

impl PDAIdentifier for SSL {
    const IDENT: &'static [u8] = b"GFX-SSL";

    #[inline(always)]
    fn program_id() -> &'static Pubkey {
        &crate::ID
    }
}

#[account(zero_copy)]
#[cfg_attr(feature = "no-entrypoint", derive(Debug))]
pub struct SSL {
    pub controller: Pubkey,
    pub mint: Pubkey,
    pub decimals: u8, // a copy from the mint
    pub bump: u8,
    pub pt_bump: u8,

    // configurable settings
    pub suspended: bool,
    pub cranker: Pubkey, // the pubkey for the cranker

    // pool states
    pub weight: u64, // the connector token amount. This does not need to be an SPL token because this is internal
    pub liability: u64, // liability to the user
    pub swapped_liability: u64, // liability that in the form of other tokens due to swapped away, updated by the cranker
    pub total_share: u64,

    pub _pad: [u64; 32],
}

const _: [u8; 392] = [0; std::mem::size_of::<SSL>()];

impl Default for SSL {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}
