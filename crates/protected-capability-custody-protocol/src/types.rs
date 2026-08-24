use std::fmt;

use thiserror::Error;

use crate::constants::{CORRELATION_BYTES, NONCE_BYTES, PROTOCOL_VERSION};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProtocolVersion(pub(crate) u16);

impl ProtocolVersion {
    pub const CURRENT: Self = Self(PROTOCOL_VERSION);

    pub fn current() -> Self {
        Self::CURRENT
    }

    pub fn value(self) -> u16 {
        self.0
    }

    pub(crate) fn decode(value: u16) -> Result<Self, ProtocolError> {
        if value != PROTOCOL_VERSION {
            return Err(ProtocolError::UnsupportedVersion(value));
        }
        Ok(Self(value))
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Nonce(pub(crate) [u8; NONCE_BYTES]);

impl Nonce {
    pub fn try_from_bytes(value: &[u8]) -> Result<Self, ProtocolError> {
        let bytes = value.try_into().map_err(|_| ProtocolError::InvalidNonce)?;
        if bytes == [0_u8; NONCE_BYTES] {
            return Err(ProtocolError::InvalidNonce);
        }
        Ok(Self(bytes))
    }

    pub fn as_bytes(&self) -> &[u8; NONCE_BYTES] {
        &self.0
    }
}

impl fmt::Debug for Nonce {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Nonce(<redacted>)")
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct CorrelationId(pub(crate) [u8; CORRELATION_BYTES]);

impl CorrelationId {
    pub fn try_from_bytes(value: &[u8]) -> Result<Self, ProtocolError> {
        let bytes = value
            .try_into()
            .map_err(|_| ProtocolError::InvalidCorrelationId)?;
        if bytes == [0_u8; CORRELATION_BYTES] {
            return Err(ProtocolError::InvalidCorrelationId);
        }
        Ok(Self(bytes))
    }

    pub fn as_bytes(&self) -> &[u8; CORRELATION_BYTES] {
        &self.0
    }
}

impl fmt::Debug for CorrelationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("CorrelationId(<redacted>)")
    }
}

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("frame is empty")]
    EmptyFrame,
    #[error("frame exceeds the bounded protocol limit")]
    FrameTooLarge,
    #[error("frame length prefix is invalid")]
    InvalidFrameLength,
    #[error("frame has trailing bytes")]
    TrailingBytes,
    #[error("protocol version {0} is unsupported")]
    UnsupportedVersion(u16),
    #[error("protocol domain separator is invalid")]
    InvalidDomain,
    #[error("protocol message kind is invalid: {0}")]
    InvalidMessageKind(u8),
    #[error("nonce is invalid")]
    InvalidNonce,
    #[error("correlation id is invalid")]
    InvalidCorrelationId,
    #[error("field is empty")]
    EmptyField,
    #[error("field exceeds the bounded protocol limit")]
    FieldTooLarge,
    #[error("request operation is unsupported: {0}")]
    UnsupportedRequest(u8),
    #[error("action is unsupported: {0}")]
    UnsupportedAction(u8),
    #[error("target is unsupported: {0}")]
    UnsupportedTarget(u8),
    #[error("response status is unsupported: {0}")]
    UnsupportedStatus(u8),
    #[error("opaque token is invalid")]
    InvalidOpaqueToken,
    #[error("opaque token was supplied for a request that does not accept one")]
    UnexpectedOpaqueToken,
    #[error("epoch is invalid")]
    InvalidEpoch,
    #[error("authenticated session handle is invalid")]
    InvalidSessionHandle,
    #[error("broker attestation digest is invalid")]
    InvalidAttestationDigest,
    #[error("payload is truncated")]
    Truncated,
}
