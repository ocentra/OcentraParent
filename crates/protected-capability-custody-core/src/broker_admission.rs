use std::sync::Arc;

use thiserror::Error;

use crate::binding::BindingError;
use crate::custody::{CustodyError, CustodyStore};
use ocentra_protected_capability_custody_protocol::response::{
    ObservedGenerations, ResponseStatus,
};
use ocentra_protected_capability_custody_protocol::types::OpaquePreparedToken;

pub mod account_issuer_request;
mod authority;
mod error_status;
mod executable;
mod finalize;
mod outcome;
mod platform;
mod prepare;
mod record_codec;
mod recover;
mod runtime;
mod storage_path;
mod token;
mod wire;

/// A client-side, non-cloneable proof that the fixed named-pipe server and
/// this process still match protected enrollment and retained OS observations.
/// The constructor derives the fixed enrollment identity internally; callers
/// cannot supply a PID, SID, path, image digest, or authority value.
#[cfg(windows)]
pub struct ClientAnchor {
    platform: platform::BrokerClientAnchor,
}

/// OS-observed identity for the current enrolled client process.
#[cfg(windows)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ClientProcessIdentity {
    process_id: u32,
    process_epoch: u64,
    session_id: u32,
}

#[cfg(windows)]
impl ClientProcessIdentity {
    pub fn process_id(self) -> u32 {
        self.process_id
    }

    pub fn process_epoch(self) -> u64 {
        self.process_epoch
    }

    pub fn session_id(self) -> u32 {
        self.session_id
    }
}

/// OS-observed identity for the retained, enrolled broker process.
#[cfg(windows)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BrokerProcessIdentity {
    process_id: u32,
    process_epoch: u64,
    session_id: u32,
}

#[cfg(windows)]
impl BrokerProcessIdentity {
    pub(crate) fn new(process_id: u32, process_epoch: u64, session_id: u32) -> Self {
        Self {
            process_id,
            process_epoch,
            session_id,
        }
    }

    pub fn process_id(self) -> u32 {
        self.process_id
    }

    pub fn process_epoch(self) -> u64 {
        self.process_epoch
    }

    pub fn session_id(self) -> u32 {
        self.session_id
    }
}

#[cfg(windows)]
impl ClientAnchor {
    pub fn open(
        broker_process_id: u32,
        broker_session_id: u32,
    ) -> Result<Self, BrokerRuntimeError> {
        let database_path = storage_path::fixed_database_identity_path()?;
        let registry_id = platform::registry_id(&database_path)
            .map_err(|error| error_status::platform(&error))?;
        let platform =
            platform::BrokerClientAnchor::open(&registry_id, broker_process_id, broker_session_id)
                .map_err(|error| error_status::platform(&error))?;
        Ok(Self { platform })
    }

    pub fn revalidate(&self) -> Result<(), BrokerRuntimeError> {
        self.platform
            .revalidate()
            .map_err(|error| error_status::platform(&error))
    }

    pub fn client_identity(&self) -> Result<ClientProcessIdentity, BrokerRuntimeError> {
        let (process_id, process_epoch, session_id) = self
            .platform
            .client_identity()
            .map_err(|error| error_status::platform(&error))?;
        Ok(ClientProcessIdentity {
            process_id,
            process_epoch,
            session_id,
        })
    }

    pub fn authorize_broker_hello(
        &self,
        hello: &ocentra_protected_capability_custody_protocol::handshake::UntrustedBrokerHello,
        broker_process_id: u32,
        broker_session_id: u32,
    ) -> Result<(), BrokerRuntimeError> {
        self.platform
            .authorize_broker_hello(hello, broker_process_id, broker_session_id)
            .map_err(|error| error_status::platform(&error))
    }
}

pub struct BrokerCustodyRuntime {
    store: CustodyStore,
    authority: Arc<authority::BrokerCurrentBindingAuthority>,
    registry_id: String,
    #[cfg(windows)]
    windows: platform::BrokerWindowsRuntime,
    _executable: BrokerExecutableGuard,
}

/// A validated, non-cloneable handle to the running fixed-install broker
/// executable. The pinned file handle denies replacement while the service
/// runtime is alive.
struct BrokerExecutableGuard {
    _executable_handle: std::fs::File,
}

/// Opaque admission proving that the current OS process is the dedicated
/// protected-custody broker executable. Its fields are private and it is not
/// cloneable, so ordinary in-process callers cannot mint broker admission.
struct BrokerProcessAdmission {
    _executable: BrokerExecutableGuard,
    database: crate::path_security::PendingSecuredPath,
    registry_id: String,
    #[cfg(windows)]
    windows: platform::BrokerWindowsRuntime,
}

/// Opaque admission for one exact named-pipe peer. The private Windows adapter
/// retains the process, image, primary-token, and impersonated-token handles
/// through authorization. Keeping every field private prevents callers from
/// substituting PID/path snapshots or self-asserted token values.
pub struct BrokerPeerAdmissionObservation {
    #[cfg(windows)]
    platform: platform::BrokerPeerObservation,
    _private: PeerAdmissionPrivate,
}

struct PeerAdmissionPrivate;

/// Opaque proof that the retained OS peer observation was revalidated and
/// bound to the exact bootstrap/client-hello transcript immediately before
/// broker session key release. The private platform adapter is the sole
/// constructor.
pub struct BrokerAuthorizedClientTranscript {
    #[cfg(windows)]
    platform: platform::BrokerAuthorizedPeer,
    _private: AuthorizedTranscriptPrivate,
}

struct AuthorizedTranscriptPrivate;

#[path = "../tests/unit/broker_admission_account_issuer_request.rs"]
mod account_issuer_request_tests;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BrokerPlatformSessionState {
    key_epoch: u64,
    writer_lease_epoch: u64,
    watermark: u64,
}

impl BrokerPlatformSessionState {
    pub fn key_epoch(self) -> u64 {
        self.key_epoch
    }

    pub fn writer_lease_epoch(self) -> u64 {
        self.writer_lease_epoch
    }

    pub fn watermark(self) -> u64 {
        self.watermark
    }
}

pub struct BrokerCustodyOutcome {
    status: ResponseStatus,
    observed_generations: Option<ObservedGenerations>,
    opaque_token: Option<OpaquePreparedToken>,
}

impl BrokerCustodyOutcome {
    pub fn status(&self) -> ResponseStatus {
        self.status
    }

    pub fn observed_generations(&self) -> Option<ObservedGenerations> {
        self.observed_generations
    }

    pub fn into_opaque_token(self) -> Option<OpaquePreparedToken> {
        self.opaque_token
    }
}

#[derive(Debug, Error)]
pub enum BrokerRuntimeError {
    #[error("current process is not the protected custody broker")]
    InvalidBrokerProcess,
    #[error("protected custody deployment is not provisioned")]
    DeploymentRequired,
    #[error("broker request is invalid")]
    InvalidRequest,
    #[error("broker custody runtime is unavailable")]
    Unavailable,
    #[error("broker binding is invalid")]
    Binding(#[from] BindingError),
    #[error("broker custody store failed")]
    Custody(#[from] CustodyError),
}
