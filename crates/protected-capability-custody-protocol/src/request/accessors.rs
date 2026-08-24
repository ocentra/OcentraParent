use crate::handshake::{AttestationDigest, SessionHandle};
use crate::types::{CorrelationId, Nonce, ProtocolVersion};

use super::Request;

impl Request {
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

    pub fn expected_authority_generation(&self) -> u64 {
        self.expected_authority_generation
    }

    pub fn expected_target_generation(&self) -> u64 {
        self.expected_target_generation
    }

    pub fn expected_key_generation(&self) -> u64 {
        self.expected_key_generation
    }

    pub fn expected_writer_generation(&self) -> u64 {
        self.expected_writer_generation
    }

    pub fn session_handle(&self) -> SessionHandle {
        self.session_handle
    }

    pub fn attestation_digest(&self) -> AttestationDigest {
        self.attestation_digest
    }
}
