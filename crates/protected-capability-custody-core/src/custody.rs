mod finalize;
mod prepare;
mod recover;
mod support;

use std::fmt;
use std::path::Path;
use std::sync::{Arc, Mutex};

use rusqlite::Connection;
use thiserror::Error;

use crate::binding::Binding;
use crate::platform::PlatformCustodyPort;

pub struct CustodyStore<P> {
    pub(super) connection: Mutex<Connection>,
    pub(super) platform: Arc<P>,
}

pub struct PreparedCapability {
    pub(super) record_id: Vec<u8>,
    pub(super) binding_digest: [u8; 32],
    pub(super) sequence: u64,
}

pub struct CommittedCapability {
    pub(super) record_id: Vec<u8>,
    pub(super) binding_digest: [u8; 32],
    pub(super) sequence: u64,
}

impl fmt::Debug for PreparedCapability {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PreparedCapability")
            .field("opaque", &"<redacted>")
            .finish()
    }
}

impl fmt::Debug for CommittedCapability {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CommittedCapability")
            .field("opaque", &"<redacted>")
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Decision {
    Commit,
    Abort,
}

#[derive(Debug)]
pub enum FinalizeOutcome {
    Committed(CommittedCapability),
    Aborted,
    CommitAmbiguous,
    AbortAmbiguous,
}

#[derive(Debug)]
pub enum RecoveryOutcome {
    Prepared(PreparedCapability),
    Committed(CommittedCapability),
    Aborted,
    CommitAmbiguous,
    AbortAmbiguous,
}

#[derive(Debug, Error)]
pub enum CustodyError {
    #[error("protected capability custody is unavailable")]
    Unavailable,
    #[error("protected capability custody data is tampered")]
    Tampered,
    #[error("protected capability binding is wrong")]
    WrongBinding,
    #[error("protected capability key epoch rotated")]
    Rotated,
    #[error("protected capability operation conflicts with durable state")]
    Conflict,
    #[error("protected capability record is missing")]
    Missing,
    #[error("protected capability is already committed")]
    AlreadyCommitted,
    #[error("protected capability is aborted")]
    Aborted,
    #[error("protected capability commit is ambiguous")]
    CommitAmbiguous,
    #[error("protected capability abort is ambiguous")]
    AbortAmbiguous,
    #[error("protected capability database operation failed")]
    Database,
}

impl<P: PlatformCustodyPort> CustodyStore<P> {
    pub fn open(path: &Path, platform: P) -> Result<Self, CustodyError> {
        let platform = Arc::new(platform);
        support::attest(platform.as_ref())?;
        let connection = crate::storage::open(path).map_err(support::map_storage_error)?;
        crate::storage::validate_all(&connection).map_err(support::map_storage_error)?;
        Ok(Self {
            connection: Mutex::new(connection),
            platform,
        })
    }

    pub fn prepare(&self, binding: &Binding) -> Result<PreparedCapability, CustodyError> {
        prepare::run(self, binding)
    }

    pub fn finalize(
        &self,
        prepared: PreparedCapability,
        current_binding: &Binding,
        decision: Decision,
    ) -> Result<FinalizeOutcome, CustodyError> {
        finalize::run(self, prepared, current_binding, decision)
    }

    pub fn recover(&self, binding: &Binding) -> Result<RecoveryOutcome, CustodyError> {
        recover::run(self, binding)
    }
}
