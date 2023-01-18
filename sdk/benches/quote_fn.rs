use std::collections::HashMap;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jupiter_core::amm::{Amm, QuoteParams};
use gfx_ssl_sdk_rust::jupiter::GfxAmm;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::pubkey;

const BTC_MINT: Pubkey = pubkey!("9n4nbM75f5Ui33ZbPYXn59EwSgE8CGsHtAeTH5YFeJ9E");
const USDC_MINT: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");

pub fn criterion_benchmark(c: &mut Criterion) {
    let client = RpcClient::new("https://api.mainnet-beta.solana.com");

    let mut amm = GfxAmm::new(BTC_MINT, USDC_MINT)
        .unwrap();

    // We need to run the update function twice, because the addresses of
    // some necessary accounts are stored as account data.
    for _ in 0..2 {
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
    }

    c.bench_function("quote_fn", |b| b.iter(|| {
        amm.quote(black_box(&QuoteParams {
            in_amount: 40000,
            input_mint: BTC_MINT,
            output_mint: USDC_MINT,
        })).unwrap();
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);