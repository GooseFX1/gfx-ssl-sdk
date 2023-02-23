use std::collections::HashMap;
use itertools::Itertools;
use std::env::var;
use jupiter_core::amm::{Amm, KeyedAccount, QuoteParams};
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::pubkey;
use gfx_ssl_sdk::jupiter::GfxAmm;

pub const SOL_USDC: Pubkey = pubkey!("CpfpL9PXt88u3kPQ6fuD6WqQpQ8c5UEftxsop9rm1ATM");

#[test]
fn sol_usdc_pair() {
    let url =
        var("SOLANA_RPC_URL").unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string());
    let client = RpcClient::new(url);
    let account = client.get_account(&SOL_USDC).unwrap();
    let keyed_account = KeyedAccount {
        key: SOL_USDC,
        account,
        params: None
    };
    let mut amm = GfxAmm::from_keyed_account(keyed_account).unwrap();
    let accounts_to_update = amm.get_accounts_to_update();
    let mut acts_data = HashMap::new();
    // get_multiple_accounts only allows to get 5 accounts at a time?
    for chunk in &accounts_to_update.iter().chunks(5) {
        let accs: Vec<_> = Vec::from_iter(chunk.copied());
        acts_data.extend(
            client
                .get_multiple_accounts(&accs)
                .unwrap()
                .into_iter()
                .zip(accs)
                .map(|(act, key)| (key, act.unwrap().data)),
        );
    }
    amm.update(&acts_data).unwrap();
    println!("Pair: {:?}", amm.pair_pubkey);

    let mints = amm.get_reserve_mints();

    match amm.quote(&QuoteParams {
        in_amount: (1. / 20. * 1e9) as u64,
        input_mint: mints[0],
        output_mint: mints[1],
    }) {
        Ok(quote) => {
            println!("Quote: {:#?}", quote);
            let mut price = (quote.in_amount as f64 / 1e9)
                / (quote.out_amount as f64 / 1e6);
            price = 1. / price;
            println!("Price: {}\n", price);
        }
        Err(e) => {
            println!("Error: {e}\n");
        }
    }
}