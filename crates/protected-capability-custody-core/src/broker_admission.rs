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

/// A validated, non-cloneable handle to the fixed sibling broker executable.
/// It denies write/delete sharing while the client session is alive, so the
/// authenticated child cannot be replaced after preflight.
pub struct BrokerExecutableGuard {
    canonical_path: std::path::PathBuf,
    _executable_handle: std::fs::File,
}

/// Opaque admission proving that the current OS process is the dedicated
/// protected-custody broker executable. Its fields are private and it is not
/// cloneable, so ordinary in-process callers cannot mint broker admission.
pub struct BrokerProcessAdmission {
    _executable: BrokerExecutableGuard,
    database: crate::path_security::PendingSecuredPath,
}

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
