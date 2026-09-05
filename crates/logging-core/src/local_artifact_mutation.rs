//! Safe facade for the platform-owned local-artifact mutation boundary.
//!
//! The facade deliberately exposes no descriptor, handle, identity input, or
//! caller-selected provider. On Windows the native crate owns those details;
//! other platforms remain explicitly unsupported until an equivalent owner is
//! implemented.

use std::fmt;
#[cfg(not(windows))]
use std::marker::PhantomData;
use std::path::Path;
#[cfg(not(windows))]
use std::path::PathBuf;

#[cfg(windows)]
#[path = "local_artifact_mutation_identity.rs"]
mod identity;
#[cfg(windows)]
#[path = "local_artifact_mutation_native.rs"]
mod native;
#[path = "local_artifact_mutation_owner.rs"]
mod owner;
#[cfg(windows)]
#[path = "local_artifact_mutation_read_snapshot.rs"]
mod read_snapshot;
#[cfg(windows)]
#[path = "local_artifact_mutation_stat.rs"]
mod stat;
#[cfg(windows)]
use native::outcome::outcome_from_native;

pub const MAX_LOCAL_ARTIFACT_BYTES: u64 = 64 * 1024 * 1024;

/// A handle-observed physical identity. Callers may compare it, but cannot
/// mint one to authorize a mutation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalArtifactIdentity {
    volume_serial_number: u64,
    file_id: [u8; 16],
}

/// A direct-handle stat, including target kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalArtifactStat {
    identity: LocalArtifactIdentity,
    size: u64,
    links: u32,
    is_directory: bool,
    modified_ms: i64,
}

/// A byte read and stat captured from one native target handle.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalArtifactReadSnapshot {
    stat: LocalArtifactStat,
    bytes: Vec<u8>,
}

/// A directory entry observed through the native owner's guarded chain.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalArtifactDirectoryEntry {
    name: String,
    stat: LocalArtifactStat,
}

impl LocalArtifactDirectoryEntry {
    #[cfg(windows)]
    fn from_native(
        entry: &ocentra_parent_logging_local_artifact_windows_ffi::owner::LocalArtifactDirectoryEntry,
    ) -> Self {
        Self {
            name: entry.name().to_owned(),
            stat: LocalArtifactStat::from_native(entry.stat()),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn stat(&self) -> LocalArtifactStat {
        self.stat
    }
}

/// A caller request containing only a relative target and payload.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LocalArtifactMutation {
    Append {
        relative_path: String,
        payload: Vec<u8>,
    },
    Replace {
        relative_path: String,
        payload: Vec<u8>,
    },
    Remove {
        relative_path: String,
    },
    RemoveTree {
        relative_path: String,
    },
}

/// Terminal result persisted by the native owner for a request identifier.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LocalArtifactMutationOutcome {
    Appended { offset: u64, length: u64 },
    Replaced,
    Removed { existed: bool },
    TransactionCommitted { count: u32 },
    Unsupported { operation: String },
}

/// A replayable terminal mutation receipt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalArtifactMutationReceipt {
    request_id: String,
    operation: String,
    relative_path: String,
    outcome: LocalArtifactMutationOutcome,
    replayed: bool,
}

impl LocalArtifactMutationReceipt {
    #[cfg(windows)]
    fn from_native(
        receipt: &ocentra_parent_logging_local_artifact_windows_ffi::owner::LocalArtifactMutationReceipt,
    ) -> Self {
        Self {
            request_id: receipt.request_id().to_owned(),
            operation: receipt.operation().to_owned(),
            relative_path: receipt.relative_path().to_owned(),
            outcome: outcome_from_native(receipt.outcome()),
            replayed: receipt.replayed(),
        }
    }

    pub fn request_id(&self) -> &str {
        &self.request_id
    }

    pub fn operation(&self) -> &str {
        &self.operation
    }

    pub fn relative_path(&self) -> &str {
        &self.relative_path
    }

    pub fn outcome(&self) -> &LocalArtifactMutationOutcome {
        &self.outcome
    }

    pub fn replayed(&self) -> bool {
        self.replayed
    }
}

/// Failure at the owner boundary. The native error code is retained as text
/// only for diagnostics; it is never interpreted as caller authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LocalArtifactMutationError {
    UnsupportedPlatform,
    InvalidPath,
    InvalidRequestId,
    RequestIdConflict,
    RootIdentityChanged,
    AncestorIdentityChanged,
    LinkOrReparseDetected,
    HardlinkDetected,
    OwnershipChanged,
    LockConflict,
    NotFound,
    AlreadyExists,
    SizeLimit,
    DurabilityFailure,
    RecoveryRequired,
    UnsupportedOperation,
    Native(String),
}

impl fmt::Display for LocalArtifactMutationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "local-artifact owner error: {self:?}")
    }
}

impl std::error::Error for LocalArtifactMutationError {}

/// The directory durability result is intentionally explicit.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocalArtifactDirectoryDurability {
    Synced,
}

#[cfg(windows)]
pub struct LocalArtifactMutationOwner {
    inner: ocentra_parent_logging_local_artifact_windows_ffi::owner::LocalArtifactMutationOwner,
}

#[cfg(not(windows))]
pub struct LocalArtifactMutationOwner {
    root: PathBuf,
}

#[cfg(windows)]
pub struct LocalArtifactMutationSession<'a> {
    inner: ocentra_parent_logging_local_artifact_windows_ffi::owner::MutationSession<'a>,
}

#[cfg(not(windows))]
pub struct LocalArtifactMutationSession<'a> {
    root: &'a Path,
    _marker: PhantomData<&'a Path>,
}
