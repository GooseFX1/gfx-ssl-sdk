use std::collections::HashMap;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jupiter_core::amm::{Amm, QuoteParams};
use gfx_ssl_sdk::jupiter::GfxAmm;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::pubkey;

const ETH_MINT: Pubkey = pubkey!("7vfCXTUXx5WJV5JADk17DUJ4ksgau7utNKj4b963voxs");
const USDC_MINT: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");

pub fn criterion_benchmark(c: &mut Criterion) {
    let client = RpcClient::new("https://api.mainnet-beta.solana.com");

    let mut amm = GfxAmm::new(ETH_MINT, USDC_MINT)
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

    c.bench_function("quote_fn", |b| b.iter(|| {
        amm.quote(black_box(&QuoteParams {
            in_amount: 40000,
            input_mint: ETH_MINT,
            output_mint: USDC_MINT,
        })).unwrap();
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);