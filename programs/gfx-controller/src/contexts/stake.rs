use crate::states::{Controller, StakingAccount};
use crate::utils::PDAIdentifier;
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

// Stake GOFX
#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub controller: Account<'info, Controller>,

    #[account(
        mut,
        seeds = [StakingAccount::IDENT, controller.key().as_ref(), user_wallet.key().as_ref()],
        bump = staking_account.bump,
    )]
    pub staking_account: Account<'info, StakingAccount>,

    #[account(
        mut,
        associated_token::mint = controller.mint,
        associated_token::authority = controller,
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = controller.mint,
        associated_token::authority = user_wallet,
    )]
    pub user_ata: Account<'info, TokenAccount>,
    pub user_wallet: Signer<'info>,

    pub token_program: Program<'info, Token>,
}
