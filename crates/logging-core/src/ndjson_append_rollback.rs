use std::{fs::File, io};

use crate::{
    ndjson_operation_marker::record_matches_at, ndjson_tail_recovery::recover_partial_tail,
};

pub(crate) fn rollback_partial_append(file: &mut File, original: io::Error) -> io::Result<()> {
    match recover_partial_tail(file).and_then(|_| file.sync_data()) {
        Ok(()) => Err(original),
        Err(rollback) => rollback_failure(&original, &rollback),
    }
}

pub(crate) fn rollback_completed_append(
    file: &mut File,
    offset: u64,
    record: &[u8],
    original: io::Error,
) -> io::Result<()> {
    let record_end = offset
        .checked_add(record.len() as u64)
        .ok_or_else(|| io::Error::other("NDJSON rollback offset overflow"))?;
    if file.metadata()?.len() != record_end || !record_matches_at(file, offset, record)? {
        return Err(original);
    }
    match file.set_len(offset).and_then(|_| file.sync_data()) {
        Ok(()) => Err(original),
        Err(rollback) => rollback_failure(&original, &rollback),
    }
}

fn rollback_failure(original: &io::Error, rollback: &io::Error) -> io::Result<()> {
    Err(io::Error::new(
        rollback.kind(),
        format!("NDJSON append failed ({original}); rollback also failed ({rollback})"),
    ))
}
