use solana_sdk::pubkey::Pubkey;
use gfx_ssl_wasm::Pair;
use type_layout::TypeLayout;

#[test]
fn pair_layout_wasm() {
    println!("{:#?}", Pair::type_layout());
    let pair = Pair {
        controller: Pubkey::new(&[0u8; 32]),
        mints: (Pubkey::new(&[1; 32]), Pubkey::new(&[1; 32])),
        bump: 2,
        oracles: Default::default(),
        A: 4,
        fee_rate: (5, 5),
        max_delay: 6,
        confidence: 7,
        _unused4: [8u8; 32],
        excessive_confiscate_rate: 9,
        fee_collector: Pubkey::new(&[10u8; 32]),
        platform_fee_rate: (11, 11),
        _unused3: [12u8; 2],
        surpluses: (13, 13),
        volumes: (14,14),
        _unused0: [15; 32],
        _unused1: [16; 32],
        _unused2: [17; 16],
        enable_rebalance_swap: true,
        _pad: [19; 7],
        _pad2: [20; 18]
    };
    let pair_bytes = bytemuck::bytes_of(&pair);
    println!("{:?}", pair_bytes);
    // println!("{:?}", StackVec::<Oracle, 4>::type_layout());
}
