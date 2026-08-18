#![forbid(unsafe_code)]

//! Durable, replaceable entitlement snapshot and revocation-state custody.
//!
//! These stores persist issuer-signed wire material only.  Reading either
//! file never establishes authority; the verifier authenticates the signature
//! again on every use.  Missing, malformed, or tampered state is surfaced as
//! an explicit unavailable/corrupt result so a stale local cache cannot unlock
//! a capability. Snapshot and revocation generations are retained monotonically;
//! revocation membership cannot shrink during replacement.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::entitlement_snapshot::SignedEntitlementSnapshot;
use crate::entitlement_snapshot_authority::verifier::SnapshotVerificationReceipt;
use crate::entitlement_snapshot_values::{
    EntitlementRevocationCursor, EntitlementSignatureKeyId, EntitlementSnapshotId,
};

#[path = "entitlement_snapshot_cache_path.rs"]
pub(crate) mod path;
#[path = "entitlement_snapshot_cache_revocation.rs"]
pub(crate) mod revocation_state;
#[path = "entitlement_snapshot_cache_storage.rs"]
mod storage;

const REVOCATION_STATE_SCHEMA_VERSION: u16 = 1;
const REVOCATION_SIGNATURE_BYTES: usize = 64;
const MAX_REVOCATION_ENTRIES: usize = 16_384;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum EntitlementSnapshotCacheError {
    StorageUnavailable,
    CorruptState,
    InvalidPath,
    StaleReplacement,
}

#[derive(Clone, Debug)]
pub(crate) struct EntitlementSnapshotCache {
    path: PathBuf,
}

impl EntitlementSnapshotCache {
    pub(crate) fn open(path: impl Into<PathBuf>) -> Result<Self, EntitlementSnapshotCacheError> {
        let path = path.into();
        path::prepare_path(&path)?;
        Ok(Self { path })
    }

    pub(crate) fn read(
        &self,
    ) -> Result<Option<SignedEntitlementSnapshot>, EntitlementSnapshotCacheError> {
        path::ensure_secure_path(&self.path)?;
        let bytes = match std::fs::read(&self.path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(_error) => return Err(EntitlementSnapshotCacheError::StorageUnavailable),
        };
        let snapshot: SignedEntitlementSnapshot = serde_json::from_slice(&bytes)
            .map_err(|_error| EntitlementSnapshotCacheError::CorruptState)?;
        snapshot
            .validate_shape()
            .map_err(|_error| EntitlementSnapshotCacheError::CorruptState)?;
        Ok(Some(snapshot))
    }

    /// Persist only a snapshot that already carries the authority's
    /// signature/currentness receipt. Raw transport values have no mutation
    /// API, so they cannot pin a higher generation or poison the cache before
    /// verification. The owner-composed platform custody still must provide a
    /// handle-safe replacement implementation before this path is production
    /// reachable; this packet keeps it crate-private/manual-required.
    pub(crate) fn replace_verified(
        &self,
        receipt: &SnapshotVerificationReceipt,
    ) -> Result<(), EntitlementSnapshotCacheError> {
        let snapshot = receipt.snapshot();
        snapshot
            .validate_shape()
            .map_err(|_error| EntitlementSnapshotCacheError::CorruptState)?;
        self.with_lock(|path| {
            let existing = storage::read_snapshot_file(path)?;
            if let Some(existing) = existing {
                storage::enforce_snapshot_monotonicity(&existing, snapshot)?;
            }
            storage::write_atomic(path, snapshot)
        })
    }

    pub(crate) fn path(&self) -> &Path {
        &self.path
    }

    fn with_lock<T>(
        &self,
        operation: impl FnOnce(&Path) -> Result<T, EntitlementSnapshotCacheError>,
    ) -> Result<T, EntitlementSnapshotCacheError> {
        let lock_path = self.path.with_extension("lock");
        path::ensure_secure_path(&lock_path)?;
        let lock = std::fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .open(&lock_path)
            .map_err(|_error| EntitlementSnapshotCacheError::StorageUnavailable)?;
        fs2::FileExt::lock_exclusive(&lock)
            .map_err(|_error| EntitlementSnapshotCacheError::StorageUnavailable)?;
        let result = operation(&self.path);
        let unlock_result = fs2::FileExt::unlock(&lock)
            .map_err(|_error| EntitlementSnapshotCacheError::StorageUnavailable);
        match (result, unlock_result) {
            (Ok(value), Ok(())) => Ok(value),
            (Err(error), _) => Err(error),
            (Ok(_), Err(error)) => Err(error),
        }
    }
}

/// Signed revocation cursor state persisted by the entitlement owner.
///
/// The update is intentionally public as wire data, but its signature is
/// checked by `EntitlementSnapshotAuthority` before it influences currentness.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SignedEntitlementRevocationUpdate {
    pub schema_version: u16,
    pub issuer_key_id: EntitlementSignatureKeyId,
    pub revocation_cursor: EntitlementRevocationCursor,
    pub authority_generation: u64,
    pub revoked_snapshot_ids: Vec<EntitlementSnapshotId>,
    pub signature: Vec<u8>,
}

impl SignedEntitlementRevocationUpdate {
    pub(crate) fn signing_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(256);
        bytes.extend_from_slice(b"ocentra.entitlement.revocation.v1\0");
        append_u16(&mut bytes, self.schema_version);
        append_text(&mut bytes, self.issuer_key_id.as_str());
        append_text(&mut bytes, self.revocation_cursor.as_str());
        append_u64(&mut bytes, self.authority_generation);
        append_u32(&mut bytes, self.revoked_snapshot_ids.len() as u32);
        for snapshot_id in &self.revoked_snapshot_ids {
            append_text(&mut bytes, snapshot_id.as_str());
        }
        bytes
    }

    pub(crate) fn validate_shape(&self) -> Result<(), EntitlementSnapshotCacheError> {
        if self.schema_version != REVOCATION_STATE_SCHEMA_VERSION
            || self.signature.len() != REVOCATION_SIGNATURE_BYTES
            || self.authority_generation == 0
            || self.revoked_snapshot_ids.len() > MAX_REVOCATION_ENTRIES
        {
            return Err(EntitlementSnapshotCacheError::CorruptState);
        }
        if self
            .revoked_snapshot_ids
            .iter()
            .enumerate()
            .any(|(index, id)| {
                self.revoked_snapshot_ids[index + 1..]
                    .iter()
                    .any(|other| other == id)
            })
        {
            return Err(EntitlementSnapshotCacheError::CorruptState);
        }
        Ok(())
    }
}

fn append_text(bytes: &mut Vec<u8>, value: &str) {
    append_u64(bytes, value.len() as u64);
    bytes.extend_from_slice(value.as_bytes());
}

fn append_u16(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

fn append_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

fn append_u64(bytes: &mut Vec<u8>, value: u64) {
    bytes.extend_from_slice(&value.to_be_bytes());
}
