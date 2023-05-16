use crate::svec::StackVec;
use crate::utils::PDAIdentifier;
use anchor_lang::prelude::*;
use bytemuck::{Pod, Zeroable};
use std::io::Write;
use std::ops::Deref;

impl PDAIdentifier for Pair {
    const IDENT: &'static [u8] = b"GFX-SSL-Pair";

    #[inline(always)]
    fn program_id() -> &'static Pubkey {
        &crate::ID
    }
}

#[zero_copy]
#[derive(Default, Debug)]
pub struct OracleComponent {
    pub key: Pubkey,
    pub inversed: u8,
}

impl OracleComponent {
    pub fn key(&self) -> &Pubkey {
        &self.key
    }

    pub fn inversed(&self) -> bool {
        self.inversed != 0
    }
}

impl From<(Pubkey, bool)> for OracleComponent {
    fn from(other: (Pubkey, bool)) -> Self {
        Self {
            key: other.0,
            inversed: other.1 as u8,
        }
    }
}

impl From<OracleComponent> for (Pubkey, bool) {
    fn from(other: OracleComponent) -> Self {
        (other.key, other.inversed != 0)
    }
}

#[zero_copy]
#[derive(Default, Debug)]
pub struct Oracle {
    pub path: StackVec<OracleComponent, 4>,
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
            assert!(oracle.path.push(OracleComponent::from(*e)).is_none());
        }

        oracle
    }
}

impl Deref for Oracle {
    type Target = [OracleComponent];
    fn deref(&self) -> &Self::Target {
        &*self.path
    }
}

#[zero_copy]
#[derive(Default, Debug)]
pub struct Referrer {
    pub key: Pubkey,
    pub rate_bp: u16,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct Tuple<A> {
    pub first: A,
    pub second: A,
}

unsafe impl<A> Zeroable for Tuple<A> where A: Pod + Zeroable {}

unsafe impl<A> Pod for Tuple<A> where A: Pod + Zeroable {}

// The address of a PairInfo is
// [Pair::IDENT, controller.key().as_ref(), base.key().as_ref(), quote.key().as_ref()]
// where base and quote should be sorted based on their pubkey
#[allow(non_snake_case)]
#[account(zero_copy)]
#[derive(Default, Debug)]
pub struct Pair {
    pub controller: Pubkey, // for indexing purpose
    pub mints: Tuple<Pubkey>,
    pub bump: u8,
    _pad0: [u8; 7],
    pub oracles: StackVec<Oracle, 5>,
    // configs
    pub A: u8,               // parameter A for poorman's curve
    pub fee_rate: Tuple<u8>, // in BP
    _pad1: [u8; 5],
    pub max_delay: u64,
    pub confidence: u64,
    pub _unused4: [u8; 32],
    pub excessive_confiscate_rate: u16, // the percentage to confiscate if a trade gets better price than the oracle price

    pub fee_collector: Pubkey, // the pubkey for the platform fee collector
    pub platform_fee_rate: Tuple<u16>,

    pub _unused3: [u8; 18],
    // volumes below is a (u128, u128)
    pub volumes: [u64; 4],

    _unused0: [u64; 10],
    pub enable_rebalance_swap: u8,
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
