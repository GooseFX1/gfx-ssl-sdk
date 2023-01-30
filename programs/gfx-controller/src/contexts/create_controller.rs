use crate::states::Controller;
use crate::PDAIdentifier;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use std::mem::size_of;

#[derive(Accounts)]
#[instruction(seed: [u8; 32])]
pub struct CreateController<'info> {
    #[account(
        init,
        seeds = [Controller::IDENT, &seed],
        bump,
        space = size_of::<Controller>() + 8,
        payer = admin
    )]
    pub controller: Account<'info, Controller>,

    pub mint: Account<'info, Mint>,

    #[account(
        init,
        associated_token::mint = mint,
        associated_token::authority = controller,
        payer = admin
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = admin,
    )]
    pub admin_ata: Account<'info, TokenAccount>, // for boostraping the controller

    #[account(mut)]
    pub admin: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}