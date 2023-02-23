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
#[cfg_attr(feature = "type-layout", derive(TypeLayout))]
#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
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

impl anchor_lang::Discriminator for Pair {
    const DISCRIMINATOR: [u8; 8] = [85, 72, 49, 176, 182, 228, 141, 82];
}
impl AccountDeserialize for Pair {
    fn try_deserialize(buf: &mut &[u8]) -> Result<Self> {
        if buf.len() < [85, 72, 49, 176, 182, 228, 141, 82].len() {
            return Err(ErrorCode::AccountDiscriminatorNotFound.into());
        }
        let given_disc = &buf[..8];
        if &[85, 72, 49, 176, 182, 228, 141, 82] != given_disc {
            return Err(
                anchor_lang::error::Error::from(AnchorError {
                    error_name: ErrorCode::AccountDiscriminatorMismatch
                        .name(),
                    error_code_number:
                        anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch.into(),
                    error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                        .to_string(),
                    error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                        anchor_lang::error::Source {
                            filename: "programs/gfx-ssl/src/states/pair.rs",
                            line: 64u32,
                        },
                    )),
                    compared_values: None,
                })
                .with_account_name("Pair"),
            );
        }
        Self::try_deserialize_unchecked(buf)
    }
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> Result<Self> {
        let data: &[u8] = &buf[8..];
        let account = bytemuck::from_bytes(data);
        Ok(*account)
    }
}
impl Owner for Pair {
    fn owner() -> Pubkey {
        crate::ID
    }
}

unsafe impl bytemuck::Pod for Pair {}
unsafe impl bytemuck::Zeroable for Pair {}


impl AccountSerialize for Pair {
    fn try_serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(bytemuck::bytes_of(self))?;
        Ok(())
    }
}


const _: [u8; 1528] = [0; std::mem::size_of::<Pair>()];
