use std::{fs, io::ErrorKind, path::Path};

use super::super::BrowserManagedProfileStoreError;
use super::path_guard::StablePathGuard;
use super::path_guards::GuardedPathKind;
use super::path_guards_platform::open_guarded_io;

pub(super) fn open_or_create_guard(
    current: &Path,
) -> Result<StablePathGuard, BrowserManagedProfileStoreError> {
    match open_guarded_io(current, GuardedPathKind::Directory, false, true) {
        Ok(file) => StablePathGuard::from_file(current, file, GuardedPathKind::Directory),
        Err(error) if error.kind() == ErrorKind::NotFound => {
            create_directory(current)?;
            let file = open_guarded_io(current, GuardedPathKind::Directory, false, true)
                .map_err(|_error| BrowserManagedProfileStoreError::Io)?;
            StablePathGuard::from_file(current, file, GuardedPathKind::Directory)
        }
        Err(_error) => Err(BrowserManagedProfileStoreError::Io),
    }
}

fn create_directory(path: &Path) -> Result<(), BrowserManagedProfileStoreError> {
    match fs::create_dir(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == ErrorKind::AlreadyExists => Ok(()),
        Err(_error) => Err(BrowserManagedProfileStoreError::Io),
    }
}
