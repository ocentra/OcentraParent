use std::fmt;

use crate::constants::{self, ATTESTATION_DIGEST_BYTES, SESSION_HANDLE_BYTES};
use crate::types::ProtocolError;

use super::{AttestationDigest, SessionHandle};

impl SessionHandle {
    pub(crate) fn try_from_bytes(value: &[u8]) -> Result<Self, ProtocolError> {
        let bytes = value
            .try_into()
            .map_err(|_error| ProtocolError::InvalidSessionHandle)?;
        if bytes == [0_u8; SESSION_HANDLE_BYTES] {
            return Err(ProtocolError::InvalidSessionHandle);
        }
        Ok(Self(bytes))
    }

    pub fn as_bytes(&self) -> &[u8; SESSION_HANDLE_BYTES] {
        &self.0
    }
}

impl fmt::Debug for SessionHandle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(constants::DEBUG_SESSION_HANDLE)
    }
}

impl AttestationDigest {
    pub(crate) fn try_from_bytes(value: &[u8]) -> Result<Self, ProtocolError> {
        let bytes = value
            .try_into()
            .map_err(|_error| ProtocolError::InvalidAttestationDigest)?;
        if bytes == [0_u8; ATTESTATION_DIGEST_BYTES] {
            return Err(ProtocolError::InvalidAttestationDigest);
        }
        Ok(Self(bytes))
    }

    pub fn as_bytes(&self) -> &[u8; ATTESTATION_DIGEST_BYTES] {
        &self.0
    }
}

impl fmt::Debug for AttestationDigest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(constants::DEBUG_ATTESTATION_DIGEST)
    }
}
