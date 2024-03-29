use std::io::Write;
use crate::utils::PDAIdentifier;
use anchor_lang::prelude::*;
use anchor_lang::Discriminator;

impl PDAIdentifier for Controller {
    const IDENT: &'static [u8] = b"GFX-CONTROLLER";

    #[inline(always)]
    fn program_id() -> &'static Pubkey {
        &crate::ID
    }
}

#[account(zero_copy)]
#[derive(Default)]
#[cfg_attr(feature = "no-entrypoint", derive(Debug))]
pub struct Controller {
    pub seed: [u8; 32],
    pub bump: u8,

    /* --------------------------------- configs -------------------------------- */
    pub admin: Pubkey,
    pub suspended: u8, // bool
    pub decimals: u8,  // Cache the decimal of the protocol token here.
    pub mint: Pubkey,  // The mint of the protocol token (e.g. GOFX)
    pub _pad0: [u8; 5],
    pub daily_reward: u64, // How many protocol token to distribute every day

    /* ---------------------------- controller states --------------------------- */
    pub total_staking_share: u64,
    // Book keeping how many of GOFX in the vault belongs to the staking fund (user deposited + rewards)
    // This number must less or equal than the amount of GOFX in the vault.
    pub staking_balance: u64,
    pub last_distribution_time: i64, // When is the last time we distribute reward

    pub withdraw_fee: u16, // in BP
    pub _pad1: [u8; 6],

    pub _pad2: [u64; 31],
}

const _: [u8; 392] = [0; std::mem::size_of::<Controller>()];

impl AccountSerialize for Controller {
    fn try_serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        let mut disc = Self::discriminator().to_vec();
        disc.append(&mut bytemuck::bytes_of(self).to_vec());
        writer.write_all(&disc)?;
        Ok(())
    }
}