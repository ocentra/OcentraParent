use std::{fs::File, path::Path};

use super::super::BrowserManagedProfileStoreError;
pub(super) fn rename_guarded(
    source: &Path,
    target: &Path,
    file: &File,
    parent: &File,
) -> Result<(), BrowserManagedProfileStoreError> {
    let _ = (source, target, file, parent);
    // A retained `File` plus a post-rename identity check is insufficient on
    // Windows: `MoveFile`/`std::fs::rename` resolves names before the check,
    // so a substituted source can already have been moved.  The crate forbids
    // handwritten unsafe FFI and has no safe handle-relative rename owner.
    // Refuse the mutation rather than mutate first and report an audit-only
    // failure afterwards.
    Err(BrowserManagedProfileStoreError::UnsafePath)
}
