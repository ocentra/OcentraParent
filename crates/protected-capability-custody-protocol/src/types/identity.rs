use std::fmt;

use crate::constants;

use super::{CorrelationId, Nonce, ProtocolError, SessionHandle};

impl Nonce {
    pub fn generate() -> Result<Self, ProtocolError> {
        let mut bytes = [0_u8; crate::constants::NONCE_BYTES];
        getrandom::fill(&mut bytes).map_err(ProtocolError::from_randomness)?;
        Self::try_from_bytes(&bytes)
    }

    pub fn try_from_bytes(value: &[u8]) -> Result<Self, ProtocolError> {
        let bytes = value.try_into().map_err(ProtocolError::from_nonce_length)?;
        if bytes == [0_u8; crate::constants::NONCE_BYTES] {
            return Err(ProtocolError::InvalidNonce);
        }
        Ok(Self(bytes))
    }

    pub fn as_bytes(&self) -> &[u8; crate::constants::NONCE_BYTES] {
        &self.0
    }
}

impl fmt::Debug for Nonce {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(constants::DEBUG_NONCE)
    }
}

impl CorrelationId {
    pub fn generate() -> Result<Self, ProtocolError> {
        let mut bytes = [0_u8; crate::constants::CORRELATION_BYTES];
        getrandom::fill(&mut bytes).map_err(ProtocolError::from_randomness)?;
        Self::try_from_bytes(&bytes)
    }

    pub fn try_from_bytes(value: &[u8]) -> Result<Self, ProtocolError> {
        let bytes = value
            .try_into()
            .map_err(ProtocolError::from_correlation_length)?;
        if bytes == [0_u8; crate::constants::CORRELATION_BYTES] {
            return Err(ProtocolError::InvalidCorrelationId);
        }
        Ok(Self(bytes))
    }

    pub fn as_bytes(&self) -> &[u8; crate::constants::CORRELATION_BYTES] {
        &self.0
    }
}

impl fmt::Debug for CorrelationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(constants::DEBUG_CORRELATION)
    }
}

impl SessionHandle {
    pub fn generate() -> Result<Self, ProtocolError> {
        let mut bytes = [0_u8; crate::constants::SESSION_HANDLE_BYTES];
        getrandom::fill(&mut bytes).map_err(ProtocolError::from_randomness)?;
        Self::try_from_untrusted_bytes(&bytes)
    }

    pub fn try_from_untrusted_bytes(value: &[u8]) -> Result<Self, ProtocolError> {
        let bytes = value
            .try_into()
            .map_err(ProtocolError::from_session_handle_length)?;
        if bytes == [0_u8; crate::constants::SESSION_HANDLE_BYTES] {
            return Err(ProtocolError::InvalidSessionHandle);
        }
        Ok(Self(bytes))
    }

    pub(crate) fn as_bytes(&self) -> &[u8; crate::constants::SESSION_HANDLE_BYTES] {
        &self.0
    }
}

impl fmt::Debug for SessionHandle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(constants::DEBUG_SESSION_HANDLE)
    }
}
