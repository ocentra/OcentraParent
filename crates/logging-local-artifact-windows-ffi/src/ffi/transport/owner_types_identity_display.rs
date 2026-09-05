use std::fmt;
use std::fmt::Write as _;

use super::FileIdentity;
use crate::constants::{HEX_DIGITS, IDENTITY_PREFIX};

impl fmt::Display for FileIdentity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(IDENTITY_PREFIX)?;
        fmt::Display::fmt(&self.volume_serial_number, formatter)?;
        for byte in self.file_id {
            formatter.write_char(HEX_DIGITS[usize::from(byte >> 4)] as char)?;
            formatter.write_char(HEX_DIGITS[usize::from(byte & 0x0f)] as char)?;
        }
        Ok(())
    }
}
