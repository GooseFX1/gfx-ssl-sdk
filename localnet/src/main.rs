use anchor_localnet::cli::SolanaLocalnetCli;
use anchor_localnet::{LocalnetAccount, SystemAccount, TestTomlGenerator};
use anyhow::anyhow;
use clap::Parser;
use solana_sdk::program_option::COption;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::read_keypair_file;
use solana_sdk::signer::Signer;

/// Mainnet pubkey
pub const SSL_PROGRAM_ID: Pubkey = pubkey!("7WduLbRfYhTJktjLw5FDEyrqoEv61aTTCuGAetgLjzN5");
/// Mainnet pubkey
pub const CONTROLLER_PROGRAM_ID: Pubkey = pubkey!("8KJx48PYGHVC9fxzRRtYp4x4CM2HyYCm2EjVuAP4vvrx");


fn main() -> anyhow::Result<()> {
    // As a one-time setup, this filesystem key needs to be generated manually with:
    // solana-keygen new -o localnet_wallet.json
    let localnet_wallet = read_keypair_file("localnet_wallet.json")
        .map_err(|e| anyhow!("Failed to read keypair file: {e}
you may need to run solana-keygen new -o localnet_wallet.json"))?;

    // This will function as an admin account, as well as a user account
    let test_user = LocalnetAccount::new(
        localnet_wallet.pubkey(),
        "localnet_wallet.json".to_string(),
        SystemAccount,
    );
    // We need a mint for the controller
    let test_mint = LocalnetAccount::new(
        Pubkey::new_unique(),
        "mint.json".to_string(),
        anchor_spl::token::Mint::from(spl_token::state::Mint {
            mint_authority: COption::Some(test_user.address),
            supply: 0,
            decimals: 9,
            is_initialized: true,
            freeze_authority: COption::Some(test_user.address),
        })
    );
    let test_token_account = LocalnetAccount::new(
        Pubkey::new_unique(),
        "test_user_token_act.json".to_string(),
        anchor_spl::token::TokenAccount::from(spl_token::state::Account {
            mint: test_mint.address,
            owner: test_user.address,
            amount: 0,
            delegate: COption::None,
            state: spl_token::state::AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::Some(test_user.address)
        })
    );

    let toml = TestTomlGenerator {
        save_directory: "./tests".to_string(),
        test_file_glob: Some("tests/test.ts".to_string()),
        accounts: vec![
            test_user,
            test_mint,
            test_token_account,
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