mod contexts;
mod errors;
mod states;
mod svec;
mod utils;

use anchor_lang::prelude::*;
use contexts::*;

declare_id!("7WduLbRfYhTJktjLw5FDEyrqoEv61aTTCuGAetgLjzN5");

#[program]
pub mod contract {
    use super::*;

    #[allow(unused_variables)]
    pub fn rebalance_swap(ctx: Context<RebalanceSwap>, amount_in: u64, min_out: u64) -> Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    pub fn pre_swap(ctx: Context<PreSwap>) -> Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    pub fn swap(ctx: Context<Swap>) -> Result<()> {
        Ok(())
    }
}

// preswap
#[derive(Accounts)]
pub struct PreSwap<'info> {
    pub controller: AccountInfo<'info>,

    #[account(mut)]
    pub pair: AccountInfo<'info>,

    pub ssl_in: AccountInfo<'info>,

    pub ssl_out: AccountInfo<'info>,

    pub user_wallet: Signer<'info>,

    pub instructions: AccountInfo<'info>, // solana_program::sysvar::instructions::ID
}

#[derive(Accounts)]
pub struct Swap<'info> {
    pub controller: AccountInfo<'info>,

    #[account(mut)]
    pub pair: AccountInfo<'info>,

    #[account(mut)]
    pub ssl_in: AccountInfo<'info>,

    #[account(mut)]
    pub ssl_out: AccountInfo<'info>,

    #[account(mut)]
    pub vault_in: AccountInfo<'info>,

    #[account(mut)]
    pub vault_out: AccountInfo<'info>,

    #[account(mut)]
    pub user_in_ata: AccountInfo<'info>,

    #[account(mut)]
    pub user_out_ata: AccountInfo<'info>,

    #[account(mut)]
    pub fee_collector_ata: AccountInfo<'info>,

    #[account(mut)]
    pub user_wallet: Signer<'info>,
    pub fee_collector: SystemAccount<'info>,

    pub instructions: AccountInfo<'info>, // solana_program::sysvar::instructions::ID
    pub token_program: AccountInfo<'info>,
}
