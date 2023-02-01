use anchor_lang_26::AccountSerialize;
use anchor_localnet::{cli::SolanaLocalnetCli, LocalnetAccount, SystemAccount, TestTomlGenerator};
use anyhow::anyhow;
use clap::Parser;
use gfx_controller_interface::{Controller, PDAIdentifier};
use gfx_ssl_interface::{PDAIdentifier as OtherPDAIdentifier, Pair, SSL, Oracle};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    program_option::COption, pubkey, pubkey::Pubkey, signature::read_keypair_file, signer::Signer,
};
use gfx_ssl_interface::svec5::StackVec5;

/// Mainnet pubkey
pub const SSL_PROGRAM_ID: Pubkey = pubkey!("7WduLbRfYhTJktjLw5FDEyrqoEv61aTTCuGAetgLjzN5");
/// Mainnet pubkey
pub const CONTROLLER_PROGRAM_ID: Pubkey = pubkey!("8KJx48PYGHVC9fxzRRtYp4x4CM2HyYCm2EjVuAP4vvrx");

/// Arbitrary readable choice
pub const MINT_DECIMALS: u8 = 3;

/// Produce a new mint and ATA, both of which are owned by the `authority`.
/// The ATA balance and mint supply are both populated with a million "satoshis"
/// of the new mint.
pub fn new_mint_and_ata(authority: Pubkey) -> (LocalnetAccount, LocalnetAccount) {
    let mint = LocalnetAccount::new(
        Pubkey::new_unique(),
        "mint.json".to_string(),
        anchor_spl::token::Mint::from(spl_token::state::Mint {
            mint_authority: COption::Some(authority),
            supply: 1000_000,
            decimals: MINT_DECIMALS,
            is_initialized: true,
            freeze_authority: COption::Some(authority),
        }),
    );
    let ata = LocalnetAccount::new(
        Pubkey::new_unique(),
        "test_user_token_act.json".to_string(),
        anchor_spl::token::TokenAccount::from(spl_token::state::Account {
            mint: mint.address,
            owner: authority,
            amount: 1000_000,
            delegate: COption::None,
            state: spl_token::state::AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::Some(authority),
        }),
    );
    (mint, ata)
}

/// Create a new SSL using the input mint and controller.
fn new_ssl(
    controller: Pubkey,
    mint: Pubkey,
) -> SSL {
    SSL {
        controller,
        mint,
        decimals: MINT_DECIMALS,
        bump: 0,
        pt_bump: 0,
        suspended: false,
        cranker: Default::default(),
        weight: 0,
        liability: 0,
        swapped_liability: 0,
        total_share: 0,
        _pad: [0; 32]
    }
}

