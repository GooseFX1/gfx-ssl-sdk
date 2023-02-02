use anyhow::Result;
use clap::Parser;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;
use crate::config::{KeypairArg, UrlArg};
use crate::maybe_print_preflight_simulation_logs;


/// Gfx Controller CLI
#[derive(Debug, Parser)]
pub struct Opts {
    #[clap(flatten)]
    pub url: UrlArg,
    #[clap(flatten)]
    pub keypair: KeypairArg,
    #[clap(subcommand)]
    pub command: Command,
}


#[derive(Debug, Parser)]
pub enum Command {
    CreateController {
        mint: Pubkey,
    },
}

pub fn entry(
    opts: &Opts,
    signer: Box<dyn Signer>,
    client: RpcClient,
) -> Result<()> {
    match &opts.command {
        Command::CreateController {mint} => {
            create_controller(
                client,
                *mint,
                signer,
            )?;
        }
    }
    Ok(())
}

pub fn create_controller(
    client: RpcClient,
    mint: Pubkey,
    payer: Box<dyn Signer>,
) -> Result<()> {
    let seed = Keypair::new().pubkey().to_bytes();

    let ix = gfx_ssl_sdk::controller::instructions::create_controller(
        seed,
        mint,
        payer.pubkey(),
    );
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &vec![payer],
        client.get_latest_blockhash()?
    );
    let signature = client.send_transaction(&tx)
        .map_err(maybe_print_preflight_simulation_logs)?;
    println!("Create Controller success: {}", &signature);
    println!(
        "https://explorer.solana.com/tx/{}?cluster=custom&customUrl=http%3A%2F%2Flocalhost%3A8899",
        signature
    );

    Ok(())
}
