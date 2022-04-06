use crate::utils::PDAIdentifier;
use anchor_lang::prelude::*;

pub struct PTMint;

impl PDAIdentifier for PTMint {
    const IDENT: &'static [u8] = b"GFX-SSL-PTMINT";

    #[inline(always)]
    fn program_id() -> &'static Pubkey {
        &crate::ID
    }
}
