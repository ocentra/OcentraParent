use std::fmt;

use crate::constants;

use super::super::super::ProtocolError;

pub(super) fn write(
    error: &ProtocolError,
    formatter: &mut fmt::Formatter<'_>,
) -> Option<fmt::Result> {
    match error {
        ProtocolError::InvalidSessionHandle => {
            Some(formatter.write_str(constants::ERROR_INVALID_SESSION_HANDLE))
        }
        ProtocolError::InvalidAttestationDigest => {
            Some(formatter.write_str(constants::ERROR_INVALID_ATTESTATION_DIGEST))
        }
        ProtocolError::InvalidTranscriptDigest => {
            Some(formatter.write_str(constants::ERROR_INVALID_TRANSCRIPT_DIGEST))
        }
        ProtocolError::InvalidAuthenticationTag => {
            Some(formatter.write_str(constants::ERROR_INVALID_AUTHENTICATION_TAG))
        }
        ProtocolError::AuthenticationFailed => {
            Some(formatter.write_str(constants::ERROR_AUTHENTICATION_FAILED))
        }
        ProtocolError::InvalidBootstrap => {
            Some(formatter.write_str(constants::ERROR_INVALID_BOOTSTRAP))
        }
        _ => None,
    }
}
