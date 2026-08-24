use std::{fs, fs::File, path::Path};

use super::super::BrowserManagedProfileStoreError;
use super::path_guard::StablePathGuard;
use super::path_guards::GuardedPathKind;
use super::path_guards_platform::metadata_is_indirection;

pub(super) fn remove_directory_tree(
    path: &Path,
    file: &File,
) -> Result<(), BrowserManagedProfileStoreError> {
    for entry in fs::read_dir(path).map_err(|_error| BrowserManagedProfileStoreError::Io)? {
        let entry = entry.map_err(|_error| BrowserManagedProfileStoreError::Io)?;
        let child_path = entry.path();
        let metadata = fs::symlink_metadata(&child_path)
            .map_err(|_error| BrowserManagedProfileStoreError::Io)?;
        if metadata_is_indirection(&metadata) {
            return Err(BrowserManagedProfileStoreError::UnsafePath);
        }
        let child_kind = if metadata.is_dir() {
            GuardedPathKind::Directory
        } else if metadata.is_file() {
            GuardedPathKind::File
        } else {
            return Err(BrowserManagedProfileStoreError::UnsafePath);
        };
        let child = StablePathGuard::open_for_destructive_operation(&child_path, child_kind)?;
        let child_metadata = child
            .file
            .metadata()
            .map_err(|_error| BrowserManagedProfileStoreError::Io)?;
        if !child_kind.matches(&child_metadata) || metadata_is_indirection(&child_metadata) {
            return Err(BrowserManagedProfileStoreError::UnsafePath);
        }
        if matches!(child_kind, GuardedPathKind::Directory) {
            remove_directory_tree(&child_path, &child.file)?;
        }
        remove_entry(&child_path, child_kind, &child.file)?;
    }
    remove_entry(path, GuardedPathKind::Directory, file)
}

fn remove_entry(
    path: &Path,
    kind: GuardedPathKind,
    file: &File,
) -> Result<(), BrowserManagedProfileStoreError> {
    // Keep the already-open guarded handle alive until the path operation
    // returns; dropping it before remove would reopen the substitution race.
    let _ = file;
    match kind {
        GuardedPathKind::Directory => {
            fs::remove_dir(path).map_err(|_error| BrowserManagedProfileStoreError::Io)
        }
        GuardedPathKind::File => {
            fs::remove_file(path).map_err(|_error| BrowserManagedProfileStoreError::Io)
        }
    }
}
