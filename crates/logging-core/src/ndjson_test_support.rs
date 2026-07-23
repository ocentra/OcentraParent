use std::{
    fs::create_dir_all,
    io::{self, Cursor, Read, Seek, SeekFrom},
    path::Path,
};

use crate::ndjson_operation::{append_operation_with_fault, FaultPoint};
use crate::ndjson_operation_fault::{
    append_operation_with_marker_fault, OperationMarkerFault as InternalOperationMarkerFault,
};

#[derive(Clone, Copy)]
pub enum AppendFault {
    Write,
    Sync,
}

#[derive(Clone, Copy)]
pub enum OperationMarkerFault {
    IntentWrite,
    IntentSync,
    CommitWrite,
    CommitSync,
}

pub fn append_record_with_fault(
    path: &Path,
    operation_id: &str,
    record: &[u8],
    fault: AppendFault,
) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    let mut file = crate::ndjson_writer::open_append_file(path)?;
    file.lock()?;
    let result = append_operation_with_fault(&mut file, path, operation_id, record, |point| {
        if fault_matches(fault, point) {
            return Err(io::Error::other("injected NDJSON append failure"));
        }
        Ok(())
    });
    let unlock_result = file.unlock();
    result.and(unlock_result)
}

pub fn append_plain_record_with_sync_fault(path: &Path, record: &[u8]) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    let mut file = crate::ndjson_writer::open_append_file(path)?;
    file.lock()?;
    let result = crate::ndjson_writer::append_locked_record_with_sync(&mut file, record, |_| {
        Err(io::Error::other("injected NDJSON sync failure"))
    });
    let unlock_result = file.unlock();
    result.and(unlock_result)
}

pub fn append_record_with_marker_fault(
    path: &Path,
    operation_id: &str,
    record: &[u8],
    fault: OperationMarkerFault,
) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    let mut file = crate::ndjson_writer::open_append_file(path)?;
    file.lock()?;
    let fault = match fault {
        OperationMarkerFault::IntentWrite => InternalOperationMarkerFault::IntentWrite,
        OperationMarkerFault::IntentSync => InternalOperationMarkerFault::IntentSync,
        OperationMarkerFault::CommitWrite => InternalOperationMarkerFault::CommitWrite,
        OperationMarkerFault::CommitSync => InternalOperationMarkerFault::CommitSync,
    };
    let result = append_operation_with_marker_fault(&mut file, path, operation_id, record, fault);
    let unlock_result = file.unlock();
    result.and(unlock_result)
}

pub fn record_matches_with_short_reads(record: &[u8]) -> io::Result<bool> {
    let cursor = Cursor::new(record.to_vec());
    let mut reader = OneByteReader { inner: cursor };
    crate::ndjson_operation_marker::record_matches_at(&mut reader, 0, record)
}

pub fn operation_state_entry_count(path: &Path) -> io::Result<usize> {
    crate::ndjson_operation_state_cleanup::operation_state_entry_count(path)
}

pub fn forget_operation_compaction_cache(path: &Path) -> io::Result<()> {
    let directory = crate::ndjson_operation_marker_state::operation_directory(path)?;
    crate::ndjson_operation_compaction_cache::forget_commit_index(&directory.join("commits.ndjson"))
}

pub fn operation_compaction_scan_bytes(path: &Path) -> io::Result<u64> {
    let directory = crate::ndjson_operation_marker_state::operation_directory(path)?;
    crate::ndjson_operation_compaction_cache::scanned_bytes(&directory.join("commits.ndjson"))
}

struct OneByteReader<R> {
    inner: R,
}

impl<R: Read> Read for OneByteReader<R> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        let length = buffer.len().min(1);
        self.inner.read(&mut buffer[..length])
    }
}

impl<R: Seek> Seek for OneByteReader<R> {
    fn seek(&mut self, position: SeekFrom) -> io::Result<u64> {
        self.inner.seek(position)
    }
}

fn fault_matches(fault: AppendFault, point: FaultPoint) -> bool {
    matches!(
        (fault, point),
        (AppendFault::Write, FaultPoint::Write) | (AppendFault::Sync, FaultPoint::Sync)
    )
}
