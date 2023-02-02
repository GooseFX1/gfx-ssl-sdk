use anchor_lang::{prelude::*, InstructionData, ToAccountMetas};
use anchor_spl::{associated_token, associated_token::get_associated_token_address, token::Token};
use anchor_spl::token::spl_token;
use gfx_controller_interface::{Controller, PDAIdentifier, StakingAccount};
use solana_program::{instruction::Instruction, pubkey::Pubkey, system_program, sysvar, sysvar::SysvarId};

/// The instructions all contain nearly the same required arguments.
/// This struct provides a more succinct, reusable input to
/// the instruction factory functions below.
#[derive(Debug, Clone)]
pub struct ControllerInstructionContext {
    controller: Pubkey,
    user_wallet: Pubkey,
    staking_account: Pubkey,
}

impl ControllerInstructionContext {
    pub fn new(controller: Pubkey, user_wallet: Pubkey) -> Self {
        let staking_account =
            StakingAccount::get_address(&[controller.as_ref(), user_wallet.as_ref()]);
        Self {
            controller,
            user_wallet,
            staking_account,
        }
    }
}

pub fn create_controller(
    seed: [u8; 32],
    mint: Pubkey,
    user_wallet: Pubkey,
) -> Instruction {
    let controller = Controller::get_address(&[&seed]);
    let data = gfx_controller_interface::instruction::CreateController { seed }.data();
    let accounts = gfx_controller_interface::accounts::CreateController {
            controller,
            mint,
            vault: get_associated_token_address(&controller, &mint),
            admin: user_wallet,
            admin_ata: get_associated_token_address(&user_wallet, &mint),
            token_program: spl_token::id(),
            associated_token_program: associated_token::AssociatedToken::id(),
            system_program: system_program::id(),
            rent: sysvar::rent::id(),
    }.to_account_metas(None);
    Instruction {
        data,
        accounts,
        program_id: gfx_controller_interface::id(),
    }
}

pub fn create_staking_account(ctx: &ControllerInstructionContext) -> Instruction {
    let data = gfx_controller_interface::instruction::CreateStakingAccount.data();
    let accounts = gfx_controller_interface::accounts::CreateStakingAccount {
        controller: ctx.controller.clone(),
        staking_account: ctx.staking_account.clone(),
        user_wallet: ctx.user_wallet.clone(),
        system_program: System::id(),
        rent: Rent::id(),
    }
    .to_account_metas(None);
    Instruction {
        data,
        accounts,
        program_id: gfx_controller_interface::id(),
    }
}

pub fn stake(
    ctx: &ControllerInstructionContext,
    controller_mint: &Pubkey,
    amount: u64,
) -> Instruction {
    let vault = get_associated_token_address(&ctx.controller, controller_mint);
    let user_ata = get_associated_token_address(&ctx.user_wallet, controller_mint);
    let data = gfx_controller_interface::instruction::Stake { amount }.data();
    let accounts = gfx_controller_interface::accounts::Stake {
        controller: ctx.controller.clone(),
        staking_account: ctx.staking_account.clone(),
        vault,
        user_ata,
        user_wallet: ctx.user_wallet.clone(),
        token_program: Token::id(),
    }
    .to_account_metas(None);
    Instruction {
        data,
        accounts,
        program_id: gfx_controller_interface::id(),
    }
}

pub fn unstake(
    ctx: &ControllerInstructionContext,
    controller_mint: &Pubkey,
    controller_admin: &Pubkey,
    unstake_percent: u64,
) -> Instruction {
    let vault = get_associated_token_address(&ctx.controller, controller_mint);
    let user_ata = get_associated_token_address(&ctx.user_wallet, controller_mint);
    let fee_collector_ata = get_associated_token_address(controller_admin, controller_mint);
    let data = gfx_controller_interface::instruction::Unstake { unstake_percent }.data();
    let accounts = gfx_controller_interface::accounts::Unstake {
        controller: ctx.controller.clone(),
        staking_account: ctx.staking_account.clone(),
        vault,
        user_ata,
        fee_collector_ata,
        user_wallet: ctx.user_wallet.clone(),
        token_program: Token::id(),
    }
    .to_account_metas(None);
    Instruction {
        data,
        accounts,
        program_id: gfx_controller_interface::id(),
    }
}
