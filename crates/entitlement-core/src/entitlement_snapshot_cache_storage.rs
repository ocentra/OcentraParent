#![forbid(unsafe_code)]

use std::{fs, io, path::Path};

use atomicwrites::{AllowOverwrite, AtomicFile};
use serde::Serialize;

use super::{path, EntitlementSnapshotCacheError};

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
