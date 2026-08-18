#![forbid(unsafe_code)]

use std::{fs, io, path::Path};

use atomicwrites::{AllowOverwrite, AtomicFile};
use chrono::{DateTime, FixedOffset};
use serde::Serialize;

use crate::entitlement_snapshot::SignedEntitlementSnapshot;

use super::{path, EntitlementSnapshotCacheError};

pub(crate) fn read_snapshot_file(
    path: &Path,
) -> Result<Option<SignedEntitlementSnapshot>, EntitlementSnapshotCacheError> {
    path::ensure_secure_path(path)?;
    let bytes = match fs::read(path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(_error) => return Err(EntitlementSnapshotCacheError::StorageUnavailable),
    };
    let snapshot: SignedEntitlementSnapshot = serde_json::from_slice(&bytes)
        .map_err(|_error| EntitlementSnapshotCacheError::CorruptState)?;
    snapshot
        .validate_shape()
        .map_err(|_error| EntitlementSnapshotCacheError::CorruptState)?;
    Ok(Some(snapshot))
}

pub(crate) fn enforce_snapshot_monotonicity(
    existing: &SignedEntitlementSnapshot,
    replacement: &SignedEntitlementSnapshot,
) -> Result<(), EntitlementSnapshotCacheError> {
    if replacement.authority_generation < existing.authority_generation
        || (replacement.authority_generation == existing.authority_generation
            && replacement != existing)
    {
        return Err(EntitlementSnapshotCacheError::StaleReplacement);
    }
    let existing_issued_at = parse_timestamp(&existing.issued_at)?;
    let replacement_issued_at = parse_timestamp(&replacement.issued_at)?;
    if replacement_issued_at < existing_issued_at
        || (replacement_issued_at == existing_issued_at && replacement != existing)
    {
        return Err(EntitlementSnapshotCacheError::StaleReplacement);
    }
    Ok(())
}

pub(crate) fn write_atomic<T: Serialize>(
    path: &Path,
    value: &T,
) -> Result<(), EntitlementSnapshotCacheError> {
    path::ensure_secure_path(path)?;
    AtomicFile::new(path, AllowOverwrite)
        .write(|file| {
            serde_json::to_writer(&mut *file, value).map_err(io::Error::other)?;
            file.sync_all()
        })
        .map_err(|_error| EntitlementSnapshotCacheError::StorageUnavailable)?;
    #[cfg(not(windows))]
    if let Some(parent) = path::non_empty_parent(path) {
        fs::File::open(parent)
            .and_then(|file| file.sync_all())
            .map_err(|_error| EntitlementSnapshotCacheError::StorageUnavailable)?;
    }
    Ok(())
}

fn parse_timestamp(value: &str) -> Result<DateTime<FixedOffset>, EntitlementSnapshotCacheError> {
    DateTime::parse_from_rfc3339(value)
        .map_err(|_error| EntitlementSnapshotCacheError::CorruptState)
}
