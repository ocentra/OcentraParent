use std::fmt;

use crate::constants;

use super::super::ProtocolError;

pub(super) fn write(
    error: &ProtocolError,
    formatter: &mut fmt::Formatter<'_>,
) -> Option<fmt::Result> {
    match error {
        ProtocolError::InvalidOpaqueToken => {
            Some(formatter.write_str(constants::ERROR_INVALID_OPAQUE_TOKEN))
        }
        ProtocolError::UnexpectedOpaqueToken => Some(write_unexpected_token(formatter)),
        ProtocolError::InvalidEpoch => Some(formatter.write_str(constants::ERROR_INVALID_EPOCH)),
        ProtocolError::InvalidSessionHandle => {
            Some(formatter.write_str(constants::ERROR_INVALID_SESSION_HANDLE))
        }
        ProtocolError::InvalidAttestationDigest => {
            Some(formatter.write_str(constants::ERROR_INVALID_ATTESTATION_DIGEST))
        }
        ProtocolError::Truncated => Some(formatter.write_str(constants::ERROR_TRUNCATED)),
        ProtocolError::InvalidStatusForRequest => {
            Some(formatter.write_str(constants::ERROR_INVALID_STATUS_FOR_REQUEST))
        }
        _ => None,
    }
}

fn write_unexpected_token(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(constants::ERROR_UNEXPECTED_OPAQUE_TOKEN_PREFIX)?;
    formatter.write_str(constants::ERROR_UNEXPECTED_OPAQUE_TOKEN_SUFFIX)
}
