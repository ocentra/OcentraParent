use std::path::Path;

use thiserror::Error;

pub(crate) mod identity;
pub(crate) mod record;
pub(crate) mod request;

use identity::{DatabaseIdentity, PhysicalDatabaseIdentity};
use record::BrokerRecord;
use request::{BrokerLookup, TransitionRequest};

pub(crate) mod sealed {
    /// Implement only beside the isolated production broker adapter. There is
    /// intentionally no blanket implementation.
    pub(crate) trait TrustedPlatformOwner {}

    /// Implement only for the owned guard returned by that trusted adapter.
    /// Keeping this separate prevents a future adapter from returning an
    /// arbitrary caller-implemented broker port after admission.
    pub(crate) trait TrustedDatabaseGuard {}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SecurityLevel {
    DedicatedServiceIsolated,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum DatabasePathSecurity {
    BrokerExclusiveWriterNoFollowRollbackJournal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PlatformAttestation {
    security_level: SecurityLevel,
    database_path_security: DatabasePathSecurity,
    pub(crate) key_epoch: u64,
    pub(crate) writer_epoch: u64,
    pub(crate) watermark_floor: u64,
    pub(crate) database_identity: DatabaseIdentity,
}

impl PlatformAttestation {
    pub(crate) fn isolated_broker(
        key_epoch: u64,
        writer_epoch: u64,
        watermark_floor: u64,
        database_identity: DatabaseIdentity,
    ) -> Self {
        Self {
            security_level: SecurityLevel::DedicatedServiceIsolated,
            database_path_security:
                DatabasePathSecurity::BrokerExclusiveWriterNoFollowRollbackJournal,
            key_epoch,
            writer_epoch,
            watermark_floor,
            database_identity,
        }
    }

    pub(crate) fn security_level(self) -> SecurityLevel {
        self.security_level
    }

    pub(crate) fn database_path_security(self) -> DatabasePathSecurity {
        self.database_path_security
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub(crate) enum SealedState {
    Prepared = 1,
    CommitAmbiguous = 2,
    AbortAmbiguous = 3,
    Committed = 4,
    Aborted = 5,
}

#[derive(Clone, Copy)]
pub(crate) struct SealContext<'a> {
    pub(crate) record_namespace: &'a [u8],
    pub(crate) schema_version: u32,
    pub(crate) binding_version: u16,
    pub(crate) database_identity: DatabaseIdentity,
    pub(crate) record_id: &'a [u8; 32],
    pub(crate) lookup_digest: &'a [u8; 32],
    pub(crate) binding_digest: &'a [u8; 32],
    pub(crate) canonical_binding: &'a [u8],
    pub(crate) state: SealedState,
    pub(crate) sequence: u64,
    pub(crate) key_epoch: u64,
    pub(crate) writer_epoch: u64,
    pub(crate) anti_rollback_watermark: u64,
}

#[derive(Debug, Error)]
pub(crate) enum PlatformError {
    #[error("platform custody is unavailable")]
    Unavailable,
    #[error("platform custody rejected the request")]
    Rejected,
    #[error("platform custody detected tampering")]
    Tampered,
    #[error("platform custody detected a wrong binding")]
    WrongBinding,
    #[error("platform custody key material rotated")]
    Rotated,
    #[error("platform custody detected a conflicting writer")]
    Conflict,
    #[error("platform custody anti-rollback watermark regressed")]
    AntiRollback,
    #[error("platform returned an invalid attestation")]
    InvalidAttestation,
}

#[derive(Debug, Error)]
pub(crate) enum TransitionFailure {
    #[error("platform transition was definitely not applied")]
    DefinitelyNotApplied(#[source] PlatformError),
    #[error("platform transition outcome is unknown")]
    OutcomeUnknown,
}

/// Admission owner implemented only beside an authenticated, isolated broker.
/// It must acquire an OS-enforced writer lease for the exact database, journal,
/// and parent namespace before SQLite is opened. The returned owned guard must
/// retain that lease until it is dropped; direct in-process DPAPI or a token
/// that merely asserts custody is not an implementation of this interface.
pub(crate) trait PlatformCustodyOwner: sealed::TrustedPlatformOwner + Send + Sync {
    /// Acquire exclusion first, then revalidate the supplied physical identity,
    /// owner ACL, empty tracked journal, and absence of WAL/SHM before return.
    /// The adapter must fail rather than return a guard if an untrusted writer
    /// can still mutate or create any of those paths while the guard is alive.
    fn acquire_database(
        &self,
        canonical_path: &Path,
        physical_identity: PhysicalDatabaseIdentity,
    ) -> Result<Box<dyn PlatformDatabaseGuard>, PlatformError>;
}

/// Opaque lifetime guard for the broker's exclusive database-writer custody.
/// All sealing operations are reachable only through this guard, so the core
/// cannot accidentally drop writer custody while SQLite or broker state is in
/// use. No external crate can implement or construct this interface.
pub(crate) trait PlatformDatabaseGuard: sealed::TrustedDatabaseGuard + Send + Sync {
    /// Validate owner ACLs, isolated writer custody, no-follow opens, stable OS
    /// file identity, the tracked PERSIST rollback journal, and absence of
    /// WAL/SHM state. Bind the pre-open physical identity to the authenticated
    /// database instance on first use and reject copies or later rebinding.
    fn attest_database(
        &self,
        canonical_path: &Path,
        identity: DatabaseIdentity,
    ) -> Result<PlatformAttestation, PlatformError>;

    /// Atomically reserve the lookup key, advance the external watermark, seal
    /// the complete canonical binding and state (never a caller digest alone),
    /// authenticate the complete context as AAD, and durably publish it.
    fn reserve(&self, next: TransitionRequest<'_>) -> Result<BrokerRecord, TransitionFailure>;

    /// Atomically compare every field of `prior`, advance the external
    /// watermark, seal the complete next binding/state, authenticate the
    /// complete next context as AAD, and durably publish it.
    fn advance(
        &self,
        prior: &BrokerRecord,
        next: TransitionRequest<'_>,
    ) -> Result<BrokerRecord, TransitionFailure>;

    /// Return the broker's durable current state for the exact domain-separated
    /// lookup, or `None` only when no reservation has ever existed.
    fn current(&self, lookup: BrokerLookup<'_>) -> Result<Option<BrokerRecord>, PlatformError>;

    /// Open the isolated seal, require its full binding/state payload to match,
    /// and authenticate every field of this context as AAD.
    fn open_and_verify(&self, context: SealContext<'_>, sealed: &[u8])
        -> Result<(), PlatformError>;
}
