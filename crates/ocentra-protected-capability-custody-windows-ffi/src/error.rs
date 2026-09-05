//! Error formatting for the raw Windows/TPM boundary.

use super::{fmt, Error};

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, formatter)
    }
}

impl std::error::Error for Error {}

impl From<core::num::TryFromIntError> for Error {
    fn from(_error: core::num::TryFromIntError) -> Self {
        Self::BufferTooLarge
    }
}
