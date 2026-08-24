mod finalize;
mod prepare;
mod reconcile;
mod recover;
mod scope;
mod support;

use std::fmt;
use std::path::Path;
use std::sync::{Arc, Mutex};

use rusqlite::Connection;
use thiserror::Error;

use crate::authority::CurrentBindingPort;
use crate::binding::BindingLocator;
use crate::path_security::{PendingSecuredPath, SecuredPath};
use crate::platform::{PlatformCustodyOwner, PlatformDatabaseGuard};

/// Opaque admission assembled only by a trusted in-crate platform owner.
/// There is intentionally no public constructor until that owner exists.
pub(crate) struct CustodyAdmission {
    platform_owner: Arc<dyn PlatformCustodyOwner>,
    authority: Arc<dyn CurrentBindingPort>,
}

pub struct CustodyStore {
    // Field order is security-relevant: SQLite must close before the broker's
    // exclusive writer guard and pinned path handles are released.
    pub(super) connection: Mutex<Connection>,
    pub(super) operation: Mutex<()>,
    pub(super) authority: Arc<dyn CurrentBindingPort>,
    pub(super) secured_path: SecuredPath,
    pub(super) platform: Box<dyn PlatformDatabaseGuard>,
}

pub struct PreparedCapability {
    pub(super) record_id: [u8; 32],
    pub(super) lookup_digest: [u8; 32],
    pub(super) sequence: u64,
    pub(super) locator: BindingLocator,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransitionPhase {
    Prepare,
    CommitIntent,
    CommitTerminal,
    AbortIntent,
    AbortTerminal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocalReplicaFailure {
    Unavailable,
    Database,
    DatabaseReplaced,
    UnsafeDatabasePath,
    Tampered,
    Conflict,
}

#[derive(Debug, Error)]
pub enum CustodyError {
    #[error("protected capability custody is unavailable")]
    Unavailable,
    #[error("protected capability custody is unsupported on this platform")]
    UnsupportedPlatform,
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
    #[error("protected capability prepare is ambiguous")]
    PrepareAmbiguous,
    #[error("platform transition was definitely rejected")]
    BrokerRejected,
    #[error("broker committed {phase:?} but the local replica failed: {failure:?}")]
    LocalReplicaBehind {
        phase: TransitionPhase,
        failure: LocalReplicaFailure,
    },
    #[error("custody database is ahead of the authenticated broker")]
    BrokerBehind,
    #[error("custody database path is unsafe")]
    UnsafeDatabasePath,
    #[error("custody database path was replaced")]
    DatabaseReplaced,
    #[error("protected capability database operation failed")]
    Database,
}

impl CustodyStore {
    pub(crate) fn open(path: &Path, admission: CustodyAdmission) -> Result<Self, CustodyError> {
        let CustodyAdmission {
            platform_owner,
            authority,
        } = admission;
        let mut pending_path = PendingSecuredPath::open(path).map_err(support::map_path_error)?;
        pending_path
            .secure_rollback_journal()
            .map_err(support::map_path_error)?;
        let physical_identity = pending_path
            .physical_identity()
            .map_err(support::map_path_error)?;
        // The broker-held OS writer lease is acquired before SQLite can read,
        // recover, create schema, or write either the main file or journal.
        let platform = platform_owner
            .acquire_database(pending_path.canonical(), physical_identity)
            .map_err(support::map_platform_error)?;
        pending_path
            .revalidate_quiescent()
            .map_err(support::map_path_error)?;
        let opened = crate::storage::open_connection(pending_path.canonical())
            .map_err(support::sqlite::map_error);
        pending_path
            .revalidate_quiescent()
            .map_err(support::map_path_error)?;
        let (mut connection, was_empty) = opened?;
        let configured =
            crate::storage::configure(&mut connection).map_err(support::sqlite::map_error);
        pending_path
            .revalidate_quiescent()
            .map_err(support::map_path_error)?;
        configured?;
        let initialized = crate::storage::initialize_or_validate(&mut connection, was_empty)
            .map_err(support::sqlite::map_error);
        pending_path
            .revalidate_quiescent()
            .map_err(support::map_path_error)?;
        let database_instance_id = initialized?;
        let secured_path = pending_path
            .bind_instance(database_instance_id)
            .map_err(support::map_path_error)?;
        support::attest_path(platform.as_ref(), &secured_path)?;
        let validated = crate::storage::validate_all(&connection, secured_path.identity())
            .map_err(support::sqlite::map_error);
        secured_path.revalidate().map_err(support::map_path_error)?;
        validated?;
        Ok(Self {
            connection: Mutex::new(connection),
            operation: Mutex::new(()),
            authority,
            secured_path,
            platform,
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
