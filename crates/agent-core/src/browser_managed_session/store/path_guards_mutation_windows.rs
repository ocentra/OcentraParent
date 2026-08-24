use std::{fs, fs::File, path::Path};

use super::super::BrowserManagedProfileStoreError;
use super::path_guards::GuardedPathKind;
use super::path_guards_platform::{metadata_is_indirection, open_guarded, stable_file_identity};

pub(super) fn rename_guarded(
    source: &Path,
    target: &Path,
    file: &File,
    parent: &File,
) -> Result<(), BrowserManagedProfileStoreError> {
    // `file` and `parent` are retained by the caller for the full operation.
    // Both were opened with OPEN_REPARSE_POINT and the source identity is
    // checked immediately before this atomic rename. Re-open the destination
    // without following a reparse point and compare it to the still-open
    // source handle after the rename; a substitution fails closed.
    // The parent borrow ties this operation to the still-open root guard.
    let _ = parent;
    match fs::symlink_metadata(target) {
        Ok(_) => return Err(BrowserManagedProfileStoreError::UnsafePath),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(_error) => return Err(BrowserManagedProfileStoreError::Io),
    }
    fs::rename(source, target).map_err(|_error| BrowserManagedProfileStoreError::Io)?;
    let target_file = open_guarded(target, GuardedPathKind::Directory, false, true)?;
    let target_metadata = target_file
        .metadata()
        .map_err(|_error| BrowserManagedProfileStoreError::Io)?;
    let source_metadata = file
        .metadata()
        .map_err(|_error| BrowserManagedProfileStoreError::Io)?;
    if metadata_is_indirection(&target_metadata)
        || stable_file_identity(&target_file, &target_metadata)?
            != stable_file_identity(file, &source_metadata)?
    {
        return Err(BrowserManagedProfileStoreError::UnsafePath);
    }
    Ok(())
}
