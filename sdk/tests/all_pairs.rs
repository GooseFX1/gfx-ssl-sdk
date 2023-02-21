use gfx_ssl_sdk::jupiter::{GfxAmm, RESERVE_MINTS};
use itertools::Itertools;
use jupiter_core::amm::{Amm, QuoteParams};
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::pubkey;
use std::{
    collections::{HashMap, HashSet},
    env::var,
};

#[test]
fn all_pairs() {
    let url =
        var("SOLANA_RPC_URL").unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string());
    let client = RpcClient::new(url);
    let mut input_amounts = HashMap::new();

    input_amounts.insert(
        pubkey!("So11111111111111111111111111111111111111112"),
        (1. / 20. * 1e9) as u64,
    );
    input_amounts.insert(
        pubkey!("7vfCXTUXx5WJV5JADk17DUJ4ksgau7utNKj4b963voxs"),
        (1. / 1500. * 1e8) as u64,
    );
    input_amounts.insert(
        pubkey!("mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So"),
        (1. / 20. * 1e9) as u64,
    );
    input_amounts.insert(
        pubkey!("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB"),
        (1. * 1e6) as u64,
    );
    input_amounts.insert(
        pubkey!("orcaEKTdK7LKz57vaAYr9QeNsVEPfiu6QeMU1kektZE"),
        (1. * 1e6) as u64,
    );
    input_amounts.insert(
        pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
        (1. * 1e6) as u64,
    );
    input_amounts.insert(
        pubkey!("7dHbWXmci3dT8UFYWYZweBLXgycu7Y3iL6trKn1Y7ARj"),
        (1. / 20. * 1e9) as u64,
    );

    let mut decimals = HashMap::new();
    decimals.insert(pubkey!("So11111111111111111111111111111111111111112"), 1e9);
    decimals.insert(pubkey!("7vfCXTUXx5WJV5JADk17DUJ4ksgau7utNKj4b963voxs"), 1e8);
    decimals.insert(pubkey!("mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So"), 1e9);
    decimals.insert(pubkey!("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB"), 1e6);
    decimals.insert(pubkey!("orcaEKTdK7LKz57vaAYr9QeNsVEPfiu6QeMU1kektZE"), 1e6);
    decimals.insert(pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"), 1e6);
    decimals.insert(pubkey!("7dHbWXmci3dT8UFYWYZweBLXgycu7Y3iL6trKn1Y7ARj"), 1e9);
    const USDT: Pubkey = pubkey!("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB");
    const USDC: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");

    let mut accounts_to_update = HashSet::new();
    let mut amms = HashMap::new();
    for mints in RESERVE_MINTS.iter().permutations(2) {
        let (m1, m2) = (mints[0], mints[1]);
        let amm = GfxAmm::new(*m1, *m2).unwrap();
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

    // Repeat the data fetching
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

    let mut failures = Vec::new();
    // Update the AMM objects again, and get quotes
    for ((m1, m2), amm) in amms.iter_mut() {
        amm.update(&acts_data).unwrap();
        accounts_to_update.extend(amm.get_accounts_to_update());

        println!("Testing mints {} -> {}", m1, m2);

        println!("Pair: {:?}", amm.pair_pubkey);

        match amm.quote(&QuoteParams {
            in_amount: *input_amounts.get(&m1).unwrap(),
            input_mint: **m1,
            output_mint: **m2,
        }) {
            Ok(quote) => {
                println!("Quote: {:#?}", quote);
                let mut price = (quote.in_amount as f64 / decimals[&m1])
                    / (quote.out_amount as f64 / decimals[&m2]);
                // if it is usd pair, always show X/USD price.
                if **m2 == USDT || **m2 == USDC {
                    price = 1. / price
                }
                println!("Price: {}\n", price);
            }
            Err(e) => {
                println!("Error: {e}\n");
                failures.push((amm.label(), m1, m2));
            }
        }
    }
    if failures.len() > 0 {
        println!("Failed to get quotes for {} pairs", failures.len());
        failures.iter().for_each(|failure| {
            println!("{}: {} -> {}", failure.0, failure.1, failure.2);
        });
    }
}
