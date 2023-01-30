use crate::states::{LiquidityAccount, SSL};
use crate::utils::PDAIdentifier;
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use gfx_controller::Controller;

// Deposit rt
#[derive(Accounts)]
pub struct Deposit<'info> {
    pub controller: Account<'info, Controller>,

    #[account(
        mut,
        seeds = [SSL::IDENT, controller.key().as_ref(), ssl.mint.as_ref()],
        bump = ssl.bump,
    )]
    pub ssl: Account<'info, SSL>,

    #[account(
        mut,
        seeds = [LiquidityAccount::IDENT, controller.key().as_ref(), ssl.mint.as_ref(), user_wallet.key().as_ref()],
        bump = liquidity_account.bump,
    )]
    pub liquidity_account: Account<'info, LiquidityAccount>,

    #[account(
        mut,
        associated_token::mint = ssl.mint,
        associated_token::authority = ssl,
    )]
    pub rt_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = ssl.mint,
        associated_token::authority = user_wallet,
    )]
    pub user_rt_ata: Box<Account<'info, TokenAccount>>,

    pub user_wallet: Signer<'info>,

    pub token_program: Program<'info, Token>,
}
