use std::{
    fs::{read_dir, OpenOptions},
    io,
    path::{Path, PathBuf},
};

use crate::ndjson_writer::append_record_for_operation;

pub(crate) fn append_routed_operation(
    directory: &Path,
    current_path: &Path,
    operation_id: &str,
    record: &[u8],
) -> io::Result<PathBuf> {
    with_route_lock(directory, || {
        let mut paths = daily_stream_paths(directory)?;
        paths.sort();
        for path in paths {
            if crate::ndjson_operation_recovery::operation_state_exists(
                &path,
                operation_id,
                record,
            )? {
                append_record_for_operation(&path, operation_id, record)?;
                return Ok(path);
            }
        }
        append_record_for_operation(current_path, operation_id, record)?;
        Ok(current_path.to_owned())
    })
}

pub(crate) fn with_route_lock<T>(
    directory: &Path,
    operation: impl FnOnce() -> io::Result<T>,
) -> io::Result<T> {
    let lock_path = directory.join(".operation-route.lock");
    let lock = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(lock_path)?;
    lock.lock()?;
    let result = operation();
    let unlock = lock.unlock();
    match result {
        Ok(value) => unlock.map(|()| value),
        Err(error) => {
            let _ = unlock;
            Err(error)
        }
    }
}

fn daily_stream_paths(directory: &Path) -> io::Result<Vec<PathBuf>> {
    read_dir(directory)?
        .filter_map(|entry| match entry {
            Ok(entry) if is_daily_stream(&entry.path()) => Some(Ok(entry.path())),
            Ok(_) => None,
            Err(error) => Some(Err(error)),
        })
        .collect()
}

fn is_daily_stream(path: &Path) -> bool {
    path.extension().and_then(|extension| extension.to_str()) == Some("ndjson")
}
