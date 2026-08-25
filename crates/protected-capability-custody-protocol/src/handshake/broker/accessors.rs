use crate::types::{
    AttestationDigest, CorrelationId, Nonce, ProtocolGeneration, ProtocolVersion, SessionHandle,
};

use super::super::UntrustedBrokerHello;

impl UntrustedBrokerHello {
    pub fn version(&self) -> ProtocolVersion {
        self.version
    }

    pub fn protocol_generation(&self) -> ProtocolGeneration {
        self.protocol_generation
    }

    pub fn client_nonce(&self) -> Nonce {
        self.client_nonce
    }

    pub fn broker_nonce(&self) -> Nonce {
        self.broker_nonce
    }

    pub fn correlation(&self) -> CorrelationId {
        self.correlation
    }

    pub fn client_process_id(&self) -> u32 {
        self.client_process_id
    }

    pub fn client_process_epoch(&self) -> u64 {
        self.client_process_epoch
    }

    pub fn broker_process_id(&self) -> u32 {
        self.broker_process_id
    }

    pub fn client_session_id(&self) -> u32 {
        self.client_session_id
    }

    pub fn broker_session_id(&self) -> u32 {
        self.broker_session_id
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

    pub fn session_handle(&self) -> SessionHandle {
        self.session_handle
    }

    pub fn attestation_digest(&self) -> AttestationDigest {
        self.attestation_digest
    }

    pub fn session_expires_at_unix_millis(&self) -> u64 {
        self.session_expires_at_unix_millis
    }
}
