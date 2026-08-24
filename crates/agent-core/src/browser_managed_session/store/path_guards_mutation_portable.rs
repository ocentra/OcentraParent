use std::{fs::File, path::Path};

use super::super::BrowserManagedProfileStoreError;

pub(super) fn rename_guarded(
    source: &Path,
    target: &Path,
    file: &File,
    parent: &File,
) -> Result<(), BrowserManagedProfileStoreError> {
    let _ = (source, target, file, parent);
    // `std::fs::rename` is name-based.  Reopening and comparing a retained
    // handle before it does not make the rename identity-atomic on portable
    // platforms, so this boundary must fail closed until a handle-relative
    // primitive is owned by the crate.
    Err(BrowserManagedProfileStoreError::UnsafePath)
}

pub(super) fn remove_directory_tree(
    path: &Path,
    file: &File,
) -> Result<(), BrowserManagedProfileStoreError> {
    let _ = (path, file);
    // `remove_dir_all` recursively follows names after the guard check and
    // cannot preserve exact object identity through each destructive effect.
    Err(BrowserManagedProfileStoreError::UnsafePath)
}
