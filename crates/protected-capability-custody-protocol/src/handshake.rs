mod broker;
mod client;
mod session;

use crate::constants::{ATTESTATION_DIGEST_BYTES, SESSION_HANDLE_BYTES};
use crate::types::{CorrelationId, Nonce, ProtocolVersion};

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct SessionHandle(pub(crate) [u8; SESSION_HANDLE_BYTES]);

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct AttestationDigest(pub(crate) [u8; ATTESTATION_DIGEST_BYTES]);

#[derive(Eq, PartialEq)]
pub struct ClientHello {
    pub(crate) version: ProtocolVersion,
    pub(crate) nonce: Nonce,
    pub(crate) correlation: CorrelationId,
    pub(crate) client_process_epoch: u64,
}

#[derive(Eq, PartialEq)]
pub struct BrokerHello {
    pub(crate) version: ProtocolVersion,
    pub(crate) client_nonce: Nonce,
    pub(crate) broker_nonce: Nonce,
    pub(crate) correlation: CorrelationId,
    pub(crate) client_process_epoch: u64,
    pub(crate) broker_epoch: u64,
    pub(crate) broker_key_epoch: u64,
    pub(crate) writer_lease_epoch: u64,
    pub(crate) watermark: u64,
    pub(crate) authority_generation: u64,
    pub(crate) target_generation: u64,
    pub(crate) key_generation: u64,
    pub(crate) writer_generation: u64,
    pub(crate) session_handle: SessionHandle,
    pub(crate) attestation_digest: AttestationDigest,
}
