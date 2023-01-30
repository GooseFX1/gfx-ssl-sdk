use crate::states::{Controller, StakingAccount};
use crate::utils::PDAIdentifier;
use anchor_lang::prelude::*;
use std::mem::size_of;

#[derive(Accounts)]
pub struct CreateStakingAccount<'info> {
    pub controller: Account<'info, Controller>,

    #[account(
        init,
        seeds = [StakingAccount::IDENT, controller.key().as_ref(), user_wallet.key().as_ref()],
        bump,
        space = size_of::<StakingAccount>() + 8,
        payer = user_wallet
    )]
    pub staking_account: Account<'info, StakingAccount>,

    #[account(mut)]
    pub user_wallet: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
