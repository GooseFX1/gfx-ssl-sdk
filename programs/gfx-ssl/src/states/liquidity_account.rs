use crate::utils::PDAIdentifier;
use anchor_lang::prelude::*;

impl PDAIdentifier for LiquidityAccount {
    const IDENT: &'static [u8] = b"GFX-LIQUIDITYACCOUNT";

    #[inline(always)]
    fn program_id() -> &'static Pubkey {
        &crate::ID
    }
}

#[account(zero_copy)]
#[derive(Default)]
#[cfg_attr(feature = "no-entrypoint", derive(Debug))]
#[cfg_attr(feature = "type-layout", derive(TypeLayout))]
pub struct LiquidityAccount {
    pub mint: Pubkey,
    pub bump: u8,
    pub share: u64,
    pub pt_minted: u64,
    pub amount_deposited: u64,

    pub _pad: [u64; 31],
}

const _: [u8; 312] = [0; std::mem::size_of::<LiquidityAccount>()];
