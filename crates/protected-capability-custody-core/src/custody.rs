mod finalize;
mod prepare;
mod reconcile;
mod recover;
mod support;

use std::fmt;
use std::path::Path;
use std::sync::{Arc, Mutex};

use rusqlite::Connection;
use thiserror::Error;

use crate::authority::CurrentBindingPort;
use crate::binding::BindingLocator;
use crate::path_security::{PendingSecuredPath, SecuredPath};
use crate::platform::PlatformCustodyPort;

pub struct CustodyStore<P, A> {
    pub(super) connection: Mutex<Connection>,
    pub(super) operation: Mutex<()>,
    pub(super) platform: Arc<P>,
    pub(super) authority: Arc<A>,
    pub(super) secured_path: SecuredPath,
}

pub struct PreparedCapability {
    pub(super) record_id: [u8; 32],
    pub(super) lookup_digest: [u8; 32],
    pub(super) sequence: u64,
}

pub struct CommittedCapability {
    pub(super) record_id: [u8; 32],
    pub(super) lookup_digest: [u8; 32],
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
        let _retained_identity = (&self.record_id, &self.lookup_digest, self.sequence);
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
    #[error("protected capability binding is not current")]
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
    #[error("custody database is ahead of the authenticated broker")]
    BrokerBehind,
    #[error("custody database path is unsafe")]
    UnsafeDatabasePath,
    #[error("custody database path was replaced")]
    DatabaseReplaced,
    #[error("protected capability database operation failed")]
    Database,
}

impl<P: PlatformCustodyPort, A: CurrentBindingPort> CustodyStore<P, A> {
    pub fn open(path: &Path, platform: P, authority: A) -> Result<Self, CustodyError> {
        let pending_path = PendingSecuredPath::open(path).map_err(support::map_path_error)?;
        let (connection, database_instance_id) =
            crate::storage::open(pending_path.canonical()).map_err(support::map_storage_error)?;
        pending_path.revalidate().map_err(support::map_path_error)?;
        let secured_path = pending_path
            .bind_instance(database_instance_id)
            .map_err(support::map_path_error)?;
        let platform = Arc::new(platform);
        support::attest_path(platform.as_ref(), &secured_path)?;
        crate::storage::validate_all(&connection, secured_path.identity())
            .map_err(support::map_storage_error)?;
        secured_path.revalidate().map_err(support::map_path_error)?;
        Ok(Self {
            connection: Mutex::new(connection),
            operation: Mutex::new(()),
            platform,
            authority: Arc::new(authority),
            secured_path,
        })
    }

    pub fn prepare(&self, locator: &BindingLocator) -> Result<PreparedCapability, CustodyError> {
        prepare::run(self, locator)
    }

    pub fn finalize(
        &self,
        prepared: PreparedCapability,
        decision: Decision,
    ) -> Result<FinalizeOutcome, CustodyError> {
        finalize::run(self, prepared, decision)
    }

    pub fn recover(&self, locator: &BindingLocator) -> Result<RecoveryOutcome, CustodyError> {
        recover::run(self, locator)
    }

    pub fn resolve_ambiguity(
        &self,
        locator: &BindingLocator,
    ) -> Result<RecoveryOutcome, CustodyError> {
        recover::resolve(self, locator)
    }
}
