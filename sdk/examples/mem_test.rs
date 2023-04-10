use std::collections::HashMap;
use std::env::var;
use jupiter_core::amm::{Amm, KeyedAccount, QuoteParams};
use solana_client::rpc_client::RpcClient;
use gfx_ssl_interface::Pair;
use gfx_ssl_sdk::jupiter::GfxAmm;
use itertools::Itertools;
use anchor_lang::prelude::*;
use solana_sdk::pubkey;

const NUM_ITERATIONS: u32 = 1_000;

const SOL_USDC: Pubkey = pubkey!("CpfpL9PXt88u3kPQ6fuD6WqQpQ8c5UEftxsop9rm1ATM");

fn main() {
    let url =
        var("SOLANA_RPC_URL").unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string());
    let client = RpcClient::new(url);
    let account = client.get_account(&SOL_USDC).unwrap();
    let pair =
        Pair::try_deserialize(&mut account.data.as_slice()).expect("Could not deserialize pair");
    println!("{:#?}", pair);
    let keyed_account = KeyedAccount {
        key: SOL_USDC,
        account,
        params: None,
    };
    let mut amm = GfxAmm::from_keyed_account(keyed_account.clone()).unwrap();
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
                .map(|(act, key)| {
                    (
                        key,
                        act.expect(&format!("account not found: {}", key.to_string()))
                            .data,
                    )
                }),
        );
    }
    amm.update(&acts_data).unwrap();
    println!("Pair: {:?}", amm.pair_pubkey);

    let mints = amm.get_reserve_mints();

    for i in 0..NUM_ITERATIONS {
        let mut amm = GfxAmm::from_keyed_account(keyed_account.clone()).unwrap();
        amm.update(&acts_data).unwrap();
        match amm.quote(&QuoteParams {
            in_amount: u64::MAX,
            input_mint: mints[0],
            output_mint: mints[1],
        }) {
            Ok(quote) => {
            }
            Err(e) => {
            }
        }
        if i % 1000 == 0 {
            println!("Finished {i}th iteration.")
        }
    }
}