use solana_program::pubkey::Pubkey;
use gfx_ssl_interface::{Oracle, Pair};
use type_layout::TypeLayout;
use gfx_ssl_interface::svec::StackVec;

#[test]
fn pair_layout() {
    println!("{:#?}", Pair::type_layout());
    let mut pair = Pair {
        controller: Pubkey::new(&[98; 32]),
        mints: (Pubkey::new(&[1; 32]), Pubkey::new(&[1; 32])),
        bump: 2,
        _pad0: [100; 7],
        oracles: Default::default(),
        A: 4,
        fee_rate: (5, 5),
        _pad1: [101; 5],
        max_delay: 6,
        confidence: 7,
        _unused4: [8u8; 32],
        excessive_confiscate_rate: 9,
        fee_collector: Pubkey::new(&[10u8; 32]),
        platform_fee_rate: (11, 11),
        _unused3: [12u8; 18],
        volumes: (14,14),
        _unused0: [15u64; 10],
        enable_rebalance_swap: true,
        _pad3: [102; 7],
        referral_info: Default::default(),
        _pad6: [103; 4]
    };
    let pubkey_99s = Pubkey::new(&[99u8; 32]);
    let mut svec: StackVec<(Pubkey, bool), 4> = Default::default();
    svec.push((pubkey_99s, true));
    pair.oracles.push(Oracle {
        path: svec,
        padding: Default::default(),
    });
    let pair_bytes = bytemuck::bytes_of(&pair);
    println!("{:?}", pair_bytes);
    let index = pair_bytes.iter().enumerate().find(|(idx, v)| **v == 1).unwrap();
    println!("Mints starts at offset: {}", index.0);
    let index = pair_bytes.iter().enumerate().find(|(idx, v)| **v == 2).unwrap();
    println!("Bump starts at offset: {}", index.0);
    let index = pair_bytes.iter().enumerate().find(|(idx, v)| **v == 99).unwrap();
    println!("Oracles starts at offset: {}", index.0);
    let index = pair_bytes.iter().enumerate().find(|(idx, v)| **v == 4).unwrap();
    println!("A starts at offset: {}", index.0);
    let index = pair_bytes.iter().enumerate().find(|(idx, v)| **v == 5).unwrap();
    println!("Fee rate starts at offset: {}", index.0);
    let index = pair_bytes.iter().enumerate().find(|(idx, v)| **v == 6).unwrap();
    println!("Max delay starts at offset: {}", index.0);
}