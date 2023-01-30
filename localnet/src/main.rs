use anchor_lang_26::AccountSerialize;
use anchor_localnet::cli::SolanaLocalnetCli;
use anchor_localnet::{LocalnetAccount, SystemAccount, TestTomlGenerator};
use anyhow::anyhow;
use clap::Parser;
use solana_sdk::program_option::COption;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::read_keypair_file;
use solana_sdk::signer::Signer;
use gfx_controller_interface::{Controller, PDAIdentifier};

/// Mainnet pubkey
pub const SSL_PROGRAM_ID: Pubkey = pubkey!("7WduLbRfYhTJktjLw5FDEyrqoEv61aTTCuGAetgLjzN5");
/// Mainnet pubkey
pub const CONTROLLER_PROGRAM_ID: Pubkey = pubkey!("8KJx48PYGHVC9fxzRRtYp4x4CM2HyYCm2EjVuAP4vvrx");

pub const CONTROLLER_MINT_DECIMALS: u8 = 9;

fn main() -> anyhow::Result<()> {
    // As a one-time setup, this filesystem key needs to be generated manually with:
    // solana-keygen new -o localnet_wallet.json
    let localnet_wallet = read_keypair_file("localnet_wallet.json")
        .map_err(|e| anyhow!("Failed to read keypair file: {e}
you may need to run solana-keygen new -o localnet_wallet.json"))?;

    // This will function as an admin account, as well as a user account
    let controller_admin = LocalnetAccount::new(
        localnet_wallet.pubkey(),
        "localnet_wallet.json".to_string(),
        SystemAccount,
    );
    // We need a mint for the controller
    let controller_mint = LocalnetAccount::new(
        Pubkey::new_unique(),
        "mint.json".to_string(),
        anchor_spl::token::Mint::from(spl_token::state::Mint {
            mint_authority: COption::Some(controller_admin.address),
            supply: 0,
            decimals: CONTROLLER_MINT_DECIMALS,
            is_initialized: true,
            freeze_authority: COption::Some(controller_admin.address),
        })
    );
    let controller_admin_ata = LocalnetAccount::new(
        Pubkey::new_unique(),
        "test_user_token_act.json".to_string(),
        anchor_spl::token::TokenAccount::from(spl_token::state::Account {
            mint: controller_mint.address,
            owner: controller_admin.address,
            amount: 0,
            delegate: COption::None,
            state: spl_token::state::AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::Some(controller_admin.address)
        })
    );

    let seed = Pubkey::new_unique().to_bytes();
    let (controller_address, bump) = Controller::get_address_with_bump(
        &vec![
            seed.as_slice(),
        ]
    );

    let controller_account = Controller {
        seed: Pubkey::new_unique().to_bytes(),
        bump,
        admin: controller_admin.address,
        suspended: false,
        decimals: CONTROLLER_MINT_DECIMALS,
        mint: controller_mint.address,
        daily_reward: 1,
        total_staking_share: 0,
        staking_balance: 0,
        last_distribution_time: 0,
        withdraw_fee: 0,
        _pad0: [0; 6],
        _pad1: [0; 31]
    };
    let mut serialized = Vec::new();
    controller_account.try_serialize(&mut serialized).unwrap();
    let controller_account = LocalnetAccount::new_raw(
        controller_address,
        "controller".to_string(),
        serialized,
    );

    let toml = TestTomlGenerator {
        save_directory: "./tests".to_string(),
        test_file_glob: Some("tests/test.ts".to_string()),
        accounts: vec![
            controller_admin,
            controller_mint,
            controller_admin_ata,
            controller_account
        ],
        programs: vec![
            (
                SSL_PROGRAM_ID.to_string(),
                "../dump/ssl.so".to_string(),
            ),
            (
                CONTROLLER_PROGRAM_ID.to_string(),
                "../dump/controller.so".to_string(),
            ),
        ],
        ..Default::default()
    };

    let opts = SolanaLocalnetCli::parse();
    opts.process(vec![
        toml,
    ])?;
    Ok(())
}