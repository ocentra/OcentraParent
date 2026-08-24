mod accessors;
mod debug;
mod generations;
mod status;

use crate::constants::REQUEST_DIGEST_BYTES;
use crate::handshake::{AttestationDigest, SessionHandle};
use crate::request::RequestKind;
use crate::types::{BindingEpochs, CorrelationId, Nonce, ProtocolError, ProtocolVersion};

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
    PrepareAmbiguous = 9,
}

pub struct UntrustedResponseFacts {
    pub nonce: Nonce,
    pub broker_nonce: Nonce,
    pub correlation: CorrelationId,
    pub client_process_epoch: u64,
    pub session_handle: SessionHandle,
    pub attestation_digest: AttestationDigest,
    pub request_kind: RequestKind,
    pub request_digest: [u8; REQUEST_DIGEST_BYTES],
    pub status: ResponseStatus,
    pub broker_epoch: u64,
    pub broker_key_epoch: u64,
    pub writer_lease_epoch: u64,
    pub watermark: u64,
    pub authority_generation: u64,
    pub target_generation: u64,
    pub key_generation: u64,
    pub writer_generation: u64,
    pub opaque_token: Vec<u8>,
}

#[derive(Eq, PartialEq)]
pub struct Response {
    pub(crate) version: ProtocolVersion,
    pub(crate) nonce: Nonce,
    pub(crate) broker_nonce: Nonce,
    pub(crate) correlation: CorrelationId,
    pub(crate) client_process_epoch: u64,
    pub(crate) session_handle: SessionHandle,
    pub(crate) attestation_digest: AttestationDigest,
    pub(crate) request_kind: RequestKind,
    pub(crate) request_digest: [u8; REQUEST_DIGEST_BYTES],
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
    pub fn try_from_untrusted_facts(facts: UntrustedResponseFacts) -> Result<Self, ProtocolError> {
        Self::from_parts(facts)
    }

    pub(crate) fn from_parts(facts: UntrustedResponseFacts) -> Result<Self, ProtocolError> {
        BindingEpochs {
            client_process_epoch: facts.client_process_epoch,
            broker_epoch: facts.broker_epoch,
            broker_key_epoch: facts.broker_key_epoch,
            writer_lease_epoch: facts.writer_lease_epoch,
            authority_generation: facts.authority_generation,
            target_generation: facts.target_generation,
            key_generation: facts.key_generation,
            writer_generation: facts.writer_generation,
        }
        .validate()?;
        status::validation::validate_result(&facts)?;
        Ok(Self {
            version: ProtocolVersion::CURRENT,
            nonce: facts.nonce,
            broker_nonce: facts.broker_nonce,
            correlation: facts.correlation,
            client_process_epoch: facts.client_process_epoch,
            session_handle: facts.session_handle,
            attestation_digest: facts.attestation_digest,
            request_kind: facts.request_kind,
            request_digest: facts.request_digest,
            status: facts.status,
            broker_epoch: facts.broker_epoch,
            broker_key_epoch: facts.broker_key_epoch,
            writer_lease_epoch: facts.writer_lease_epoch,
            watermark: facts.watermark,
            authority_generation: facts.authority_generation,
            target_generation: facts.target_generation,
            key_generation: facts.key_generation,
            writer_generation: facts.writer_generation,
            opaque_token: facts.opaque_token,
        })
    }
}
