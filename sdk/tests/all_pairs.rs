use std::collections::HashMap;
use std::hint::black_box;
use jupiter_core::amm::{Amm, QuoteParams};
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use gfx_ssl_sdk::jupiter::{GfxAmm, RESERVE_MINTS};

#[test]
fn all_pairs() {
    let client = RpcClient::new("https://api.mainnet-beta.solana.com");

    println!("Iterating over all possible pairs, ordered both ways");
    for mint1 in RESERVE_MINTS.iter() {
        for mint2 in RESERVE_MINTS.iter() {
            if *mint1 != *mint2 {
                for (m1, m2) in [
                    (*mint1, *mint2),
                    (*mint2, *mint1),
                ] {
                    let mut amm = GfxAmm::new(m1, m2)
                        .unwrap();

                    // Initialize the account state
                    let acts_to_update = amm.get_accounts_to_update();
                    let acts_data = client.get_multiple_accounts(&acts_to_update)
                        .unwrap()
                        .into_iter()
                        .map(|act| {
                            act.unwrap().data
                        })
                        .collect::<Vec<_>>();
                    let acts: HashMap<Pubkey, Vec<u8>> = acts_to_update
                        .into_iter()
                        .zip(acts_data)
                        .collect();
                    amm.update(&acts).unwrap();

                    match amm.quote(black_box(&QuoteParams {
                        in_amount: 40000,
                        input_mint: m1,
                        output_mint: m2,
                    })) {
                        Ok(quote) => {
                            //println!("{}:", amm.label());
                            //println!("{:#?}\n", quote);
                        },
                        Err(e) => {
                            println!("{}:", amm.label());
                            println!("{e}\n");
                        }
                    }
                }
            }
        }
    }
}
