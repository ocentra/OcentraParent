use std::{fs::File, path::Path};

use super::super::BrowserManagedProfileStoreError;
pub(super) fn remove_directory_tree(
    path: &Path,
    file: &File,
) -> Result<(), BrowserManagedProfileStoreError> {
    let _ = (path, file);
    // `read_dir` + `remove_dir/remove_file` is a name-based recursive delete.
    // FILE_SHARE_DELETE and retained handles do not make those names refer to
    // the same objects at the destructive call.  Without a safe
    // handle-relative Windows delete owner, fail closed; never delete a
    // substituted profile or a reparse target.
    Err(BrowserManagedProfileStoreError::UnsafePath)
}
