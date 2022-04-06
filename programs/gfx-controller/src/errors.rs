use anchor_lang::prelude::*;
use std::convert::TryInto;

const ERROR_CODE_OFFSET: u32 = 6000;

// Define errors, custom error code: 6000 + idx => 0x1770 + 0x${idx}
#[error_code(offset = 6000)]
pub enum ErrorCode {
    #[msg("[G000] The pool is suspended")] //0x1770 (6000)
    Suspended,

    #[msg("[G001] Not admin")] //0x1771 (6001)
    NotAdmin,

    #[msg("[G002] Percentage out of range")] //0x1772 (6002)
    PercentageOutOfRange,

    #[msg("[G003] Not enough reward reserve")] //0x1773 (6003)
    NotEnoughRewardReserve,
}

impl TryInto<ErrorCode> for u32 {
    type Error = (); // Error if u32 is out of range

    fn try_into(self) -> std::result::Result<ErrorCode, ()> {
        if (ERROR_CODE_OFFSET..=ERROR_CODE_OFFSET + 3).contains(&self) {
            Ok(unsafe { std::mem::transmute(self - ERROR_CODE_OFFSET) })
        } else {
            Err(())
        }
    }
}
