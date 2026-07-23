use std::{
    fs::{create_dir_all, File, OpenOptions},
    io::{self, Seek, SeekFrom},
    path::{Path, PathBuf},
};

#[cfg(not(windows))]
use std::io::Write;

use serde::Serialize;

use crate::{
    ndjson_append_rollback::{rollback_completed_append, rollback_partial_append},
    path::{date_stamp_now, sanitize_segment},
};

pub struct NdjsonWriter {
    root: PathBuf,
}

impl NdjsonWriter {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn append_event<T: Serialize>(
        &self,
        scope: &str,
        stream: &str,
        event: &T,
    ) -> io::Result<PathBuf> {
        let scope = sanitize_segment(scope)?;
        let stream = sanitize_segment(stream)?;
        let directory = self.root.join(scope).join("ndjson").join(stream);
        create_directory_hierarchy(&directory)?;
        let path = directory.join(format!("{}.ndjson", date_stamp_now()));
        let mut record = serde_json::to_vec(event)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
        record.push(b'\n');
        append_record(&path, &record)?;
        Ok(path)
    }

    pub fn append_event_for_operation<T: Serialize>(
        &self,
        scope: &str,
        stream: &str,
        operation_id: &str,
        event: &T,
    ) -> io::Result<PathBuf> {
        let scope = sanitize_segment(scope)?;
        let stream = sanitize_segment(stream)?;
        let directory = self.root.join(scope).join("ndjson").join(stream);
        create_directory_hierarchy(&directory)?;
        let current_path = directory.join(format!("{}.ndjson", date_stamp_now()));
        let mut record = serde_json::to_vec(event)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
        record.push(b'\n');
        crate::ndjson_record_validation::validate_record(&record)?;
        crate::ndjson_operation_route::append_routed_operation(
            &directory,
            &current_path,
            operation_id,
            &record,
        )
    }
}

pub fn append_record(path: &std::path::Path, record: &[u8]) -> io::Result<()> {
    crate::ndjson_record_validation::validate_record(record)?;
    if let Some(parent) = path.parent() {
        create_directory_hierarchy(parent)?;
    }
    crate::ndjson_operation_state_lock::with_stream_lock(path, || {
        let mut file = open_append_file(path)?;
        lock_and_append(&mut file, record)
    })
}

pub fn append_record_for_operation(
    path: &Path,
    operation_id: &str,
    record: &[u8],
) -> io::Result<()> {
    crate::ndjson_record_validation::validate_record(record)?;
    if operation_id.trim().is_empty() || operation_id.contains('\r') || operation_id.contains('\n')
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "operation id must be nonblank and contain no line breaks",
        ));
    }
    if let Some(parent) = path.parent() {
        create_directory_hierarchy(parent)?;
    }
    crate::ndjson_operation_state_lock::with_stream_lock(path, || {
        append_record_for_operation_stream_locked(path, operation_id, record)
    })
}

fn create_directory_hierarchy(path: &Path) -> io::Result<()> {
    create_directory_hierarchy_with_sync(path, crate::artifact_publish_platform::sync_parent)
}

fn create_directory_hierarchy_with_sync(
    path: &Path,
    mut sync_parent: impl FnMut(&Path) -> io::Result<()>,
) -> io::Result<()> {
    let mut missing = Vec::new();
    let mut current = path;
    while !current.exists() {
        missing.push(current.to_path_buf());
        let Some(parent) = current.parent() else {
            break;
        };
        current = parent;
    }
    create_dir_all(path)?;
    for directory in missing.iter().rev() {
        sync_parent(directory)?;
    }
    Ok(())
}

#[cfg(feature = "test-support")]
pub(crate) fn created_directory_parent_sync_count(path: &Path) -> io::Result<usize> {
    let mut count = 0;
    create_directory_hierarchy_with_sync(path, |_| {
        count += 1;
        Ok(())
    })?;
    Ok(count)
}

fn append_record_for_operation_stream_locked(
    path: &Path,
    operation_id: &str,
    record: &[u8],
) -> io::Result<()> {
    let mut file = open_append_file(path)?;
    file.lock()?;
    let result =
        crate::ndjson_operation::append_operation_locked(&mut file, path, operation_id, record);
    let unlock_result = file.unlock();
    result.and(unlock_result)
}

pub fn remove_record_file_with_operation_state(path: &Path) -> io::Result<()> {
    let directory = path.parent().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "NDJSON stream path has no parent directory",
        )
    })?;
    crate::ndjson_operation_route::with_route_lock(directory, || {
        crate::ndjson_operation_state_cleanup::remove_operation_state(path)
    })
}

fn lock_and_append(file: &mut File, record: &[u8]) -> io::Result<()> {
    file.lock()?;
    let result = append_locked_record(file, record);
    let unlock_result = file.unlock();
    result.and(unlock_result)
}

fn append_locked_record(file: &mut File, record: &[u8]) -> io::Result<()> {
    append_locked_record_with_sync(file, record, File::sync_data)
}

pub(crate) fn append_locked_record_with_sync(
    file: &mut File,
    record: &[u8],
    sync: impl FnOnce(&File) -> io::Result<()>,
) -> io::Result<()> {
    crate::ndjson_record_validation::validate_record(record)?;
    recover_partial_tail(file)?;
    file.seek(SeekFrom::End(0))?;
    let committed_offset = match append_bytes(file, record) {
        Ok(offset) => offset,
        Err(error) => return rollback_partial_append(file, error),
    };
    if let Err(error) = sync(file) {
        return rollback_completed_append(file, committed_offset, record, error);
    }
    Ok(())
}

pub(crate) fn open_append_file(path: &Path) -> io::Result<File> {
    let mut options = OpenOptions::new();
    options.read(true).create(true);
    #[cfg(windows)]
    options.write(true);
    #[cfg(not(windows))]
    options.append(true);
    options.open(path)
}

#[cfg(windows)]
pub(crate) fn append_bytes(file: &File, bytes: &[u8]) -> io::Result<u64> {
    use std::os::windows::fs::FileExt;

    let mut written = 0;
    while written < bytes.len() {
        let count = file.seek_write(&bytes[written..], u64::MAX)?;
        if count == 0 {
            return Err(io::Error::new(
                io::ErrorKind::WriteZero,
                "failed to append the complete NDJSON record",
            ));
        }
        written += count;
    }
    file.metadata()?
        .len()
        .checked_sub(bytes.len() as u64)
        .ok_or_else(|| io::Error::other("NDJSON append offset underflow"))
}

#[cfg(not(windows))]
pub(crate) fn append_bytes(file: &mut File, bytes: &[u8]) -> io::Result<u64> {
    file.write_all(bytes)?;
    file.stream_position()?
        .checked_sub(bytes.len() as u64)
        .ok_or_else(|| io::Error::other("NDJSON append offset underflow"))
}

pub(crate) fn recover_partial_tail(file: &mut File) -> io::Result<()> {
    crate::ndjson_tail_recovery::recover_partial_tail(file)
}
