use crate::utils::PDAIdentifier;
use crate::{
    errors::ErrorCode::*,
    states::{Pair, SSL},
    utils::sorted,
};
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use gfx_controller::Controller;

#[derive(Accounts)]
pub struct Swap<'info> {
    pub controller: AccountLoader<'info, Controller>,

    #[account(
        mut,
        seeds = [
            Pair::IDENT,
            controller.key().as_ref(),
            sorted::<_, 0>(&ssl_in.load()?.mint, &ssl_out.load()?.mint).as_ref(),
            sorted::<_, 1>(&ssl_in.load()?.mint, &ssl_out.load()?.mint).as_ref()
        ],
        bump = pair.load()?.bump,
        has_one = fee_collector @ FeeCollectorIncorrect,
    )]
    pub pair: AccountLoader<'info, Pair>,

    #[account(
        mut,
        seeds = [SSL::IDENT, controller.key().as_ref(), ssl_in.load()?.mint.as_ref()],
        bump = ssl_in.load()?.bump,
    )]
    pub ssl_in: AccountLoader<'info, SSL>,

    #[account(
        mut,
        seeds = [SSL::IDENT, controller.key().as_ref(), ssl_out.load()?.mint.as_ref()],
        bump = ssl_out.load()?.bump,
    )]
    pub ssl_out: AccountLoader<'info, SSL>,

    #[account(
        mut,
        associated_token::mint = ssl_in.load()?.mint,
        associated_token::authority = ssl_in,
    )]
    pub liability_vault_in: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = ssl_out.load()?.mint,
        associated_token::authority = ssl_in,
    )]
    pub swapped_liability_vault_in: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = ssl_out.load()?.mint,
        associated_token::authority = ssl_out,
    )]
    pub liability_vault_out: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = ssl_in.load()?.mint,
        associated_token::authority = ssl_out,
    )]
    pub swapped_liability_vault_out: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = ssl_in.load()?.mint,
        associated_token::authority = user_wallet,
    )]
    pub user_in_ata: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub user_out_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = ssl_in.load()?.mint,
        associated_token::authority = fee_collector,
    )]
    pub fee_collector_ata: Box<Account<'info, TokenAccount>>,

    pub user_wallet: Signer<'info>,
    pub fee_collector: SystemAccount<'info>,

    pub token_program: Program<'info, Token>,
}
