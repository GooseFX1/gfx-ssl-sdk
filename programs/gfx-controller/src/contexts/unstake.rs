use crate::states::{Controller, StakingAccount};
use crate::utils::PDAIdentifier;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

// Unstake
#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub controller: AccountLoader<'info, Controller>,

    #[account(
        mut,
        seeds = [StakingAccount::IDENT, controller.key().as_ref(), user_wallet.key().as_ref()],
        bump = staking_account.load()?.bump,
    )]
    pub staking_account: AccountLoader<'info, StakingAccount>,

    pub mint: Account<'info, Mint>, // check is done on ssl

    #[account(
        mut,
        associated_token::mint = controller.load()?.mint,
        associated_token::authority = controller,
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = controller.load()?.mint,
        associated_token::authority = user_wallet,
    )]
    pub user_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = controller.load()?.mint,
        associated_token::authority = controller.load()?.admin,
    )]
    pub fee_collector_ata: Account<'info, TokenAccount>,

    pub user_wallet: Signer<'info>,

    pub token_program: Program<'info, Token>,
}
