use crate::states::{LiquidityAccount, SSL};
use crate::utils::PDAIdentifier;
use anchor_lang::prelude::*;
use gfx_controller::Controller;
use std::mem::size_of;

#[derive(Accounts)]
pub struct CreateLiquidityAccount<'info> {
    pub controller: Account<'info, Controller>,

    #[account(
        mut,
        seeds = [SSL::IDENT, controller.key().as_ref(), ssl.mint.as_ref()],
        bump = ssl.bump,
    )]
    pub ssl: Account<'info, SSL>,

    #[account(
        init,
        seeds = [LiquidityAccount::IDENT, controller.key().as_ref(), ssl.mint.as_ref(), user_wallet.key().as_ref()],
        bump,
        space = size_of::<LiquidityAccount>() + 8,
        payer = user_wallet
    )]
    pub liquidity_account: Account<'info, LiquidityAccount>,

    #[account(mut)]
    pub user_wallet: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