fn main() -> anyhow::Result<()> {
    // As a one-time setup, this filesystem key needs to be generated manually with:
    // solana-keygen new -o localnet_wallet.json
    let localnet_wallet = read_keypair_file("localnet_wallet.json").map_err(|e| {
        anyhow!(
            "Failed to read keypair file: {e}
you may need to run solana-keygen new -o localnet_wallet.json"
        )
    })?;

    // This will function as an admin account, as well as a user account
    let admin = LocalnetAccount::new(
        localnet_wallet.pubkey(),
        "localnet_wallet.json".to_string(),
        SystemAccount,
    );

    // Controller
    let (controller_mint, controller_admin_ata) = new_mint_and_ata(admin.address);
    let seed = Pubkey::new_unique().to_bytes();
    let (controller_address, bump) = Controller::get_address_with_bump(&vec![seed.as_slice()]);
    let controller_account = Controller {
        seed: Pubkey::new_unique().to_bytes(),
        bump,
        admin: admin.address,
        suspended: false,
        decimals: MINT_DECIMALS,
        mint: controller_mint.address,
        daily_reward: 1,
        total_staking_share: 0,
        staking_balance: 0,
        last_distribution_time: 0,
        withdraw_fee: 0,
        _pad0: [0; 6],
        _pad1: [0; 31],
    };
    // Need to use LocalnetAccount::new_raw instead of LocalnetAccount::new
    // because the Anchor localnet crate currently resides on a fork on Anchor,
    // which causes conflicting versions of the AccountSerialize trait.
    let mut serialized = Vec::new();
    controller_account.try_serialize(&mut serialized).unwrap();
    let controller_account =
        LocalnetAccount::new_raw(controller_address, "controller".to_string(), serialized);

    // SSL 1
    let (ssl_1_mint, ssl_1_ata) = new_mint_and_ata(admin.address);
    let ssl_address = SSL::get_address(&vec![
        controller_address.as_ref(),
        ssl_1_mint.address.as_ref(),
    ]);
    let ssl = new_ssl(controller_address, ssl_1_mint.address);
    let mut serialized = Vec::new();
    ssl.try_serialize(&mut serialized).unwrap();
    let ssl_1 = LocalnetAccount::new_raw(
        ssl_address,
        "ssl_account_1".to_string(),
        serialized
    );

    // SSL 2
    let (ssl_2_mint, ssl_2_ata) = new_mint_and_ata(admin.address);
    let ssl_address = SSL::get_address(&vec![
        controller_address.as_ref(),
        ssl_2_mint.address.as_ref(),
    ]);
    let ssl = new_ssl(controller_address, ssl_2_mint.address);
    let mut serialized = Vec::new();
    ssl.try_serialize(&mut serialized).unwrap();
    let ssl_2 = LocalnetAccount::new_raw(
        ssl_address,
        "ssl_account_2".to_string(),
        serialized
    );

    // To clone accounts from mainnet
    let client = RpcClient::new("https://api.mainnet-beta.solana.com");

    // Cloned oracle accounts
    // On localnet testing, these do not need to actually match
    // any given mint that they're associated with.
    let sol_usd_oracle = LocalnetAccount::new_from_clone_unchecked(
        &pubkey!("H6ARHf6YXhGYeQfUzQNGk6rDNnLBQKrenN712K4AQJEG"),
        &client,
        "sol_usd_oracle".to_string(),
    )?;
    let usdc_usd_oracle = LocalnetAccount::new_from_clone_unchecked(
        &pubkey!("Gnt27xtC473ZT2Mw5u8wZ68Z3gULkSTb5DuxJy7eJotD"),
        &client,
        "usdc_usd_oracle".to_string(),
    )?;

    // TODO Review the svec initialization and other fields
    // Pair
    let mut pair = Pair::default();
    let mut is_reversed = false;
    pair.mints = if ssl_1_mint.address < ssl_2_mint.address {
        (ssl_1_mint.address, ssl_2_mint.address)
    } else {
        is_reversed = true;
        (ssl_2_mint.address, ssl_1_mint.address)
    };
    pair.controller = controller_address;
    // Localnet starts at slot 1, so max_delay should never be an issue.
    pair.max_delay = 1;
    pair.confidence = 1;
    pair.A = 1;  // TODO Review this
    let mut svec = StackVec5::<Oracle>::default();
    svec[0].path[0] = (sol_usd_oracle.address, false);
    svec[0].path[1] = (usdc_usd_oracle.address, false);
    pair.oracles = if is_reversed {
        svec[0].path[0] = (svec[0].path[1].0, true);
        svec
    } else {
        svec
    };
    let (pair_address, bump) = Pair::get_address_with_bump(&vec![
        pair.controller.as_ref(),
        pair.mints.0.as_ref(),
        pair.mints.1.as_ref(),
    ]);
    let mut serialized = Vec::new();
    pair.try_serialize(&mut serialized).unwrap();
    let pair = LocalnetAccount::new_raw(
        Default::default(),
        "pair".to_string(),
        serialized,
    );

    // Putting all accounts together, with the dumped SSL / Controller programs.
    // All JSON files will build in `<project-root>/tests`.
    // We put a dummy file `tests/test.ts` because it is required for localnet to start.
    let toml = TestTomlGenerator {
        save_directory: "./tests".to_string(),
        test_file_glob: Some("tests/test.ts".to_string()),
        accounts: vec![
            admin,
            controller_mint,
            controller_admin_ata,
            controller_account,
            ssl_1_mint,
            ssl_1_ata,
            ssl_1,
            ssl_2_mint,
            ssl_2_ata,
            ssl_2,
            pair,
            sol_usd_oracle,
            usdc_usd_oracle,
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

    // Parse CLI arguments, process the parsed command.
    let opts = SolanaLocalnetCli::parse();
    opts.process(vec![toml])?;
    Ok(())
}
