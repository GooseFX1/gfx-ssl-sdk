use anchor_lang::{prelude::*, InstructionData, ToAccountMetas};
use anchor_spl::{associated_token::get_associated_token_address, token::Token};
use gfx_ssl_interface::{skey, LiquidityAccount, PDAIdentifier, Pair, SSL};
use solana_program::{instruction::Instruction, sysvar::SysvarId};

/// The instructions all contain nearly the same required arguments.
/// This struct provides a more succinct, reusable input to
/// the instruction factory functions below.
#[derive(Debug, Clone)]
pub struct SSLInstructionContext {
    controller: Pubkey,
    ssl: Pubkey,
    ssl_mint: Pubkey,
    user_wallet: Pubkey,
    liquidity_account: Pubkey,
}

impl SSLInstructionContext {
    pub fn new(controller: Pubkey, ssl_mint: Pubkey, user_wallet: Pubkey) -> Self {
        let ssl = SSL::get_address(&[controller.as_ref(), ssl_mint.as_ref()]);
        let liquidity_account = LiquidityAccount::get_address(&[
            controller.as_ref(),
            ssl_mint.as_ref(),
            user_wallet.as_ref(),
        ]);
        Self {
            controller,
            ssl,
            ssl_mint,
            user_wallet,
            liquidity_account,
        }
    }
}

pub fn create_liquidity_account(ctx: &SSLInstructionContext) -> Instruction {
    let data = gfx_ssl_interface::instruction::CreateLiquidityAccount.data();
    let accounts = gfx_ssl_interface::accounts::CreateLiquidityAccount {
        controller: ctx.controller.clone(),
        ssl: ctx.ssl.clone(),
        liquidity_account: ctx.liquidity_account.clone(),
        user_wallet: ctx.user_wallet.clone(),
        system_program: System::id(),
        rent: Rent::id(),
    }
    .to_account_metas(None);
    Instruction {
        data,
        accounts,
        program_id: gfx_ssl_interface::id(),
    }
}

pub fn deposit(ctx: &SSLInstructionContext, amount: u64) -> Instruction {
    let rt_vault = get_associated_token_address(&ctx.ssl, &ctx.ssl_mint);
    let user_rt_ata = get_associated_token_address(&ctx.user_wallet, &ctx.ssl_mint);
    let data = gfx_ssl_interface::instruction::Deposit { amount }.data();
    let accounts = gfx_ssl_interface::accounts::Deposit {
        controller: ctx.controller.clone(),
        ssl: ctx.ssl.clone(),
        liquidity_account: ctx.liquidity_account.clone(),
        rt_vault,
        user_rt_ata,
        user_wallet: ctx.user_wallet.clone(),
        token_program: Token::id(),
    }
    .to_account_metas(None);
    Instruction {
        data,
        accounts,
        program_id: gfx_ssl_interface::id(),
    }
}

pub fn withdraw(ctx: &SSLInstructionContext, withdraw_percent: u64) -> Instruction {
    let rt_vault = get_associated_token_address(&ctx.ssl, &ctx.ssl_mint);
    let user_rt_ata = get_associated_token_address(&ctx.user_wallet, &ctx.ssl_mint);
    let data = gfx_ssl_interface::instruction::Withdraw { withdraw_percent }.data();
    let accounts = gfx_ssl_interface::accounts::Withdraw {
        controller: ctx.controller.clone(),
        ssl: ctx.ssl.clone(),
        liquidity_account: ctx.liquidity_account.clone(),
        rt_vault,
        user_rt_ata,
        user_wallet: ctx.user_wallet.clone(),
        token_program: Token::id(),
    }
    .to_account_metas(None);
    Instruction {
        data,
        accounts,
        program_id: gfx_ssl_interface::id(),
    }
}

/// Factored out for use in `jupiter` module to fulfill `Amm` trait contract.
pub(crate) fn swap_account_metas(
    ctx: &SSLInstructionContext,
    ssl_in_mint: &Pubkey,
    ssl_out_mint: &Pubkey,
    fee_collector: &Pubkey,
) -> Vec<AccountMeta> {
    let ssl_in = SSL::get_address(&[ctx.controller.as_ref(), ssl_in_mint.as_ref()]);
    let ssl_out = SSL::get_address(&[ctx.controller.as_ref(), ssl_out_mint.as_ref()]);
    let pair = Pair::get_address(&[
        skey::<_, true>(ssl_in_mint, ssl_out_mint).as_ref(),
        skey::<_, false>(ssl_in_mint, ssl_out_mint).as_ref(),
    ]);
    let liability_vault_in = get_associated_token_address(&ssl_in, ssl_in_mint);
    let swapped_liability_vault_in = get_associated_token_address(&ssl_in, ssl_out_mint);
    let liability_vault_out = get_associated_token_address(&ssl_out, ssl_in_mint);
    let swapped_liability_vault_out = get_associated_token_address(&ssl_out, ssl_out_mint);
    let user_in_ata = get_associated_token_address(&ctx.user_wallet, ssl_in_mint);
    let user_out_ata = get_associated_token_address(&ctx.user_wallet, ssl_out_mint);
    let fee_collector_ata = get_associated_token_address(&fee_collector, ssl_in_mint);
    gfx_ssl_interface::accounts::Swap {
        controller: ctx.controller.clone(),
        pair,
        ssl_in,
        ssl_out,
        liability_vault_in,
        swapped_liability_vault_in,
        liability_vault_out,
        swapped_liability_vault_out,
        user_in_ata,
        user_out_ata,
        fee_collector_ata,
        user_wallet: ctx.user_wallet.clone(),
        fee_collector: fee_collector.clone(),
        token_program: Token::id(),
    }
    .to_account_metas(None)
}

pub fn swap(
    ctx: &SSLInstructionContext,
    ssl_in_mint: &Pubkey,
    ssl_out_mint: &Pubkey,
    fee_collector: &Pubkey,
    amount_in: u64,
    min_out: u64,
) -> Instruction {
    let data = gfx_ssl_interface::instruction::Swap { amount_in, min_out }.data();
    let accounts = swap_account_metas(ctx, ssl_in_mint, ssl_out_mint, fee_collector);
    Instruction {
        data,
        accounts,
        program_id: gfx_ssl_interface::id(),
    }
}
