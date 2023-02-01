use anyhow::Result;
use clap::Parser;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;
use crate::config::{KeypairArg, UrlArg};


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
    GetPair {
        address: Pubkey,
    },
}

pub fn entry(
    opts: &Opts,
    signer: Box<dyn Signer>,
    client: RpcClient,
) -> Result<()> {
    match &opts.command {
        Command::GetPair {address} => {
            get_pair(
                client,
                *address,
            )?;
        }
    }
    Ok(())
}

pub fn get_pair(
    client: RpcClient,
    address: Pubkey,
) -> Result<()> {
    let pair = gfx_ssl_sdk::ssl::state::get_pair_blocking(
        &address, &client
    )?;
    println!("Found pair: {}", &address);
    println!("{:#?}", pair);

    Ok(())
}
