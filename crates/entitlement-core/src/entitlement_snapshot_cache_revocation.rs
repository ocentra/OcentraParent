#![forbid(unsafe_code)]

use std::path::PathBuf;

use crate::entitlement_snapshot_cache::{
    path, storage, EntitlementSnapshotCacheError, SignedEntitlementRevocationUpdate,
};

#[derive(Clone, Debug)]
pub(crate) struct EntitlementRevocationStateStore {
    path: PathBuf,
}

impl EntitlementRevocationStateStore {
    pub(crate) fn open(path: impl Into<PathBuf>) -> Result<Self, EntitlementSnapshotCacheError> {
        let path = path.into();
        path::prepare_path(&path)?;
        Ok(Self { path })
    }

    pub(crate) fn read_signed(
        &self,
    ) -> Result<Option<SignedEntitlementRevocationUpdate>, EntitlementSnapshotCacheError> {
        read_revocation_file(&self.path)
    }

    pub(crate) fn replace_signed(
        &self,
        update: &SignedEntitlementRevocationUpdate,
    ) -> Result<(), EntitlementSnapshotCacheError> {
        update.validate_shape()?;
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
        let result = (|| {
            if let Some(existing) = read_revocation_file(&self.path)? {
                enforce_revocation_monotonicity(&existing, update)?;
            }
            storage::write_atomic(&self.path, update)
        })();
        let unlock_result = fs2::FileExt::unlock(&lock)
            .map_err(|_error| EntitlementSnapshotCacheError::StorageUnavailable);
        match (result, unlock_result) {
            (Ok(value), Ok(())) => Ok(value),
            (Err(error), _) => Err(error),
            (Ok(_), Err(error)) => Err(error),
        }
    }
}

fn read_revocation_file(
    path: &std::path::Path,
) -> Result<Option<SignedEntitlementRevocationUpdate>, EntitlementSnapshotCacheError> {
    path::ensure_secure_path(path)?;
    let bytes = match std::fs::read(path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(_error) => return Err(EntitlementSnapshotCacheError::StorageUnavailable),
    };
    let update: SignedEntitlementRevocationUpdate = serde_json::from_slice(&bytes)
        .map_err(|_error| EntitlementSnapshotCacheError::CorruptState)?;
    update.validate_shape()?;
    Ok(Some(update))
}

fn enforce_revocation_monotonicity(
    existing: &SignedEntitlementRevocationUpdate,
    replacement: &SignedEntitlementRevocationUpdate,
) -> Result<(), EntitlementSnapshotCacheError> {
    if replacement.authority_generation < existing.authority_generation
        || (replacement.authority_generation == existing.authority_generation
            && replacement != existing)
    {
        return Err(EntitlementSnapshotCacheError::StaleReplacement);
    }
    if replacement.authority_generation > existing.authority_generation
        && existing.revoked_snapshot_ids.iter().any(|snapshot_id| {
            !replacement
                .revoked_snapshot_ids
                .iter()
                .any(|replacement_id| replacement_id == snapshot_id)
        })
    {
        return Err(EntitlementSnapshotCacheError::StaleReplacement);
    }
    Ok(())
}
