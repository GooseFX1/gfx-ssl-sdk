use std::io::Write;
use std::ops::Deref;
use crate::utils::PDAIdentifier;
use type_layout::TypeLayout;
use anchor_lang::prelude::*;
use crate::svec::StackVec;

impl PDAIdentifier for Pair {
    const IDENT: &'static [u8] = b"GFX-SSL-Pair";

    #[inline(always)]
    fn program_id() -> &'static Pubkey {
        &crate::ID
    }
}

// /// Contains a list of oracle addresses, ordered to match
// /// the input/output mints of a swap
// #[account]
// #[derive(Copy, Default, TypeLayout)]
// #[cfg_attr(feature = "no-entrypoint", derive(Debug))]
// #[repr(C, align(8))]
// pub struct Oracle {
//     pub path: StackVec4<(Pubkey, bool)>,
//     pub padding: [u64; 8],
// }
//
// impl Oracle {
//     pub fn new() -> Oracle {
//         Oracle {
//             path: StackVec4::new(),
//             padding: [0; 8],
//         }
//     }
//
//     pub fn with_path(path: &[(Pubkey, bool)]) -> Self {
//         assert!(path.len() <= 4);
//         let mut oracle = Self::new();
//
//         for e in path {
//             assert!(oracle.path.push(*e).is_none());
//         }
//
//         oracle
//     }
// }
//
//
// #[allow(non_snake_case)]
// #[account]
// #[cfg_attr(feature = "no-entrypoint", derive(Debug))]
// #[derive(Default, Copy, TypeLayout)]
// #[repr(packed)]
// pub struct Pair {
//     pub controller: Pubkey, // for indexing purpose
//     pub mints: (Pubkey, Pubkey),
//     pub bump: u8,
//     pub _pad0: [u8; 7],
//     pub oracles: StackVec5<Oracle>,
//     // configs
//     pub A: u8,              // parameter A for poorman's curve
//     pub fee_rate: (u8, u8), // in BP
//     pub _pad1: [u8; 5],
//     pub max_delay: u64,
//     pub confidence: u64,
//     pub _unused4: [u8; 32],
//     pub excessive_confiscate_rate: u16, // the percentage to confiscate if a trade gets better price than the oracle price
//
//     pub fee_collector: Pubkey, // the pubkey for the platform fee collector
//     pub platform_fee_rate: (u16, u16),
//
//     pub _unused3: [u8; 18],
//     pub _unused5: [u8; 4],
//     pub surpluses: (u64, u64), // surpluses.0 is the surplus of mints.0, which is owned by the ssl of mints.1
//     pub volumes: (u128, u128),
//
//     pub _unused0: [u64; 10],
//     pub enable_rebalance_swap: bool,
//     pub _pad3: [u8; 7],
//     pub _pad5: [u64; 11],
//     pub _pad6: [u64; 4],
// }

#[derive(Copy, Clone, Default, Debug, TypeLayout)]
#[repr(C, align(8))]
pub struct Oracle {
    pub path: StackVec<(Pubkey, bool), 4>,
    pub padding: [u64; 8],
}

impl Oracle {
    pub fn new() -> Oracle {
        Oracle {
            path: StackVec::new(),
            padding: [0; 8],
        }
    }

    pub fn with_path(path: &[(Pubkey, bool)]) -> Self {
        assert!(path.len() <= 4);
        let mut oracle = Self::new();

        for e in path {
            assert!(oracle.path.push(*e).is_none());
        }

        oracle
    }
}

impl Deref for Oracle {
    type Target = [(Pubkey, bool)];
    fn deref(&self) -> &Self::Target {
        &*self.path
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Copy, Clone, Default, Debug, TypeLayout)]
#[repr(C)]
pub struct Referrer {
    pub key: Pubkey,
    pub rate_bp: u16,
}

// The address of a PairInfo is
// [Pair::IDENT, controller.key().as_ref(), base.key().as_ref(), quote.key().as_ref()]
// where base and quote should be sorted based on their pubkey
#[allow(non_snake_case)]
#[account(zero_copy)]
#[derive(Default, Debug, TypeLayout)]
pub struct Pair {
    pub controller: Pubkey, // for indexing purpose
    pub mints: (Pubkey, Pubkey),
    pub bump: u8,
    _pad0: [u8; 7],
    pub oracles: StackVec<Oracle, 5>,
    // configs
    pub A: u8,              // parameter A for poorman's curve
    pub fee_rate: (u8, u8), // in BP
    _pad1: [u8; 5],
    pub max_delay: u64,
    pub confidence: u64,
    pub _unused4: [u8; 32],
    pub excessive_confiscate_rate: u16, // the percentage to confiscate if a trade gets better price than the oracle price

    pub fee_collector: Pubkey, // the pubkey for the platform fee collector
    pub platform_fee_rate: (u16, u16),

    pub _unused3: [u8; 18],
    pub volumes: (u128, u128),

    _unused0: [u64; 10],
    pub enable_rebalance_swap: bool,
    _pad3: [u8; 7],
    pub referral_info: StackVec<Referrer, 3>,
    _pad6: [u64; 4],
}

impl AccountSerialize for Pair {
    fn try_serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(bytemuck::bytes_of(self))?;
        Ok(())
    }
}


const _: [u8; 1528] = [0; std::mem::size_of::<Pair>()];
