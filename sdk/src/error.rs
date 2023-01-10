use solana_program::pubkey::Pubkey;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum GfxSdkError {
    #[error("Account not found: {0}")]
    AccountNotFound(Pubkey),
    #[error("Could not deserialize {0} as type: {1}")]
    DeserializeFailure(Pubkey, String),
}

pub type Result<T> = std::result::Result<T, GfxSdkError>;