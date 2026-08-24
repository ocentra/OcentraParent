use std::fmt;

use crate::constants;
use crate::constants::{CORRELATION_BYTES, NONCE_BYTES, PROTOCOL_VERSION};

mod display;

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
        let bytes = value
            .try_into()
            .map_err(|_error| ProtocolError::InvalidNonce)?;
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
        formatter.write_str(constants::DEBUG_NONCE)
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct CorrelationId(pub(crate) [u8; CORRELATION_BYTES]);

impl CorrelationId {
    pub fn try_from_bytes(value: &[u8]) -> Result<Self, ProtocolError> {
        let bytes = value
            .try_into()
            .map_err(|_error| ProtocolError::InvalidCorrelationId)?;
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
        formatter.write_str(constants::DEBUG_CORRELATION)
    }
}

pub(crate) struct BindingEpochs {
    pub(crate) client_process_epoch: u64,
    pub(crate) broker_epoch: u64,
    pub(crate) broker_key_epoch: u64,
    pub(crate) writer_lease_epoch: u64,
    pub(crate) authority_generation: u64,
    pub(crate) target_generation: u64,
    pub(crate) key_generation: u64,
    pub(crate) writer_generation: u64,
}

impl BindingEpochs {
    pub(crate) fn validate(&self) -> Result<(), ProtocolError> {
        if self.client_process_epoch == 0
            || self.broker_epoch == 0
            || self.broker_key_epoch == 0
            || self.writer_lease_epoch == 0
            || self.authority_generation == 0
            || self.target_generation == 0
            || self.key_generation == 0
            || self.writer_generation == 0
        {
            return Err(ProtocolError::InvalidEpoch);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum ProtocolError {
    EmptyFrame,
    FrameTooLarge,
    InvalidFrameLength,
    TrailingBytes,
    UnsupportedVersion(u16),
    InvalidDomain,
    InvalidMessageKind(u8),
    InvalidNonce,
    InvalidCorrelationId,
    EmptyField,
    FieldTooLarge,
    UnsupportedRequest(u8),
    UnsupportedAction(u8),
    UnsupportedTarget(u8),
    UnsupportedStatus(u8),
    InvalidOpaqueToken,
    UnexpectedOpaqueToken,
    InvalidEpoch,
    InvalidSessionHandle,
    InvalidAttestationDigest,
    Truncated,
    InvalidStatusForRequest,
}
