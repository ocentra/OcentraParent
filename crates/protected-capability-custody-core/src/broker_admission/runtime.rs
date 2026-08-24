use std::path::Path;
use std::sync::Arc;

use ocentra_protected_capability_custody_protocol::request::{
    authenticated::AuthenticatedRequest, RequestKind,
};

use super::{
    authority, error_status, finalize, platform, prepare, recover, wire, BrokerCustodyOutcome,
    BrokerCustodyRuntime, BrokerPeerTokenIdentity, BrokerPlatformSessionState,
    BrokerProcessAdmission, BrokerRuntimeError,
};

impl BrokerProcessAdmission {
    pub fn for_current_process() -> Result<Self, BrokerRuntimeError> {
        let executable = super::BrokerExecutableGuard::open_current_broker()?;
        let database = super::storage_path::open_fixed_database()?;
        Ok(Self {
            _executable: executable,
            database,
        })
    }
}

impl BrokerCustodyRuntime {
    /// Opens the neutral custody runtime for the fixed database selected by
    /// the isolated broker process. This is not caller-selected authority.
    pub fn open_broker_owned(
        admission: BrokerProcessAdmission,
    ) -> Result<Self, BrokerRuntimeError> {
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

    /// The safe named-pipe dependency provides impersonation RAII but does not
    /// expose the impersonated token's SID, integrity level, and session as a
    /// single pinned observation. Refuse admission until the dedicated
    /// Windows token adapter supplies all three from that token. In
    /// particular, do not substitute same-user or stream-session metadata.
    pub fn observe_impersonated_client() -> Result<BrokerPeerTokenIdentity, BrokerRuntimeError> {
        Err(BrokerRuntimeError::DeploymentRequired)
    }

    /// The broker must not advertise a running listener until the exact
    /// Windows process/token adapter is linked. This remains an explicit
    /// deployment boundary rather than silently accepting a weaker identity.
    pub fn peer_admission_available() -> Result<(), BrokerRuntimeError> {
        Err(BrokerRuntimeError::DeploymentRequired)
    }

    /// Authorizes the one OS-observed pipe peer against the broker's
    /// SYSTEM-only enrollment key. The caller cannot mint the token identity;
    /// it can only pass the opaque observation returned above.
    pub fn authorize_client_peer(
        &self,
        process_id: u32,
        executable_path: &Path,
        executable_digest: [u8; 32],
        token: BrokerPeerTokenIdentity,
    ) -> Result<(), BrokerRuntimeError> {
        #[cfg(windows)]
        {
            platform::admission::authorize_client_peer(
                &self.registry_id,
                process_id,
                executable_path,
                executable_digest,
                &token.token_sid,
                token.integrity_level,
                token.session_id,
            )
            .map_err(error_status::platform)
        }
        #[cfg(not(windows))]
        {
            let _ = (process_id, executable_path, executable_digest, token);
            Err(BrokerRuntimeError::Unavailable)
        }
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
