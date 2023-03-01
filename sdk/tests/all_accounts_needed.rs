use gfx_ssl_sdk::jupiter::{GfxAmm, RESERVE_MINTS};
use itertools::Itertools;
use jupiter_core::amm::Amm;
use solana_client::rpc_client::RpcClient;
use std::{
    collections::{HashMap, HashSet},
    env::var,
};

#[test]
fn all_accounts_needed() {
    let url =
        var("SOLANA_RPC_URL").unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string());
    let client = RpcClient::new(url);
    let mut accounts_to_update = HashSet::new();
    let mut amms = HashMap::new();
    for mints in RESERVE_MINTS.iter().permutations(2) {
        let (m1, m2) = (mints[0], mints[1]);
        let amm = GfxAmm::new(*m1, *m2).unwrap();
        // Don't add if it's in error
        if client.get_account(&amm.pair_pubkey).is_err() {
            continue;
        }
        accounts_to_update.extend(amm.get_accounts_to_update());
        amms.insert((m1, m2), amm);
    }

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
    // Update the AMM objects
    for (_, amm) in amms.iter_mut() {
        amm.update(&acts_data).unwrap();
        accounts_to_update.extend(amm.get_accounts_to_update());
    }

    // // Repeat the data fetching
    // for chunk in &accounts_to_update.iter().chunks(5) {
    //     let accs: Vec<_> = Vec::from_iter(chunk.copied());
    //     acts_data.extend(
    //         client
    //             .get_multiple_accounts(&accs)
    //             .unwrap()
    //             .into_iter()
    //             .zip(accs)
    //             .map(|(act, key)| (key, act.unwrap().data)),
    //     );
    // }
    //
    // let mut failures = Vec::new();
    // // Update the AMM objects again, and get quotes
    // for ((m1, m2), amm) in amms.iter_mut() {
    //     amm.update(&acts_data).unwrap();
    //     accounts_to_update.extend(amm.get_accounts_to_update());
    //
    //     println!("Testing mints {} -> {}", m1, m2);
    //
    //     println!("Pair: {:?}", amm.pair_pubkey);
    //
    //     match amm.quote(&QuoteParams {
    //         in_amount: *input_amounts.get(&m1).unwrap(),
    //         input_mint: **m1,
    //         output_mint: **m2,
    //     }) {
    //         Ok(quote) => {
    //             println!("Quote: {:#?}", quote);
    //             let mut price = (quote.in_amount as f64 / decimals[&m1])
    //                 / (quote.out_amount as f64 / decimals[&m2]);
    //             // if it is usd pair, always show X/USD price.
    //             if **m2 == USDT || **m2 == USDC {
    //                 price = 1. / price
    //             }
    //             println!("Price: {}\n", price);
    //         }
    //         Err(e) => {
    //             println!("Error: {e}\n");
    //             failures.push((amm.label(), m1, m2));
    //         }
    //     }
    // }
    // if failures.len() > 0 {
    //     println!("Failed to get quotes for {} pairs", failures.len());
    //     failures.iter().for_each(|failure| {
    //         println!("{}: {} -> {}", failure.0, failure.1, failure.2);
    //     });
    // }
    println!("{:#?}", accounts_to_update.iter().collect::<Vec<_>>());
}
