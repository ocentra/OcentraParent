use crate::types::{
    CorrelationId, Nonce, ProtocolGeneration, ProtocolVersion, SessionHandle,
    SessionTranscriptDigest,
};

use super::super::UntrustedRequest;

impl UntrustedRequest {
    pub fn version(&self) -> ProtocolVersion {
        self.session.version
    }

    pub fn protocol_generation(&self) -> ProtocolGeneration {
        self.session.protocol_generation
    }

    pub fn nonce(&self) -> Nonce {
        self.session.client_nonce
    }

    pub fn broker_nonce(&self) -> Nonce {
        self.session.broker_nonce
    }

    pub fn correlation(&self) -> CorrelationId {
        self.session.correlation
    }

    pub fn client_process_id(&self) -> u32 {
        self.session.client_process_id
    }

    pub fn client_process_epoch(&self) -> u64 {
        self.session.client_process_epoch
    }

    pub fn client_session_id(&self) -> u32 {
        self.session.client_session_id
    }

    pub fn broker_process_id(&self) -> u32 {
        self.session.broker_process_id
    }

    pub fn broker_session_id(&self) -> u32 {
        self.session.broker_session_id
    }

    pub fn broker_epoch(&self) -> u64 {
        self.session.broker_epoch
    }

    pub fn broker_key_epoch(&self) -> u64 {
        self.session.broker_key_epoch
    }

    pub fn writer_lease_epoch(&self) -> u64 {
        self.session.writer_lease_epoch
    }

    pub fn watermark(&self) -> u64 {
        self.session.watermark
    }

    pub fn session_handle(&self) -> SessionHandle {
        self.session.session_handle
    }

    pub fn transcript_digest(&self) -> SessionTranscriptDigest {
        self.session.transcript_digest
    }

    pub fn sequence(&self) -> u64 {
        self.session.sequence
    }

    pub fn expires_at_unix_millis(&self) -> u64 {
        self.session.expires_at_unix_millis
    }
}
