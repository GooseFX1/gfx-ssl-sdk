use std::collections::HashMap;
use std::hint::black_box;
use jupiter_core::amm::{Amm, QuoteParams};
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use gfx_ssl_sdk::jupiter::{GfxAmm, RESERVE_MINTS};

#[test]
fn main() {
    let client = RpcClient::new("https://api.mainnet-beta.solana.com");

    RESERVE_MINTS.iter().zip(RESERVE_MINTS.iter())
        .filter(|(mint1, mint2)| mint1 != mint2)
        .for_each(|(mint1, mint2)| {
            let mut amm = GfxAmm::new(*mint1, *mint2)
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

            let quote = amm.quote(black_box(&QuoteParams {
                in_amount: 40000,
                input_mint: *mint1,
                output_mint: *mint2,
            })).unwrap();
            println!("{:?}", quote);
        });

}
