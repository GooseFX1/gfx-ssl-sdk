pub mod contexts;
pub mod errors;
pub mod states;
mod utils;

pub use self::{contexts::*, errors::ErrorCode, states::*, utils::BP_DECIMAL};
use anchor_lang::prelude::*;
pub use program_id::*;

mod program_id {
    use anchor_lang::prelude::*;
    declare_id!("8KJx48PYGHVC9fxzRRtYp4x4CM2HyYCm2EjVuAP4vvrx");
}

#[program]
pub mod contract {
    use super::*;

    pub fn create_staking_account(_ctx: Context<CreateStakingAccount>) -> Result<()> {
        Ok(())
    }

    pub fn stake(_ctx: Context<Stake>, _amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn unstake(_ctx: Context<Unstake>, _unstake_percent: u64) -> Result<()> {
        Ok(())
    }
}
