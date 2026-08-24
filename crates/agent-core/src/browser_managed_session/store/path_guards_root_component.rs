use std::{io::ErrorKind, path::Path};

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
    let _ = path;
    // There is no handle-relative, no-follow directory-create primitive in
    // this crate's safe Rust boundary.  A name-based create here would make
    // the retained root guard non-authoritative under a junction/symlink
    // substitution race.  Refuse the operation instead of creating an
    // unowned root and pretending that later validation closes the race.
    Err(BrowserManagedProfileStoreError::UnsafePath)
}
