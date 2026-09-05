use std::sync::Arc;

use ocentra_protected_capability_custody_protocol::request::{
    authenticated::AuthenticatedRequest, RequestKind,
};
use ocentra_protected_capability_custody_protocol::{
    bootstrap::BootstrapPacket, handshake::UntrustedClientHello,
};

use super::account_issuer_request::ProtectedAccountIssuerRequestAdmission;
use super::{
    authority, error_status, finalize, platform, prepare, recover, wire,
    BrokerAuthorizedClientTranscript, BrokerCustodyOutcome, BrokerCustodyRuntime,
    BrokerPeerAdmissionObservation, BrokerPlatformSessionState, BrokerProcessAdmission,
    BrokerRuntimeError,
};
use ocentra_protected_capability_custody_protocol::account_issuer_session::AuthenticatedAccountIssuerRequest;

#[cfg(windows)]
use super::BrokerProcessIdentity;

impl BrokerProcessAdmission {
    #[cfg(windows)]
    fn for_current_process() -> Result<Self, BrokerRuntimeError> {
        let database_path = super::storage_path::fixed_database_identity_path()?;
        let registry_id = platform::registry_id(&database_path)
            .map_err(|error| error_status::platform(&error))?;
        // Retain every enrollment, broker-process, SCM, and TPM observation
        // before the fixed database path can be opened or created.
        let windows = platform::BrokerWindowsRuntime::open(&registry_id)
            .map_err(|error| error_status::platform(&error))?;
        let executable = super::BrokerExecutableGuard::open_current_broker(&windows)?;
        windows
            .revalidate_broker()
            .map_err(|error| error_status::platform(&error))?;
        let database = super::storage_path::open_fixed_database()?;
        windows
            .revalidate_broker()
            .map_err(|error| error_status::platform(&error))?;
        Ok(Self {
            _executable: executable,
            database,
            registry_id,
            windows,
        })
    }

    #[cfg(not(windows))]
    fn for_current_process() -> Result<Self, BrokerRuntimeError> {
        Err(BrokerRuntimeError::Unavailable)
    }
}

