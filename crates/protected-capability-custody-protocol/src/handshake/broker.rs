use crate::types::{BindingEpochs, CorrelationId, Nonce, ProtocolError, ProtocolVersion};

use super::{
    AttestationDigest, BrokerHello, BrokerHelloParts, ClientHello, SessionHandle,
    UntrustedBrokerFacts,
};

mod binding;
mod debug;

impl BrokerHello {
    pub fn try_from_untrusted_facts(
        client: &ClientHello,
        facts: &UntrustedBrokerFacts,
    ) -> Result<Self, ProtocolError> {
        let session_handle = SessionHandle::try_from_bytes(&facts.session_handle_bytes)?;
        let attestation_digest =
            AttestationDigest::try_from_bytes(&facts.attestation_digest_bytes)?;
        Self::from_parts(
            client,
            &BrokerHelloParts {
                broker_nonce: facts.broker_nonce,
                broker_epoch: facts.broker_epoch,
                broker_key_epoch: facts.broker_key_epoch,
                writer_lease_epoch: facts.writer_lease_epoch,
                watermark: facts.watermark,
                authority_generation: facts.authority_generation,
                target_generation: facts.target_generation,
                key_generation: facts.key_generation,
                writer_generation: facts.writer_generation,
                session_handle,
                attestation_digest,
            },
        )
    }

    pub(crate) fn from_parts(
        client: &ClientHello,
        parts: &BrokerHelloParts,
    ) -> Result<Self, ProtocolError> {
        BindingEpochs {
            client_process_epoch: client.client_process_epoch(),
            broker_epoch: parts.broker_epoch,
            broker_key_epoch: parts.broker_key_epoch,
            writer_lease_epoch: parts.writer_lease_epoch,
            authority_generation: parts.authority_generation,
            target_generation: parts.target_generation,
            key_generation: parts.key_generation,
            writer_generation: parts.writer_generation,
        }
        .validate()?;
        Ok(Self {
            version: client.version(),
            client_nonce: client.nonce(),
            broker_nonce: parts.broker_nonce,
            correlation: client.correlation(),
            client_process_epoch: client.client_process_epoch(),
            broker_epoch: parts.broker_epoch,
            broker_key_epoch: parts.broker_key_epoch,
            writer_lease_epoch: parts.writer_lease_epoch,
            watermark: parts.watermark,
            authority_generation: parts.authority_generation,
            target_generation: parts.target_generation,
            key_generation: parts.key_generation,
            writer_generation: parts.writer_generation,
            session_handle: parts.session_handle,
            attestation_digest: parts.attestation_digest,
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
