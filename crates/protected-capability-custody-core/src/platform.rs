use std::path::Path;

use thiserror::Error;

pub mod record;
pub mod request;

use record::BrokerRecord;
use request::{BrokerLookup, TransitionRequest};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SecurityLevel {
    Unavailable,
    InProcessOnly,
    SameUserIsolated,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DatabasePathSecurity {
    Unavailable,
    OwnerOnlyNoFollowStable,
}

const DATABASE_IDENTITY_BYTES: usize = 96;

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct DatabaseIdentity {
    canonical: [u8; DATABASE_IDENTITY_BYTES],
}

impl DatabaseIdentity {
    pub fn as_bytes(&self) -> &[u8; DATABASE_IDENTITY_BYTES] {
        &self.canonical
    }

    pub(crate) fn from_parts(
        canonical_path_digest: [u8; 32],
        physical_file_digest: [u8; 32],
        database_instance_id: [u8; 32],
    ) -> Result<Self, PlatformError> {
        if canonical_path_digest == [0_u8; 32]
            || physical_file_digest == [0_u8; 32]
            || database_instance_id == [0_u8; 32]
        {
            return Err(PlatformError::InvalidAttestation);
        }
        let mut canonical = [0_u8; DATABASE_IDENTITY_BYTES];
        canonical[..32].copy_from_slice(&canonical_path_digest);
        canonical[32..64].copy_from_slice(&physical_file_digest);
        canonical[64..].copy_from_slice(&database_instance_id);
        Ok(Self { canonical })
    }

    pub(crate) fn from_bytes(value: &[u8]) -> Result<Self, PlatformError> {
        let canonical: [u8; DATABASE_IDENTITY_BYTES] = value
            .try_into()
            .map_err(|_| PlatformError::InvalidAttestation)?;
        if canonical[..32] == [0_u8; 32]
            || canonical[32..64] == [0_u8; 32]
            || canonical[64..] == [0_u8; 32]
        {
            return Err(PlatformError::InvalidAttestation);
        }
        Ok(Self { canonical })
    }
}

impl std::fmt::Debug for DatabaseIdentity {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("DatabaseIdentity")
            .field("opaque", &"<redacted>")
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlatformAttestation {
    pub security_level: SecurityLevel,
    pub database_path_security: DatabasePathSecurity,
    pub key_epoch: u64,
    pub writer_epoch: u64,
    pub watermark_floor: u64,
    pub database_identity: DatabaseIdentity,
}

impl PlatformAttestation {
    pub fn new(
        security_level: SecurityLevel,
        database_path_security: DatabasePathSecurity,
        key_epoch: u64,
        writer_epoch: u64,
        watermark_floor: u64,
        database_identity: DatabaseIdentity,
    ) -> Result<Self, PlatformError> {
        if key_epoch == 0 || writer_epoch == 0 {
            return Err(PlatformError::InvalidAttestation);
        }
        Ok(Self {
            security_level,
            database_path_security,
            key_epoch,
            writer_epoch,
            watermark_floor,
            database_identity,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum SealedState {
    Prepared = 1,
    CommitAmbiguous = 2,
    AbortAmbiguous = 3,
    Committed = 4,
    Aborted = 5,
}

#[derive(Clone, Copy)]
pub struct SealContext<'a> {
    pub record_namespace: &'a [u8],
    pub schema_version: u32,
    pub binding_version: u16,
    pub database_identity: DatabaseIdentity,
    pub record_id: &'a [u8; 32],
    pub lookup_digest: &'a [u8; 32],
    pub binding_digest: &'a [u8; 32],
    pub canonical_binding: &'a [u8],
    pub state: SealedState,
    pub sequence: u64,
    pub key_epoch: u64,
    pub writer_epoch: u64,
    pub anti_rollback_watermark: u64,
}

#[derive(Debug, Error)]
pub enum PlatformError {
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

/// This port must be implemented by an authenticated, isolated same-user
/// broker. Direct in-process sealing is intentionally rejected by the core.
pub trait PlatformCustodyPort: Send + Sync {
    /// Validate ACLs, owner-only mutation, no-follow opens, and stable OS file
    /// identity for the exact database before returning
    /// `OwnerOnlyNoFollowStable`. The core also holds no-follow file/parent
    /// handles, but rejects the adapter unless both layers attest the path. The
    /// broker must durably bind the physical-file identity to the database
    /// instance identifier on first use and reject later instance rebinding.
    fn attest_database(
        &self,
        canonical_path: &Path,
        identity: DatabaseIdentity,
    ) -> Result<PlatformAttestation, PlatformError>;

    /// Atomically reserve the lookup key, advance the external watermark, seal
    /// the complete next context, and durably publish it before returning.
    fn reserve(&self, next: TransitionRequest<'_>) -> Result<BrokerRecord, PlatformError>;

    /// Atomically compare every field of `prior`, advance the external
    /// watermark, seal the next context, and durably publish it.
    fn advance(
        &self,
        prior: &BrokerRecord,
        next: TransitionRequest<'_>,
    ) -> Result<BrokerRecord, PlatformError>;

    /// Return the broker's durable current state for the exact domain-separated
    /// lookup, or `None` only when no reservation has ever existed.
    fn current(&self, lookup: BrokerLookup<'_>) -> Result<Option<BrokerRecord>, PlatformError>;

    /// Authenticate the seal against every field in this context.
    fn verify(&self, context: SealContext<'_>, sealed: &[u8]) -> Result<(), PlatformError>;
}
