use crate::states::{LiquidityAccount, SSL};
use crate::utils::PDAIdentifier;
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use gfx_controller::Controller;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub controller: AccountLoader<'info, Controller>,

    #[account(
        mut,
        seeds = [SSL::IDENT, controller.key().as_ref(), ssl.load()?.mint.as_ref()],
        bump = ssl.load()?.bump,
    )]
    pub ssl: AccountLoader<'info, SSL>,

    #[account(
        mut,
        seeds = [LiquidityAccount::IDENT, controller.key().as_ref(), ssl.load()?.mint.as_ref(), user_wallet.key().as_ref()],
        bump = liquidity_account.load()?.bump,
    )]
    pub liquidity_account: AccountLoader<'info, LiquidityAccount>,

    #[account(
        mut,
        associated_token::mint = ssl.load()?.mint,
        associated_token::authority = ssl,
    )]
    pub rt_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = ssl.load()?.mint,
        associated_token::authority = user_wallet,
    )]
    pub user_rt_ata: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub user_wallet: Signer<'info>,

    pub token_program: Program<'info, Token>,
}
