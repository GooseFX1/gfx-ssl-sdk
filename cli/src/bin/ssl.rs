use clap::{Parser, IntoApp};
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use gfx_cli::config::get_solana_cli_config;
use gfx_cli::interface::ssl::{entry, Opts};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    let app = Opts::into_app();
    let matches = app.get_matches();

    // Solana CLI config file
    let config = get_solana_cli_config()?;

    // Signer and URL
    let signer = opts.keypair.resolve(&matches, Some(&config))?;
    let url = opts.url.resolve(Some(&config))?;
    let client = RpcClient::new_with_commitment(
        &url,CommitmentConfig::processed());
    entry(
        &opts,
        signer,
        client,
    )
}
