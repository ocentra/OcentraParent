#![forbid(unsafe_code)]

use std::path::PathBuf;

use crate::entitlement_snapshot_cache::{
    path, EntitlementSnapshotCacheError, SignedEntitlementRevocationUpdate,
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
