use sha2::{Digest, Sha256};

use crate::platform::PlatformError;

mod client_anchor;
mod enrollment;
mod enrollment_record;
mod enrollment_security;
mod monotonic;
mod peer;
mod scm;
mod service_sid;
mod token_groups;

pub(in crate::broker_admission) struct WindowsCustodyRuntime {
    enrollment: enrollment::VerifiedEnrollment,
    broker_process: peer::VerifiedBrokerProcess,
    service: scm::VerifiedBrokerService,
}

pub(in crate::broker_admission) struct RetainedPeer(peer::PeerObservation);

pub(in crate::broker_admission) struct AuthorizedPeer {
    _peer: peer::AuthorizedPeer,
}

pub(super) type BrokerClientAnchor = client_anchor::ClientAnchor;

impl WindowsCustodyRuntime {
    pub(in crate::broker_admission) fn open(registry_id: &str) -> Result<Self, PlatformError> {
        let enrollment = enrollment::VerifiedEnrollment::open(registry_id)?;
        let broker_process = peer::VerifiedBrokerProcess::open(&enrollment)?;
        let service = scm::VerifiedBrokerService::open(&enrollment)?;
        monotonic::preflight(&enrollment)?;
        Ok(Self {
            enrollment,
            broker_process,
            service,
        })
    }

    pub(in crate::broker_admission) fn revalidate_broker(&self) -> Result<(), PlatformError> {
        self.enrollment.revalidate()?;
        self.broker_process.revalidate(&self.enrollment)?;
        self.service.revalidate(&self.enrollment)
    }

    pub(in crate::broker_admission) fn broker_process_identity(
        &self,
    ) -> Result<(u32, u64, u32), PlatformError> {
        self.revalidate_broker()?;
        self.broker_process.observed_identity()
    }

    pub(in crate::broker_admission) fn pipe_sddl(&self) -> Result<String, PlatformError> {
        self.revalidate_broker()?;
        Ok(format!(
            "D:P(A;;GRGW;;;SY)(A;;GRGW;;;{})",
            self.enrollment.client_sid_sddl()
        ))
    }

    pub(in crate::broker_admission) fn observe_peer(
        &self,
        pipe_process_id: u32,
        pipe_session_id: u32,
    ) -> Result<RetainedPeer, PlatformError> {
        self.revalidate_broker()?;
        peer::PeerObservation::observe(pipe_process_id, pipe_session_id).map(RetainedPeer)
    }

    pub(in crate::broker_admission) fn authorize_peer(
        &self,
        peer: &RetainedPeer,
    ) -> Result<(), PlatformError> {
        self.revalidate_broker()?;
        peer.0.revalidate(&self.enrollment)
    }

    pub(in crate::broker_admission) fn authorize_transcript(
        &self,
        peer: RetainedPeer,
        bootstrap: &ocentra_protected_capability_custody_protocol::bootstrap::BootstrapPacket,
        hello: &ocentra_protected_capability_custody_protocol::handshake::UntrustedClientHello,
        pipe_process_id: u32,
        pipe_session_id: u32,
    ) -> Result<AuthorizedPeer, PlatformError> {
        self.revalidate_broker()?;
        peer.0
            .authorize_transcript(
                &self.enrollment,
                bootstrap,
                hello,
                pipe_process_id,
                pipe_session_id,
            )
            .map(|peer| AuthorizedPeer { _peer: peer })
    }
}

pub(super) fn preflight(registry_id: &str) -> Result<(), PlatformError> {
    WindowsCustodyRuntime::open(registry_id).map(|_runtime| ())
}

pub(super) struct ObservationDigest(Sha256);

impl ObservationDigest {
    pub(super) fn new(domain: &[u8]) -> Self {
        let mut digest = Sha256::new();
        digest.update((domain.len() as u32).to_be_bytes());
        digest.update(domain);
        Self(digest)
    }

    pub(super) fn field(&mut self, value: &[u8]) {
        self.0.update((value.len() as u32).to_be_bytes());
        self.0.update(value);
    }

    pub(super) fn text(&mut self, value: &str) {
        self.field(value.as_bytes());
    }

    pub(super) fn u32(&mut self, value: u32) {
        self.field(&value.to_be_bytes());
    }

    pub(super) fn u64(&mut self, value: u64) {
        self.field(&value.to_be_bytes());
    }

    pub(super) fn boolean(&mut self, value: bool) {
        self.field(&[u8::from(value)]);
    }

    pub(super) fn finish(self) -> [u8; 32] {
        let output = self.0.finalize();
        let mut digest = [0_u8; 32];
        digest.copy_from_slice(&output);
        digest
    }
}

pub(super) fn map_ffi_error(
    error: ocentra_protected_capability_custody_windows_ffi::Error,
) -> PlatformError {
    use ocentra_protected_capability_custody_windows_ffi::Error;

    match error {
        Error::UnsupportedPlatform | Error::Win32(2 | 3) => PlatformError::DeploymentRequired,
        Error::InvalidInput(_)
        | Error::MalformedTpm
        | Error::BufferTooLarge
        | Error::CryptoPropertyViolation => PlatformError::Tampered,
        Error::Win32(_) | Error::Tpm(_) | Error::Tbs(_) | Error::Crypto(_) => {
            PlatformError::Unavailable
        }
    }
}
