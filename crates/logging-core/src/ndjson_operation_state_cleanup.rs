use std::{fs::remove_dir_all, io, path::Path};

use crate::{
    artifact_publish_lock::remove_temporary, artifact_publish_platform::sync_parent,
    ndjson_operation_marker_state::operation_directory,
};

pub(crate) fn remove_operation_state(path: &Path) -> io::Result<()> {
    if path.exists() {
        sync_parent(path)?;
    }
    remove_temporary(path)?;
    let directory = operation_directory(path)?;
    match remove_dir_all(directory) {
        Ok(()) => {}
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => return Err(error),
    }
    Ok(())
}

#[cfg(feature = "test-support")]
pub(crate) fn operation_state_entry_count(path: &Path) -> io::Result<usize> {
    let directory = operation_directory(path)?;
    match std::fs::read_dir(directory) {
        Ok(entries) => Ok(entries.filter_map(Result::ok).count()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(0),
        Err(error) => Err(error),
    }
}
