use std::{fs, fs::File, path::Path};

use super::super::BrowserManagedProfileStoreError;

pub(super) fn rename_guarded(
    source: &Path,
    target: &Path,
    file: &File,
    parent: &File,
) -> Result<(), BrowserManagedProfileStoreError> {
    let _ = (file, parent);
    fs::rename(source, target).map_err(|_error| BrowserManagedProfileStoreError::Io)
}

pub(super) fn remove_directory_tree(
    path: &Path,
    file: &File,
) -> Result<(), BrowserManagedProfileStoreError> {
    let _ = file;
    fs::remove_dir_all(path).map_err(|_error| BrowserManagedProfileStoreError::Io)
}
