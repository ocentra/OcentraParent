mod accessors;
mod debug;
mod status;

use crate::handshake::{AttestationDigest, SessionHandle};
use crate::types::{CorrelationId, Nonce, ProtocolError, ProtocolVersion};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum ResponseStatus {
    Prepared = 1,
    Committed = 2,
    Aborted = 3,
    CommitAmbiguous = 4,
    AbortAmbiguous = 5,
    Rejected = 6,
    Unavailable = 7,
    UnsupportedPlatform = 8,
}

#[derive(Eq, PartialEq)]
pub struct Response {
    pub(crate) version: ProtocolVersion,
    pub(crate) nonce: Nonce,
    pub(crate) correlation: CorrelationId,
    pub(crate) client_process_epoch: u64,
    pub(crate) session_handle: SessionHandle,
    pub(crate) attestation_digest: AttestationDigest,
    pub(crate) status: ResponseStatus,
    pub(crate) broker_epoch: u64,
    pub(crate) broker_key_epoch: u64,
    pub(crate) writer_lease_epoch: u64,
    pub(crate) watermark: u64,
    pub(crate) authority_generation: u64,
    pub(crate) target_generation: u64,
    pub(crate) key_generation: u64,
    pub(crate) writer_generation: u64,
    pub(crate) opaque_token: Vec<u8>,
}

impl Response {
    pub fn try_from_untrusted_facts(
        nonce: Nonce,
        correlation: CorrelationId,
        client_process_epoch: u64,
        session_handle: SessionHandle,
        attestation_digest: AttestationDigest,
        status: ResponseStatus,
        broker_epoch: u64,
        broker_key_epoch: u64,
        writer_lease_epoch: u64,
        watermark: u64,
        authority_generation: u64,
        target_generation: u64,
        key_generation: u64,
        writer_generation: u64,
        opaque_token: Vec<u8>,
    ) -> Result<Self, ProtocolError> {
        Self::from_parts(
            nonce,
            correlation,
            client_process_epoch,
            session_handle,
            attestation_digest,
            status,
            broker_epoch,
            broker_key_epoch,
            writer_lease_epoch,
            watermark,
            authority_generation,
            target_generation,
            key_generation,
            writer_generation,
            opaque_token,
        )
    }

    pub(crate) fn from_parts(
        nonce: Nonce,
        correlation: CorrelationId,
        client_process_epoch: u64,
        session_handle: SessionHandle,
        attestation_digest: AttestationDigest,
        status: ResponseStatus,
        broker_epoch: u64,
        broker_key_epoch: u64,
        writer_lease_epoch: u64,
        watermark: u64,
        authority_generation: u64,
        target_generation: u64,
        key_generation: u64,
        writer_generation: u64,
        opaque_token: Vec<u8>,
    ) -> Result<Self, ProtocolError> {
        status::validation::validate_result(
            status,
            client_process_epoch,
            broker_epoch,
            broker_key_epoch,
            writer_lease_epoch,
            authority_generation,
            target_generation,
            key_generation,
            writer_generation,
            &opaque_token,
        )?;
        Ok(Self {
            version: ProtocolVersion::CURRENT,
            nonce,
            correlation,
            client_process_epoch,
            session_handle,
            attestation_digest,
            status,
            broker_epoch,
            broker_key_epoch,
            writer_lease_epoch,
            watermark,
            authority_generation,
            target_generation,
            key_generation,
            writer_generation,
            opaque_token,
        })
    }
}
