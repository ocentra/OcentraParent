use std::sync::Arc;

use ocentra_protected_capability_custody_protocol::request::{
    authenticated::AuthenticatedRequest, RequestKind,
};
use ocentra_protected_capability_custody_protocol::{
    bootstrap::BootstrapPacket, handshake::UntrustedClientHello,
};

use super::{
    authority, error_status, finalize, platform, prepare, recover, wire,
    BrokerAuthorizedClientTranscript, BrokerCustodyOutcome, BrokerCustodyRuntime,
    BrokerPeerAdmissionObservation, BrokerPlatformSessionState, BrokerProcessAdmission,
    BrokerRuntimeError,
};

impl BrokerProcessAdmission {
    fn for_current_process() -> Result<Self, BrokerRuntimeError> {
        let executable = super::BrokerExecutableGuard::open_current_broker()?;
        let database = super::storage_path::open_fixed_database()?;
        Ok(Self {
            _executable: executable,
            database,
        })
    }
}

impl BrokerCustodyRuntime {
    /// Proves that every sealed Windows adapter required for service startup
    /// is linked before broker admission can select or create any storage.
    /// This preflight is capability-only and performs no filesystem, registry,
    /// listener, or durable-state operation.
    pub fn preflight_service_start() -> Result<(), BrokerRuntimeError> {
        Self::peer_admission_available()?;
        platform::preflight_service_start().map_err(error_status::platform)
    }

    /// Starts custody through the one cross-crate broker-owner seam. The
    /// capability-only preflight is deliberately the first operation, before
    /// executable admission can select or create the fixed storage path.
    pub fn start_broker_owned() -> Result<Self, BrokerRuntimeError> {
        Self::preflight_service_start()?;
        let admission = BrokerProcessAdmission::for_current_process()?;
        Self::open_broker_owned(admission)
    }

    /// Opens the neutral custody runtime for the fixed database selected by
    /// the isolated broker process. This is not caller-selected authority.
    fn open_broker_owned(admission: BrokerProcessAdmission) -> Result<Self, BrokerRuntimeError> {
        let BrokerProcessAdmission {
            _executable,
            database,
        } = admission;
        let registry_id =
            platform::registry_id(database.canonical()).map_err(error_status::platform)?;
        let authority = Arc::new(authority::BrokerCurrentBindingAuthority::new(
            registry_id.clone(),
        ));
        let platform_owner = Arc::new(platform::BrokerPlatformOwner::new());
        let store = crate::custody::CustodyStore::open_pending(
            database,
            platform_owner,
            authority.clone(),
        )?;
        Ok(Self {
            store,
            authority,
            registry_id,
            _executable,
        })
    }

    /// Observes one exact named-pipe peer while the caller holds the pipe's
    /// RAII impersonation guard. The future adapter must open and retain one
    /// Windows process handle, derive PID/creation epoch/image/digest from that
    /// handle, revalidate liveness, read SID/integrity/session from the
    /// impersonated token, and bind both PID and session to the pipe values.
    /// No sysinfo, path-only, same-user, or split-snapshot substitute is valid.
    pub fn observe_impersonated_named_pipe_client(
        pipe_process_id: u32,
        pipe_session_id: u32,
    ) -> Result<BrokerPeerAdmissionObservation, BrokerRuntimeError> {
        if pipe_process_id == 0 || pipe_session_id == 0 {
            return Err(BrokerRuntimeError::InvalidRequest);
        }
        Err(BrokerRuntimeError::DeploymentRequired)
    }

    /// The broker must not advertise a running listener until the exact
    /// Windows process/token adapter is linked. This remains an explicit
    /// deployment boundary rather than silently accepting a weaker identity.
    pub fn peer_admission_available() -> Result<(), BrokerRuntimeError> {
        Err(BrokerRuntimeError::DeploymentRequired)
    }

    /// Authorizes the one pinned observation returned by the missing platform
    /// adapter. This remains unavailable until the adapter can keep its process
    /// handle live through enrollment comparison and immediately revalidate it
    /// before the broker emits `BrokerHello`.
    pub fn authorize_client_peer(
        &self,
        observation: &BrokerPeerAdmissionObservation,
    ) -> Result<(), BrokerRuntimeError> {
        let _ = (self, observation);
        Err(BrokerRuntimeError::DeploymentRequired)
    }

    /// Revalidates the retained process handle and binds its PID, creation
    /// epoch, token SID/integrity/session, and pipe PID/session to this exact
    /// bootstrap/client-hello nonce and transcript immediately before the
    /// broker releases session key material. This separate sealed result keeps
    /// a future adapter from authenticating once, dropping its handle, and
    /// then trusting a reusable PID or caller-asserted epoch.
    pub fn authorize_client_transcript(
        &self,
        observation: &BrokerPeerAdmissionObservation,
        bootstrap: &BootstrapPacket,
        hello: &UntrustedClientHello,
    ) -> Result<BrokerAuthorizedClientTranscript, BrokerRuntimeError> {
        let _ = (self, observation, bootstrap, hello);
        Err(BrokerRuntimeError::DeploymentRequired)
    }

    /// Builds the listener ACL from the installer-owned enrollment record.
    /// Missing or malformed enrollment keeps the broker unavailable; the
    /// service must never fall back to a broad/default same-user ACL.
    #[cfg(windows)]
    pub fn broker_pipe_sddl(&self) -> Result<String, BrokerRuntimeError> {
        platform::admission::broker_pipe_sddl(&self.registry_id).map_err(error_status::platform)
    }

    #[cfg(not(windows))]
    pub fn broker_pipe_sddl(&self) -> Result<String, BrokerRuntimeError> {
        Err(BrokerRuntimeError::Unavailable)
    }

    /// Executes a request only after the broker validates kernel peer
    /// identity and the authenticated, expiring, one-shot transcript.
    pub fn execute_authenticated_request(
        &self,
        request: &AuthenticatedRequest,
    ) -> Result<BrokerCustodyOutcome, BrokerRuntimeError> {
        let request = request.as_untrusted();
        let locator = wire::validated_locator(request)?;
        let current = self
            .authority
            .resolve_for_request(
                locator.clone(),
                request.kind(),
                request.expected_generations(),
            )
            .map_err(error_status::authority)?;
        let generations = wire::observed(current)?;
        match request.kind() {
            RequestKind::Prepare => prepare::prepare(self, &locator, generations),
            RequestKind::Commit => finalize::finalize(
                self,
                request,
                locator,
                generations,
                crate::custody::Decision::Commit,
            ),
            RequestKind::Abort => finalize::finalize(
                self,
                request,
                locator,
                generations,
                crate::custody::Decision::Abort,
            ),
            RequestKind::Recover => recover::recover(self, &locator, generations, false),
            RequestKind::ResolveAmbiguity => recover::recover(self, &locator, generations, true),
        }
    }

    pub fn platform_session_state(&self) -> Result<BrokerPlatformSessionState, BrokerRuntimeError> {
        let (key_epoch, writer_lease_epoch, watermark) = self.store.broker_session_epochs()?;
        Ok(BrokerPlatformSessionState {
            key_epoch,
            writer_lease_epoch,
            watermark,
        })
    }
}
