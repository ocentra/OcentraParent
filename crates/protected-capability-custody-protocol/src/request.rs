use crate::constants::OPAQUE_TOKEN_BYTES;
use crate::handshake::{AttestationDigest, SessionHandle};
use crate::target::{Action, TargetDescriptor};
use crate::types::{CorrelationId, Nonce, ProtocolError, ProtocolVersion};

mod accessors;
mod binding;
mod debug;
mod operations;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum RequestKind {
    Prepare = 1,
    Commit = 2,
    Abort = 3,
    Recover = 4,
    ResolveAmbiguity = 5,
}

impl RequestKind {
    pub(crate) fn decode(value: u8) -> Result<Self, ProtocolError> {
        match value {
            1 => Ok(Self::Prepare),
            2 => Ok(Self::Commit),
            3 => Ok(Self::Abort),
            4 => Ok(Self::Recover),
            5 => Ok(Self::ResolveAmbiguity),
            other => Err(ProtocolError::UnsupportedRequest(other)),
        }
    }

    pub(crate) fn requires_token(self) -> bool {
        matches!(self, Self::Commit | Self::Abort)
    }
}

#[derive(Eq, PartialEq)]
pub struct Request {
    pub(crate) version: ProtocolVersion,
    pub(crate) nonce: Nonce,
    pub(crate) correlation: CorrelationId,
    pub(crate) client_process_epoch: u64,
    pub(crate) broker_epoch: u64,
    pub(crate) broker_key_epoch: u64,
    pub(crate) writer_lease_epoch: u64,
    pub(crate) watermark: u64,
    pub(crate) expected_authority_generation: u64,
    pub(crate) expected_target_generation: u64,
    pub(crate) expected_key_generation: u64,
    pub(crate) expected_writer_generation: u64,
    pub(crate) session_handle: SessionHandle,
    pub(crate) attestation_digest: AttestationDigest,
    pub(crate) kind: RequestKind,
    pub(crate) operation: Vec<u8>,
    pub(crate) action: Action,
    pub(crate) target: TargetDescriptor,
    pub(crate) opaque_token: Vec<u8>,
}

impl Request {
    pub fn try_from_untrusted_wire_values(
        nonce: Nonce,
        correlation: CorrelationId,
        client_process_epoch: u64,
        broker_epoch: u64,
        broker_key_epoch: u64,
        writer_lease_epoch: u64,
        watermark: u64,
        expected_authority_generation: u64,
        expected_target_generation: u64,
        expected_key_generation: u64,
        expected_writer_generation: u64,
        session_handle: SessionHandle,
        attestation_digest: AttestationDigest,
        kind: RequestKind,
        operation: Vec<u8>,
        action: Action,
        target: TargetDescriptor,
        opaque_token: Vec<u8>,
    ) -> Result<Self, ProtocolError> {
        validate_epochs(
            client_process_epoch,
            broker_epoch,
            broker_key_epoch,
            writer_lease_epoch,
            expected_authority_generation,
            expected_target_generation,
            expected_key_generation,
            expected_writer_generation,
        )?;
        crate::target::validation::validate_field(&operation)?;
        if kind.requires_token() && opaque_token.len() != OPAQUE_TOKEN_BYTES {
            return Err(ProtocolError::InvalidOpaqueToken);
        }
        if !kind.requires_token() && !opaque_token.is_empty() {
            return Err(ProtocolError::UnexpectedOpaqueToken);
        }
        Ok(Self {
            version: ProtocolVersion::CURRENT,
            nonce,
            correlation,
            client_process_epoch,
            broker_epoch,
            broker_key_epoch,
            writer_lease_epoch,
            watermark,
            expected_authority_generation,
            expected_target_generation,
            expected_key_generation,
            expected_writer_generation,
            session_handle,
            attestation_digest,
            kind,
            operation,
            action,
            target,
            opaque_token,
        })
    }
}

fn validate_epochs(
    client_process_epoch: u64,
    broker_epoch: u64,
    broker_key_epoch: u64,
    writer_lease_epoch: u64,
    expected_authority_generation: u64,
    expected_target_generation: u64,
    expected_key_generation: u64,
    expected_writer_generation: u64,
) -> Result<(), ProtocolError> {
    if client_process_epoch == 0
        || broker_epoch == 0
        || broker_key_epoch == 0
        || writer_lease_epoch == 0
        || expected_authority_generation == 0
        || expected_target_generation == 0
        || expected_key_generation == 0
        || expected_writer_generation == 0
    {
        return Err(ProtocolError::InvalidEpoch);
    }
    Ok(())
}
