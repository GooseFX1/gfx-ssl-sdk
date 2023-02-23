use std::io::Write;
use std::ops::Deref;
use crate::utils::PDAIdentifier;
use anchor_lang::prelude::*;
use crate::svec::StackVec;
#[cfg(feature = "type-layout")]
use type_layout::TypeLayout;

impl PDAIdentifier for Pair {
    const IDENT: &'static [u8] = b"GFX-SSL-Pair";

    #[inline(always)]
    fn program_id() -> &'static Pubkey {
        &crate::ID
    }
}

#[derive(Copy, Clone, Default, Debug)]
#[cfg_attr(feature = "type-layout", derive(TypeLayout))]
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

#[derive(AnchorDeserialize, AnchorSerialize, Copy, Clone, Default, Debug)]
#[cfg_attr(feature = "type-layout", derive(TypeLayout))]
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
#[cfg_attr(feature = "type-layout", derive(TypeLayout))]
#[derive(Default, Debug)]
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


//const _: [u8; 1528] = [0; std::mem::size_of::<Pair>()];
