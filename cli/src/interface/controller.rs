use anyhow::Result;
use clap::Parser;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use gfx_controller_interface::{Controller, PDAIdentifier};
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

    let controller = Controller::get_address(&[&seed]);
    let ix = gfx_ssl_sdk::controller::instructions::create_controller(
        controller,
        mint,
        payer.pubkey(),
    );
    Ok(())
}

// pub fn new_multisig(
//     threshold: u16,
//     members: &Vec<String>,
//     include_signer: bool,
//     client: RpcClient,
//     payer: &dyn Signer,
//     matches: &ArgMatches,
// ) -> Result<()> {
//     let mut members: Vec<Pubkey> = members
//         .iter()
//         .map(|path| pubkey_or_signer_path(path, matches))
//         .flatten()
//         .collect();
//     if include_signer {
//         members.push(payer.pubkey());
//     }
//     let base = Keypair::new();
//     let multisig_address = find_multisig_wallet_address(&base.pubkey());
//     println!("Creating multisig wallet: {}", multisig_address.to_string());
//     let signature = new_multisig_rpc(
//         threshold,
//         members.clone(),
//         &client,
//         payer,
//         Some(&base),
//     )?;
//     println!("New multisig successfully created. \
//     signature: {}", signature.to_string());
//     Ok(())
// }
