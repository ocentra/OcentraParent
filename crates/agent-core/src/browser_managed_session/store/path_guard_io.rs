use std::{fs, io::ErrorKind, path::Path};

use super::super::BrowserManagedProfileStoreError;
use super::path_guard::StablePathGuard;
use super::path_guards::GuardedPathKind;
use super::path_guards_platform::{metadata_is_indirection, open_guarded};

pub(super) fn open_optional(
    path: &Path,
    kind: GuardedPathKind,
    deny_delete: bool,
) -> Result<Option<StablePathGuard>, BrowserManagedProfileStoreError> {
    match fs::symlink_metadata(path) {
        Ok(metadata) => {
            if metadata_is_indirection(&metadata) {
                return Err(BrowserManagedProfileStoreError::UnsafePath);
            }
            let file = open_guarded(path, kind, false, deny_delete)?;
            StablePathGuard::from_file(path, file, kind).map(Some)
        }
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(None),
        Err(_error) => Err(BrowserManagedProfileStoreError::Io),
    }
}

pub(super) fn open_or_create(
    path: &Path,
    deny_delete: bool,
) -> Result<StablePathGuard, BrowserManagedProfileStoreError> {
    reject_indirection(path)?;
    let file = open_guarded(path, GuardedPathKind::File, true, deny_delete)?;
    StablePathGuard::from_file(path, file, GuardedPathKind::File)
}

pub(super) fn reject_indirection(path: &Path) -> Result<(), BrowserManagedProfileStoreError> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata_is_indirection(&metadata) => {
            Err(BrowserManagedProfileStoreError::UnsafePath)
        }
        Ok(_) => Ok(()),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(()),
        Err(_error) => Err(BrowserManagedProfileStoreError::Io),
    }
}
