use std::fs;
#[cfg(unix)]
use std::fs::File;
use std::io::ErrorKind;
use std::path::Path;

use crate::parent_presence_store::ParentPresenceStoreError;
use crate::parent_presence_store_file_temporary::reserve_private_temporary_artifact;

pub(crate) fn publish_initialized_store_if_absent(
    path: &Path,
    initialize: impl FnOnce(&Path) -> Result<(), ParentPresenceStoreError>,
) -> Result<(), ParentPresenceStoreError> {
    match fs::symlink_metadata(path) {
        Ok(_metadata) => return Ok(()),
        Err(error) if error.kind() == ErrorKind::NotFound => {}
        Err(_error) => return Err(ParentPresenceStoreError::Unavailable),
    }

    let temporary = reserve_private_temporary_artifact(path)?;
    initialize(temporary.path())?;
    temporary.validate_path_identity()?;
    temporary.sync_all()?;

    match fs::hard_link(temporary.path(), path) {
        Ok(()) => {}
        Err(error) if error.kind() == ErrorKind::AlreadyExists => {}
        Err(_error) => return Err(ParentPresenceStoreError::Unavailable),
    }

    drop(temporary);
    sync_parent_directory(path)
}

#[cfg(unix)]
fn sync_parent_directory(path: &Path) -> Result<(), ParentPresenceStoreError> {
    let parent = path.parent().ok_or(ParentPresenceStoreError::Unavailable)?;
    File::open(parent)
        .and_then(|directory| directory.sync_all())
        .map_err(|_error| ParentPresenceStoreError::Unavailable)
}

#[cfg(windows)]
fn sync_parent_directory(_path: &Path) -> Result<(), ParentPresenceStoreError> {
    Ok(())
}
