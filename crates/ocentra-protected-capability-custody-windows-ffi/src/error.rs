//! Error formatting for the raw Windows/TPM boundary.

use super::{fmt, Error};

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedPlatform => {
                formatter.write_str("Windows ABI unavailable on this target")
            }
            Self::Win32(code) => write!(formatter, "Windows API failed with error {code}"),
            Self::Tpm(code) => write!(formatter, "TBS/TPM API failed with status {code:#x}"),
            Self::BufferTooLarge => {
                formatter.write_str("Windows ABI buffer exceeds the bounded limit")
            }
            Self::InvalidInput(message) => formatter.write_str(message),
            Self::MalformedTpm => formatter.write_str("malformed TPM response or command"),
        }
    }
}

impl std::error::Error for Error {}
