use crate::states::{LiquidityAccount, PTMint, SSL};
use crate::utils::PDAIdentifier;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use gfx_controller::Controller;

#[derive(Accounts)]
pub struct MintPT<'info> {
    pub controller: Account<'info, Controller>,

    #[account(
        mut,
        seeds = [SSL::IDENT, controller.key().as_ref(), ssl.mint.as_ref()],
        bump = ssl.bump,
    )]
    pub ssl: Account<'info, SSL>,

    #[account(
        mut,
        associated_token::mint = ssl.mint,
        associated_token::authority = ssl,
    )]
    pub rt_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [LiquidityAccount::IDENT, controller.key().as_ref(), ssl.mint.as_ref(), user_wallet.key().as_ref()],
        bump = liquidity_account.bump,
    )]
    pub liquidity_account: Account<'info, LiquidityAccount>,

    #[account(
        mut,
        seeds = [PTMint::IDENT, controller.key().as_ref(), ssl.mint.as_ref()],
        bump = ssl.pt_bump,
    )]
    pub pt_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = pt_mint,
        associated_token::authority = user_wallet,
    )]
    pub user_pt_ata: Box<Account<'info, TokenAccount>>,

    pub user_wallet: Signer<'info>,

    pub token_program: Program<'info, Token>,
}
