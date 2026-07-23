use std::{
    fs::{remove_file, File},
    io::{self, Seek, SeekFrom, Write},
    path::Path,
};

use crate::ndjson_operation_compaction::{compact_commit, compacted_offset};
use crate::ndjson_operation_marker::{marker_content, record_matches_at};
use crate::ndjson_operation_marker_publish::write_marker;
use crate::ndjson_operation_marker_state::{
    operation_paths, read_commit_offset, read_intent_offset,
};
use crate::ndjson_writer::{recover_partial_tail, rollback_append, validate_record};

pub(crate) fn append_operation_locked(
    file: &mut File,
    path: &Path,
    operation_id: &str,
    record: &[u8],
) -> io::Result<()> {
    append_operation_with_fault(file, path, operation_id, record, |_| Ok(()))
}

pub(crate) fn append_operation_with_fault<F>(
    file: &mut File,
    path: &Path,
    operation_id: &str,
    record: &[u8],
    before: F,
) -> io::Result<()>
where
    F: FnMut(FaultPoint) -> io::Result<()>,
{
    append_operation_with_hooks(
        file,
        path,
        operation_id,
        record,
        before,
        |path, content, _| write_marker(path, content),
    )
}

pub(crate) fn append_operation_with_hooks<F, M>(
    file: &mut File,
    path: &Path,
    operation_id: &str,
    record: &[u8],
    mut before: F,
    mut write_operation_marker: M,
) -> io::Result<()>
where
    F: FnMut(FaultPoint) -> io::Result<()>,
    M: FnMut(&Path, &str, OperationMarkerKind) -> io::Result<()>,
{
    validate_record(record)?;
    recover_partial_tail(file)?;
    let operation = operation_paths(path, operation_id)?;
    if let Some(offset) = read_commit_offset(&operation.commit, operation_id, record)? {
        verify_committed_record(file, offset, record)?;
        compact_commit(&operation, operation_id, record)?;
        remove_intent(&operation.intent)?;
        return Ok(());
    }
    if let Some(offset) = compacted_offset(&operation, operation_id, record)? {
        verify_committed_record(file, offset, record)?;
        remove_intent(&operation.intent)?;
        return Ok(());
    }

    let offset = match read_intent_offset(&operation.intent, operation_id, record)? {
        Some(offset) if record_matches_at(file, offset, record)? => {
            before(FaultPoint::Sync)?;
            file.sync_data()?;
            write_operation_marker(
                &operation.commit,
                &marker_content(operation_id, record, offset),
                OperationMarkerKind::Commit,
            )?;
            compact_commit(&operation, operation_id, record)?;
            remove_intent(&operation.intent)?;
            return Ok(());
        }
        Some(_) => {
            remove_intent(&operation.intent)?;
            file.seek(SeekFrom::End(0))?
        }
        None => file.seek(SeekFrom::End(0))?,
    };

    write_operation_marker(
        &operation.intent,
        &marker_content(operation_id, record, offset),
        OperationMarkerKind::Intent,
    )?;
    if let Err(error) = before(FaultPoint::Write).and_then(|_| file.write_all(record)) {
        remove_intent(&operation.intent)?;
        return rollback_append(file, offset, error);
    }
    before(FaultPoint::Sync)?;
    file.sync_data()?;
    write_operation_marker(
        &operation.commit,
        &marker_content(operation_id, record, offset),
        OperationMarkerKind::Commit,
    )?;
    compact_commit(&operation, operation_id, record)?;
    remove_intent(&operation.intent)
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum OperationMarkerKind {
    Intent,
    Commit,
}

#[derive(Clone, Copy)]
pub(crate) enum FaultPoint {
    Write,
    Sync,
}

fn remove_intent(path: &Path) -> io::Result<()> {
    match remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error),
    }
}

fn verify_committed_record(file: &mut File, offset: u64, record: &[u8]) -> io::Result<()> {
    if record_matches_at(file, offset, record)? {
        return Ok(());
    }
    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "committed operation record is missing or corrupted",
    ))
}
