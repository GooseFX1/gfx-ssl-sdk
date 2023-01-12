mod contexts;
mod errors;
mod states;
mod svec;
mod utils;

use anchor_lang::prelude::*;
use contexts::*;
pub use states::*;
pub use utils::{PDAIdentifier, skey};

declare_id!("7WduLbRfYhTJktjLw5FDEyrqoEv61aTTCuGAetgLjzN5");

#[program]
pub mod contract {
    use super::*;

    #[allow(unused_variables)]
    pub fn create_liquidity_account(ctx: Context<CreateLiquidityAccount>) -> Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    pub fn withdraw(ctx: Context<Withdraw>, withdraw_percent: u64) -> Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    pub fn swap(ctx: Context<Swap>, amount_in: u64, min_out: u64) -> Result<()> {
        Ok(())
    }
}
