use sha2::{Digest, Sha256};

use crate::constants::SESSION_TRANSCRIPT_DOMAIN;
use crate::types::{
    AttestationDigest, AuthenticationDomain, AuthenticationTag, BootstrapAuthenticator,
    ProtocolError, SessionTranscriptDigest,
};

use super::{BrokerSessionWireValues, UntrustedBrokerHello, UntrustedClientHello};

mod accessors;
mod binding;
mod debug;

impl UntrustedBrokerHello {
    pub fn authenticate_wire(
        client: &UntrustedClientHello,
        session: BrokerSessionWireValues,
        now_unix_millis: u64,
    ) -> Result<Self, ProtocolError> {
        let session = session.try_new(now_unix_millis)?;
        let authenticator = BootstrapAuthenticator::generate()?;
        let mut canonical = Vec::with_capacity(224);
        session.append_attestation_message(client, &mut canonical);
        let tag =
            authenticator.authenticate(AuthenticationDomain::BrokerAttestation, &canonical)?;
        Ok(Self::from_parts(
            client,
            session,
            AttestationDigest::from_authentication_tag(tag),
            authenticator,
        ))
    }

    pub(crate) fn from_untrusted_wire(
        client: &UntrustedClientHello,
        session: BrokerSessionWireValues,
        attestation_digest: AttestationDigest,
        authenticator: BootstrapAuthenticator,
        now_unix_millis: u64,
    ) -> Result<Self, ProtocolError> {
        Ok(Self::from_parts(
            client,
            session.try_new(now_unix_millis)?,
            attestation_digest,
            authenticator,
        ))
    }

    fn from_parts(
        client: &UntrustedClientHello,
        session: BrokerSessionWireValues,
        attestation_digest: AttestationDigest,
        authenticator: BootstrapAuthenticator,
    ) -> Self {
        Self {
            version: client.version(),
            protocol_generation: client.protocol_generation(),
            client_nonce: client.nonce(),
            broker_nonce: session.broker_nonce,
            correlation: client.correlation(),
            client_process_id: client.client_process_id(),
            client_process_epoch: client.client_process_epoch(),
            client_session_id: client.client_session_id(),
            broker_process_id: session.broker_process_id,
            broker_session_id: session.broker_session_id,
            broker_epoch: session.broker_epoch,
            broker_key_epoch: session.broker_key_epoch,
            writer_lease_epoch: session.writer_lease_epoch,
            watermark: session.watermark,
            session_handle: session.session_handle,
            attestation_digest,
            authenticator,
            session_expires_at_unix_millis: session.session_expires_at_unix_millis,
        }
    }

    pub fn verify_authenticated_provenance(
        &self,
        client: &UntrustedClientHello,
        now_unix_millis: u64,
    ) -> Result<SessionTranscriptDigest, ProtocolError> {
        if !self.matches_client(client) || !self.is_live_at(now_unix_millis) {
            return Err(ProtocolError::AuthenticationFailed);
        }
        let session = self.session_wire_values();
        let mut canonical = Vec::with_capacity(224);
        session.append_attestation_message(client, &mut canonical);
        self.authenticator.verify(
            AuthenticationDomain::BrokerAttestation,
            &canonical,
            AuthenticationTag::from_attestation_digest(self.attestation_digest),
        )?;
        Ok(self.transcript_digest())
    }

    /// Clones only the broker-generated session key used to authenticate the
    /// one authenticated pipe session. The bootstrap packet never carries or
    /// chooses this secret.
    pub fn clone_authenticator(&self) -> BootstrapAuthenticator {
        self.authenticator.clone()
    }

    pub fn authenticator(&self) -> &BootstrapAuthenticator {
        &self.authenticator
    }

    pub fn transcript_digest(&self) -> SessionTranscriptDigest {
        let mut canonical = Vec::with_capacity(256);
        self.session_wire_values()
            .append_attestation_message(&self.client_hello(), &mut canonical);
        canonical.extend_from_slice(self.attestation_digest.as_bytes());
        let mut digest = Sha256::new();
        digest.update((SESSION_TRANSCRIPT_DOMAIN.len() as u32).to_be_bytes());
        digest.update(SESSION_TRANSCRIPT_DOMAIN.as_bytes());
        digest.update((canonical.len() as u32).to_be_bytes());
        digest.update(&canonical);
        SessionTranscriptDigest::from_digest(digest.finalize().into())
    }

    pub fn is_live_at(&self, now_unix_millis: u64) -> bool {
        now_unix_millis != 0 && now_unix_millis < self.session_expires_at_unix_millis
    }

    pub(crate) fn session_wire_values(&self) -> BrokerSessionWireValues {
        BrokerSessionWireValues {
            broker_nonce: self.broker_nonce,
            broker_process_id: self.broker_process_id,
            broker_session_id: self.broker_session_id,
            broker_epoch: self.broker_epoch,
            broker_key_epoch: self.broker_key_epoch,
            writer_lease_epoch: self.writer_lease_epoch,
            watermark: self.watermark,
            session_handle: self.session_handle,
            session_expires_at_unix_millis: self.session_expires_at_unix_millis,
        }
    }

    fn client_hello(&self) -> UntrustedClientHello {
        UntrustedClientHello {
            version: self.version,
            protocol_generation: self.protocol_generation,
            nonce: self.client_nonce,
            correlation: self.correlation,
            client_process_id: self.client_process_id,
            client_process_epoch: self.client_process_epoch,
            client_session_id: self.client_session_id,
        }
    }
}
