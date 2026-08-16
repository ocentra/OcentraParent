use std::{
    fs::{read, remove_file, rename},
    io,
    path::Path,
};

use crate::{
    artifact_publish_lock::remove_temporary, artifact_publish_platform::sync_parent,
    ndjson_operation_marker_publish::remove_failed_marker,
    ndjson_operation_marker_state::marker_is_complete,
};

pub(crate) fn publish_marker(temporary: &Path, path: &Path, content: &[u8]) -> io::Result<()> {
    match rename(temporary, path) {
        Ok(()) => sync_parent(path),
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
            remove_temporary(temporary)?;
            if read(path)? == content {
                Ok(())
            } else {
                Err(error)
            }
        }
        Err(error) => remove_failed_marker(temporary, error),
    }
}

pub(crate) fn accept_or_repair_existing(path: &Path, content: &[u8]) -> io::Result<bool> {
    match read(path) {
        Ok(existing) if existing == content => Ok(true),
        Ok(existing) if marker_is_complete(&existing) => Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "operation marker conflicts with existing complete marker",
        )),
        Ok(_) => {
            remove_file(path)?;
            Ok(false)
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(error),
    }
}
