use std::sync::Arc;

use thiserror::Error;

use crate::binding::BindingError;
use crate::custody::{CustodyError, CustodyStore};
use ocentra_protected_capability_custody_protocol::response::{
    ObservedGenerations, ResponseStatus,
};
use ocentra_protected_capability_custody_protocol::types::OpaquePreparedToken;

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

pub struct BrokerCustodyRuntime {
    store: CustodyStore,
    authority: Arc<authority::BrokerCurrentBindingAuthority>,
    registry_id: String,
    _executable: BrokerExecutableGuard,
}

/// A validated, non-cloneable handle to the running fixed-install broker
/// executable. The pinned file handle denies replacement while the service
/// runtime is alive.
pub struct BrokerExecutableGuard {
    _executable_handle: std::fs::File,
}

/// Opaque admission proving that the current OS process is the dedicated
/// protected-custody broker executable. Its fields are private and it is not
/// cloneable, so ordinary in-process callers cannot mint broker admission.
pub struct BrokerProcessAdmission {
    _executable: BrokerExecutableGuard,
    database: crate::path_security::PendingSecuredPath,
}

/// Opaque admission for one exact named-pipe peer. A future Windows adapter
/// must retain one `OpenProcess` handle while deriving PID, creation epoch,
/// canonical image, image digest, and liveness; observe SID, integrity, and
/// token session from the impersonated pipe token; match both PID and session
/// to the pipe; and keep the process handle alive through authorization.
///
/// The current safe dependency set cannot construct this type. Keeping its
/// only field private prevents callers from substituting PID/path snapshots or
/// self-asserted token values.
pub struct BrokerPeerAdmissionObservation {
    _private: PeerAdmissionPrivate,
}

struct PeerAdmissionPrivate;

/// Opaque proof that the retained OS peer observation was revalidated and
/// bound to the exact bootstrap/client-hello transcript immediately before
/// broker session key release. The missing platform adapter is the sole
/// intended constructor.
pub struct BrokerAuthorizedClientTranscript {
    _private: AuthorizedTranscriptPrivate,
}

struct AuthorizedTranscriptPrivate;

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
