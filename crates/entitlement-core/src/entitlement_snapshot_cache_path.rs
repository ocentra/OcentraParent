#![forbid(unsafe_code)]

use std::{fs, io, path::Path};

use super::EntitlementSnapshotCacheError;

pub(crate) fn prepare_path(path: &Path) -> Result<(), EntitlementSnapshotCacheError> {
    ensure_secure_path(path)?;
    if let Some(parent) = non_empty_parent(path) {
        fs::create_dir_all(parent)
            .map_err(|_error| EntitlementSnapshotCacheError::StorageUnavailable)?;
    }
    ensure_secure_path(path)?;
    Ok(())
}

pub(crate) fn ensure_secure_path(path: &Path) -> Result<(), EntitlementSnapshotCacheError> {
    let mut current = Some(path);
    while let Some(candidate) = current {
        match fs::symlink_metadata(candidate) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                return Err(EntitlementSnapshotCacheError::InvalidPath)
            }
            Ok(_) => {}
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(_error) => return Err(EntitlementSnapshotCacheError::StorageUnavailable),
        }
        current = candidate.parent();
    }
    Ok(())
}

pub(crate) fn non_empty_parent(path: &Path) -> Option<&Path> {
    path.parent()
        .filter(|parent| !parent.as_os_str().is_empty())
}
