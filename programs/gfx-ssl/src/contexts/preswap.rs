use crate::utils::PDAIdentifier;
use crate::{
    states::{Pair, SSL},
    utils::skey,
};
use anchor_lang::prelude::*;

use gfx_controller::Controller;
use solana_program::sysvar::instructions;

#[derive(Accounts)]
pub struct PreSwap<'info> {
    pub controller: AccountLoader<'info, Controller>,

    #[account(
        mut,
        seeds = [
            Pair::IDENT,
            controller.key().as_ref(),
            skey::<_, true>(&ssl_in.load()?.mint, &ssl_out.load()?.mint).as_ref(),
            skey::<_, false>(&ssl_in.load()?.mint, &ssl_out.load()?.mint).as_ref()
        ],
        bump = pair.load()?.bump,
    )]
    pub pair: AccountLoader<'info, Pair>,

    #[account(
        seeds = [SSL::IDENT, controller.key().as_ref(), ssl_in.load()?.mint.as_ref()],
        bump = ssl_in.load()?.bump,
    )]
    pub ssl_in: AccountLoader<'info, SSL>,

    #[account(
        seeds = [SSL::IDENT, controller.key().as_ref(), ssl_out.load()?.mint.as_ref()],
        bump = ssl_out.load()?.bump,
    )]
    pub ssl_out: AccountLoader<'info, SSL>,

    pub user_wallet: Signer<'info>,

    /// CHECK: make sure to check this address is correct
    #[account(address = instructions::ID)]
    pub instructions: AccountInfo<'info>, // The instruction sysvar
}