impl BrokerCustodyRuntime {
    /// Proves that every sealed Windows adapter required for service startup
    /// is linked before broker admission can select or create any storage.
    /// It performs read-only process, registry, SCM, and TPM observations but
    /// no storage, registry-state, journal, listener, or readiness mutation.
    pub fn preflight_service_start() -> Result<(), BrokerRuntimeError> {
        let database_path = super::storage_path::fixed_database_identity_path()?;
        let registry_id = platform::registry_id(&database_path)
            .map_err(|error| error_status::platform(&error))?;
        platform::preflight_service_start(&registry_id)
            .map_err(|error| error_status::platform(&error))
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
            registry_id,
            #[cfg(windows)]
            windows,
        } = admission;
        if platform::registry_id(database.canonical())
            .map_err(|error| error_status::platform(&error))?
            != registry_id
        {
            return Err(BrokerRuntimeError::Unavailable);
        }
        #[cfg(windows)]
        windows
            .revalidate_broker()
            .map_err(|error| error_status::platform(&error))?;
        let authority = Arc::new(authority::BrokerCurrentBindingAuthority::new(
            registry_id.clone(),
        ));
        let platform_owner = Arc::new(platform::BrokerPlatformOwner::new());
        let store = crate::custody::CustodyStore::open_pending(
            database,
            platform_owner.as_ref(),
            Arc::<authority::BrokerCurrentBindingAuthority>::clone(&authority),
        )?;
        Ok(Self {
            store,
            authority,
            registry_id,
            #[cfg(windows)]
            windows,
            _executable,
        })
    }

    /// Observes one exact named-pipe peer while the broker holds the pipe's
    /// RAII impersonation guard. The private adapter retains the process,
    /// image, primary-token, and impersonated-token handles and binds their
    /// observations to the kernel pipe PID/session. No sysinfo, path-only,
    /// same-user, or split-snapshot substitute is accepted.
    pub fn observe_impersonated_named_pipe_client(
        &self,
        pipe_process_id: u32,
        pipe_session_id: u32,
    ) -> Result<BrokerPeerAdmissionObservation, BrokerRuntimeError> {
        if pipe_process_id == 0 || pipe_session_id == 0 {
            return Err(BrokerRuntimeError::InvalidRequest);
        }
        #[cfg(windows)]
        {
            let platform = self
                .windows
                .observe_peer(pipe_process_id, pipe_session_id)
                .map_err(|error| error_status::platform(&error))?;
            Ok(BrokerPeerAdmissionObservation {
                platform,
                _private: super::PeerAdmissionPrivate,
            })
        }
        #[cfg(not(windows))]
        {
            Err(BrokerRuntimeError::Unavailable)
        }
    }

    /// The broker must not advertise a listener after any retained broker,
    /// enrollment, or SCM observation has drifted.
    pub fn peer_admission_available(&self) -> Result<(), BrokerRuntimeError> {
        #[cfg(windows)]
        {
            self.windows
                .revalidate_broker()
                .map_err(|error| error_status::platform(&error))
        }
        #[cfg(not(windows))]
        {
            Err(BrokerRuntimeError::Unavailable)
        }
    }

    #[cfg(windows)]
    pub fn broker_process_identity(&self) -> Result<BrokerProcessIdentity, BrokerRuntimeError> {
        let (process_id, process_epoch, session_id) = self
            .windows
            .broker_process_identity()
            .map_err(|error| error_status::platform(&error))?;
        Ok(BrokerProcessIdentity::new(
            process_id,
            process_epoch,
            session_id,
        ))
    }

    /// Revalidates the one pinned peer observation against immutable
    /// installer enrollment before any caller-controlled transcript is used.
    pub fn authorize_client_peer(
        &self,
        observation: &BrokerPeerAdmissionObservation,
    ) -> Result<(), BrokerRuntimeError> {
        #[cfg(windows)]
        {
            self.windows
                .authorize_peer(&observation.platform)
                .map_err(|error| error_status::platform(&error))
        }
        #[cfg(not(windows))]
        {
            let _ = observation;
            Err(BrokerRuntimeError::Unavailable)
        }
    }

    /// Revalidates the retained process handle and binds its PID, creation
    /// epoch, token SID/integrity/session, and pipe PID/session to this exact
    /// bootstrap/client-hello nonce and transcript immediately before the
    /// broker releases session key material. Consuming the non-cloneable
    /// observation makes the per-connection admission one-shot.
    pub fn authorize_client_transcript(
        &self,
        observation: BrokerPeerAdmissionObservation,
        bootstrap: &BootstrapPacket,
        hello: &UntrustedClientHello,
        pipe_process_id: u32,
        pipe_session_id: u32,
    ) -> Result<BrokerAuthorizedClientTranscript, BrokerRuntimeError> {
        #[cfg(windows)]
        {
            let platform = self
                .windows
                .authorize_transcript(
                    observation.platform,
                    bootstrap,
                    hello,
                    pipe_process_id,
                    pipe_session_id,
                )
                .map_err(|error| error_status::platform(&error))?;
            Ok(BrokerAuthorizedClientTranscript {
                platform,
                _private: super::AuthorizedTranscriptPrivate,
            })
        }
        #[cfg(not(windows))]
        {
            let _ = (
                observation,
                bootstrap,
                hello,
                pipe_process_id,
                pipe_session_id,
            );
            Err(BrokerRuntimeError::Unavailable)
        }
    }

    /// Revalidate the retained OS transcript immediately before binding it to
    /// one authenticated AccountIssuer request. Consuming the transcript keeps
    /// the Protected admission one-shot and prevents request substitution.
    pub fn authorize_account_issuer_request(
        &self,
        transcript: BrokerAuthorizedClientTranscript,
        request: &AuthenticatedAccountIssuerRequest,
    ) -> Result<ProtectedAccountIssuerRequestAdmission, BrokerRuntimeError> {
        #[cfg(windows)]
        {
            self.windows
                .revalidate_authorized_peer(&transcript.platform)
                .map_err(|error| error_status::platform(&error))?;
            Ok(
                ProtectedAccountIssuerRequestAdmission::from_authorized_peer(
                    transcript.platform,
                    request,
                ),
            )
        }
        #[cfg(not(windows))]
        {
            let _ = (transcript, request);
            Err(BrokerRuntimeError::Unavailable)
        }
    }

    /// Builds the listener ACL from the installer-owned enrollment record.
    /// Missing or malformed enrollment keeps the broker unavailable; the
    /// service must never fall back to a broad/default same-user ACL.
    #[cfg(windows)]
    pub fn broker_pipe_sddl(&self) -> Result<String, BrokerRuntimeError> {
        self.windows
            .pipe_sddl()
            .map_err(|error| error_status::platform(&error))
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
