use crate::states::{LiquidityAccount, PTMint, SSL};
use crate::utils::PDAIdentifier;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use gfx_controller::Controller;

#[derive(Accounts)]
pub struct BurnPT<'info> {
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
        seeds = [PTMint::IDENT, controller.key().as_ref(), ssl.load()?.mint.as_ref()],
        bump = ssl.load()?.pt_bump,
    )]
    pub pt_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = pt_mint,
        associated_token::authority = user_wallet,
    )]
    pub user_pt_ata: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub user_wallet: Signer<'info>,

    pub token_program: Program<'info, Token>,
}
