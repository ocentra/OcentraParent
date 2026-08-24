use crate::types::{CorrelationId, Nonce, ProtocolError, ProtocolVersion};

use super::{AttestationDigest, BrokerHello, ClientHello, SessionHandle};

mod binding;
mod debug;

impl BrokerHello {
    pub fn try_from_untrusted_facts(
        client: &ClientHello,
        broker_nonce: Nonce,
        broker_epoch: u64,
        broker_key_epoch: u64,
        writer_lease_epoch: u64,
        watermark: u64,
        authority_generation: u64,
        target_generation: u64,
        key_generation: u64,
        writer_generation: u64,
        session_handle_bytes: &[u8],
        attestation_digest_bytes: &[u8],
    ) -> Result<Self, ProtocolError> {
        let session_handle = SessionHandle::try_from_bytes(session_handle_bytes)?;
        let attestation_digest = AttestationDigest::try_from_bytes(attestation_digest_bytes)?;
        Self::from_parts(
            client,
            broker_nonce,
            broker_epoch,
            broker_key_epoch,
            writer_lease_epoch,
            watermark,
            authority_generation,
            target_generation,
            key_generation,
            writer_generation,
            session_handle,
            attestation_digest,
        )
    }

    pub(crate) fn from_parts(
        client: &ClientHello,
        broker_nonce: Nonce,
        broker_epoch: u64,
        broker_key_epoch: u64,
        writer_lease_epoch: u64,
        watermark: u64,
        authority_generation: u64,
        target_generation: u64,
        key_generation: u64,
        writer_generation: u64,
        session_handle: SessionHandle,
        attestation_digest: AttestationDigest,
    ) -> Result<Self, ProtocolError> {
        if broker_epoch == 0
            || broker_key_epoch == 0
            || writer_lease_epoch == 0
            || authority_generation == 0
            || target_generation == 0
            || key_generation == 0
            || writer_generation == 0
        {
            return Err(ProtocolError::InvalidEpoch);
        }
        Ok(Self {
            version: client.version(),
            client_nonce: client.nonce(),
            broker_nonce,
            correlation: client.correlation(),
            client_process_epoch: client.client_process_epoch(),
            broker_epoch,
            broker_key_epoch,
            writer_lease_epoch,
            watermark,
            authority_generation,
            target_generation,
            key_generation,
            writer_generation,
            session_handle,
            attestation_digest,
        })
    }

    pub fn version(&self) -> ProtocolVersion {
        self.version
    }

    pub fn negotiated_version(&self) -> ProtocolVersion {
        self.version
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

    pub fn authority_generation(&self) -> u64 {
        self.authority_generation
    }

    pub fn target_generation(&self) -> u64 {
        self.target_generation
    }

    pub fn key_generation(&self) -> u64 {
        self.key_generation
    }

    pub fn writer_generation(&self) -> u64 {
        self.writer_generation
    }

    pub fn session_handle(&self) -> SessionHandle {
        self.session_handle
    }

    pub fn attestation_digest(&self) -> AttestationDigest {
        self.attestation_digest
    }
}
