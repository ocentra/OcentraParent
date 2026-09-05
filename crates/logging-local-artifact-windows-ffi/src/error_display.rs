use std::fmt;

use super::{ArtifactError, ErrorCode};

const INVALID_PATH_PREFIX: &str = "invalid local-artifact path: ";
const UNSUPPORTED_OPERATION_PREFIX: &str = "local-artifact operation is unsupported: ";
const IO_FAILURE_PREFIX: &str = "local-artifact I/O failure: ";
const DURABILITY_FAILURE_PREFIX: &str = "local-artifact durability failure: ";
const OPERATION_FAILURE_PREFIX: &str = "local-artifact operation failed: ";

impl fmt::Display for ErrorCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0)
    }
}

impl fmt::Display for ArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPath(message) => {
                formatter.write_str(INVALID_PATH_PREFIX)?;
                formatter.write_str(message)
            }
            Self::UnsupportedOperation(operation) => {
                formatter.write_str(UNSUPPORTED_OPERATION_PREFIX)?;
                formatter.write_str(operation)
            }
            Self::Io(message) => {
                formatter.write_str(IO_FAILURE_PREFIX)?;
                formatter.write_str(message)
            }
            Self::DurabilityFailureWith(message) => {
                formatter.write_str(DURABILITY_FAILURE_PREFIX)?;
                formatter.write_str(message)
            }
            other => {
                formatter.write_str(OPERATION_FAILURE_PREFIX)?;
                fmt::Display::fmt(&other.code(), formatter)
            }
        }
    }
}

impl std::error::Error for ArtifactError {}
