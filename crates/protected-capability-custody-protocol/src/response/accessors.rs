use crate::handshake::{AttestationDigest, SessionHandle};
use crate::request::Request;
use crate::types::{CorrelationId, Nonce, ProtocolVersion};

use super::{Response, ResponseStatus};

impl Response {
    pub fn is_bound_to(&self, request: &Request) -> bool {
        self.version == request.version()
            && self.nonce == request.nonce()
            && self.broker_nonce == request.broker_nonce()
            && self.correlation == request.correlation()
            && self.client_process_epoch == request.client_process_epoch()
            && self.session_handle == request.session_handle()
            && self.attestation_digest == request.attestation_digest()
            && self.request_kind == request.kind()
            && self.request_digest == request.request_digest()
            && self.status.is_compatible_with(request.kind())
            && self.broker_epoch == request.broker_epoch()
            && self.broker_key_epoch == request.broker_key_epoch()
            && self.writer_lease_epoch == request.writer_lease_epoch()
            && self.watermark == request.watermark()
            && self.authority_generation == request.expected_authority_generation()
            && self.target_generation == request.expected_target_generation()
            && self.key_generation == request.expected_key_generation()
            && self.writer_generation == request.expected_writer_generation()
    }

    pub fn version(&self) -> ProtocolVersion {
        self.version
    }

    pub fn nonce(&self) -> Nonce {
        self.nonce
    }

    pub fn broker_nonce(&self) -> Nonce {
        self.broker_nonce
    }

    pub fn correlation(&self) -> CorrelationId {
        self.correlation
    }

    pub fn client_process_epoch(&self) -> u64 {
        self.client_process_epoch
    }

    pub fn session_handle(&self) -> SessionHandle {
        self.session_handle
    }

    pub fn attestation_digest(&self) -> AttestationDigest {
        self.attestation_digest
    }

    pub fn status(&self) -> ResponseStatus {
        self.status
    }

    pub fn request_kind(&self) -> crate::request::RequestKind {
        self.request_kind
    }

    pub fn request_digest(&self) -> &[u8; crate::constants::REQUEST_DIGEST_BYTES] {
        &self.request_digest
    }

    pub fn broker_epoch(&self) -> u64 {
        self.broker_epoch
    }

    pub fn broker_key_epoch(&self) -> u64 {
        self.broker_key_epoch
    }

    pub fn writer_lease_epoch(&self) -> u64 {
        self.writer_lease_epoch
    }

    pub fn watermark(&self) -> u64 {
        self.watermark
    }

    pub fn opaque_token(&self) -> &[u8] {
        &self.opaque_token
    }
}
