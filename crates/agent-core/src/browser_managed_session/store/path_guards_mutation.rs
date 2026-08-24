use std::{fs::File, path::Path};

use super::super::BrowserManagedProfileStoreError;

pub(super) fn rename_guarded(
    source: &Path,
    target: &Path,
    file: &File,
    parent: &File,
) -> Result<(), BrowserManagedProfileStoreError> {
    #[cfg(windows)]
    {
        super::path_guards_mutation_windows::rename_guarded(source, target, file, parent)
    }
    #[cfg(not(windows))]
    {
        super::path_guards_mutation_portable::rename_guarded(source, target, file, parent)
    }
}

pub(super) fn remove_directory_tree(
    path: &Path,
    file: &File,
) -> Result<(), BrowserManagedProfileStoreError> {
    #[cfg(windows)]
    {
        super::path_guards_mutation_windows_delete::remove_directory_tree(path, file)
    }
    #[cfg(not(windows))]
    {
        super::path_guards_mutation_portable::remove_directory_tree(path, file)
    }
}
