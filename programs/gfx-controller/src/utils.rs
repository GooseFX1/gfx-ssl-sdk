use anchor_lang::prelude::*;

pub static BP_DECIMAL: u32 = 4;

pub trait PDAIdentifier {
    const IDENT: &'static [u8];

    fn program_id() -> &'static Pubkey;

    fn get_address(seeds: &[&[u8]]) -> Pubkey {
        Self::get_address_with_bump(seeds).0
    }

    fn get_address_with_bump(seeds: &[&[u8]]) -> (Pubkey, u8) {
        // TODO: avoid heap allocation
        let mut seeds = seeds.to_vec();
        seeds.insert(0, Self::IDENT);
        Pubkey::find_program_address(&seeds, Self::program_id())
    }
}
