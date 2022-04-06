use anchor_lang::prelude::*;
use std::convert::TryInto;

const ERROR_CODE_OFFSET: u32 = 6100;

// Define errors, custom error code: 6100 + idx => 0x17D4 + 0x${idx}
#[error_code(offset = 6100)]
pub enum ErrorCode {
    #[msg("[G100] The pool is suspended")] //0x17D4 (6100)
    Suspended,

    #[msg("[G101] Not admin")] //0x17D5 (6101)
    NotAdmin,

    #[msg("[G102] Mints are not sorted")] //0x17D6 (6102)
    MintsNotSorted,

    #[msg("[G103] The risk token mint is wrong")] //0x17D7 (6103)
    WrongRTMint,

    #[msg("[G104] The required oracle is not present")] //0x17D8 (6104)
    OracleNotPresent,

    #[msg("[G105] The oracle is not in a healthy state (status)")] //0x17D9 (6105)
    OracleNotHealthyStatus,

    #[msg("[G106] The oracle is not in a healthy state (delay)")] //0x17DA (6106)
    OracleNotHealthyDelay,

    #[msg("[G107] The oracle is not in a healthy state (confidence)")] //0x17DB (6107)
    OracleNotHealthyConfidence,

    #[msg("[G108] SlippageTooLarge")] //0x17DC (6108)
    SlippageTooLarge,

    #[msg("[G109] Percentage out of range")] //0x17DD (6109)
    PercentageOutOfRange,

    #[msg("[G110] Swap instruction is not executed in order")] //0x17DE (6110)
    SwapIXNotInOrder,

    #[msg("[G111] Mint does not match the pair")] //0x17DF (6111)
    MintNotMatchPair,

    #[msg("[G112] Fee collector account incorrect")] //0x17E0 (6112)
    FeeCollectorIncorrect,
}

impl TryInto<ErrorCode> for u32 {
    type Error = (); // Error if u32 is out of range

    fn try_into(self) -> std::result::Result<ErrorCode, ()> {
        if (ERROR_CODE_OFFSET..=ERROR_CODE_OFFSET + 12).contains(&self) {
            Ok(unsafe { std::mem::transmute(self - ERROR_CODE_OFFSET) })
        } else {
            Err(())
        }
    }
}
