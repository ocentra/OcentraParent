use std::{fs::remove_dir_all, io, path::Path};

use crate::{
    artifact_publish_lock::remove_temporary, artifact_publish_platform::sync_parent,
    ndjson_operation_compaction_cache::forget_commit_index,
    ndjson_operation_marker_state::operation_directory,
    ndjson_operation_state_lock::with_stream_lock,
};

pub(crate) fn remove_operation_state(path: &Path) -> io::Result<()> {
    with_stream_lock(path, || remove_operation_state_locked(path))
}

fn remove_operation_state_locked(path: &Path) -> io::Result<()> {
    let directory = operation_directory(path)?;
    if path.exists() {
        sync_parent(path)?;
    }
    remove_temporary(path)?;
    sync_parent(path)?;
    forget_commit_index(&directory.join("commits.state"))?;
    match remove_dir_all(&directory) {
        Ok(()) => sync_parent(&directory)?,
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
