use std::io::Write;
use crate::utils::PDAIdentifier;
use anchor_lang::prelude::*;
use std::mem;
use anchor_lang::Discriminator;

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
    pub suspended: u8,
    pub cranker: Pubkey, // the pubkey for the cranker
    pub _pad0: [u8; 4],

    // pool states
    pub weight: u64, // the connector token amount. This does not need to be an SPL token because this is internal
    pub liability: u64, // liability to the user
    pub swapped_liability: u64, // liability that in the form of other tokens due to swapped away, updated by the cranker
    pub total_share: u64,

    pub _pad1: [u64; 32],
}

const _: [u8; 392] = [0; std::mem::size_of::<SSL>()];

impl Default for SSL {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

impl AccountSerialize for SSL {
    fn try_serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        let mut disc = Self::discriminator().to_vec();
        disc.append(&mut bytemuck::bytes_of(self).to_vec());
        writer.write_all(&disc)?;
        Ok(())
    }
}
