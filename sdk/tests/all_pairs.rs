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

    let mut accounts_to_update = HashSet::new();
    for mints in RESERVE_MINTS.iter().permutations(2) {
        let (m1, m2) = (mints[0], mints[1]);
        let amm = GfxAmm::new(*m1, *m2).unwrap();
        accounts_to_update.extend(amm.get_accounts_to_update());
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

    println!("Iterating over all possible pairs, ordered both ways");
    for mints in RESERVE_MINTS.iter().permutations(2) {
        let (m1, m2) = (*mints[0], *mints[1]);
        println!("testing {} -> {}", m1, m2);

        let mut amm = GfxAmm::new(m1, m2).unwrap();

        // Initialize the account state
        let acts_to_update = amm.get_accounts_to_update();
        let acts: HashMap<Pubkey, Vec<u8>> = acts_to_update
            .iter()
            .map(|key| {
                (
                    *key,
                    acts_data.get(key).expect("Missing account data").clone(),
                )
            })
            .collect();

        amm.update(&acts).unwrap();

        match amm.quote(&QuoteParams {
            in_amount: 40000,
            input_mint: m1,
            output_mint: m2,
        }) {
            Ok(quote) => {
                println!("{}:", amm.label());
                println!("{:#?}\n", quote);
            }
            Err(e) => {
                println!("{}:", amm.label());
                println!("{e}\n");
            }
        }
    }
}
