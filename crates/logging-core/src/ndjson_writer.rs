use std::{
    fs::{create_dir_all, File, OpenOptions},
    io::{self, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

use serde::Serialize;

use crate::path::{date_stamp_now, sanitize_segment};

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
        create_dir_all(&directory)?;
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
        create_dir_all(&directory)?;
        let path = directory.join(format!("{}.ndjson", date_stamp_now()));
        let mut record = serde_json::to_vec(event)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
        record.push(b'\n');
        append_record_for_operation(&path, operation_id, &record)?;
        Ok(path)
    }
}

pub fn append_record(path: &std::path::Path, record: &[u8]) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(path)?;
    lock_and_append(&mut file, record)
}

pub fn append_record_for_operation(
    path: &Path,
    operation_id: &str,
    record: &[u8],
) -> io::Result<()> {
    if operation_id.trim().is_empty() || operation_id.contains('\r') || operation_id.contains('\n')
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "operation id must be nonblank and contain no line breaks",
        ));
    }
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(path)?;
    file.lock()?;
    let result =
        crate::ndjson_operation::append_operation_locked(&mut file, path, operation_id, record);
    let unlock_result = file.unlock();
    result.and(unlock_result)
}

pub fn remove_record_file_with_operation_state(path: &Path) -> io::Result<()> {
    crate::ndjson_operation_state_cleanup::remove_operation_state(path)
}

fn lock_and_append(file: &mut File, record: &[u8]) -> io::Result<()> {
    file.lock()?;
    let result = append_locked_record(file, record);
    let unlock_result = file.unlock();
    result.and(unlock_result)
}

fn append_locked_record(file: &mut File, record: &[u8]) -> io::Result<()> {
    if record.is_empty() || record.last() != Some(&b'\n') {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "NDJSON records must end with a newline",
        ));
    }

    recover_partial_tail(file)?;
    let start = file.seek(SeekFrom::End(0))?;
    if let Err(error) = file.write_all(record) {
        return rollback_append(file, start, error);
    }
    file.sync_data()?;
    Ok(())
}

pub(crate) fn validate_record(record: &[u8]) -> io::Result<()> {
    if record.is_empty() || record.last() != Some(&b'\n') {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "NDJSON records must end with a newline",
        ));
    }
    Ok(())
}

pub(crate) fn recover_partial_tail(file: &mut File) -> io::Result<()> {
    crate::ndjson_tail_recovery::recover_partial_tail(file)
}

pub(crate) fn rollback_append(file: &File, start: u64, original: io::Error) -> io::Result<()> {
    match file.set_len(start).and_then(|_| file.sync_data()) {
        Ok(()) => Err(original),
        Err(rollback) => Err(io::Error::new(
            rollback.kind(),
            format!("NDJSON append failed ({original}); rollback also failed ({rollback})"),
        )),
    }
}
