use solana_program::pubkey::Pubkey;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum GfxSdkError {
    #[error("Account not found: {0}")]
    AccountNotFound(Pubkey),

    #[error("Could not deserialize {0} as type: {1}")]
    DeserializeFailure(Pubkey, String),

    #[error("The mint {0} is not supported")]
    UnsupportedMint(Pubkey),

    #[error("Invalid account size for {0}, expect: {1}, got: {2}")]
    InvalidAccountSize(Pubkey, usize, usize),

    #[error("Some required accounts has not updated")]
    RequiredAccountNoUpdate,

    #[error("The AMM does not support provided mints")]
    UnexpectedMints,
}

pub type Result<T> = std::result::Result<T, GfxSdkError>;
