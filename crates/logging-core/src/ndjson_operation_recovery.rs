use std::{fs::File, io, path::Path};

use crate::{
    ndjson_operation_compaction::compacted_offset,
    ndjson_operation_marker_state::{operation_directory, operation_paths},
    ndjson_tail_recovery::has_complete_tail,
    ndjson_writer::recover_partial_tail,
};

pub(crate) fn prepare_uncommitted_tail(file: &mut File, has_intent: bool) -> io::Result<()> {
    if has_intent {
        return recover_partial_tail(file);
    }
    if has_complete_tail(file)? {
        return Ok(());
    }
    Err(io::Error::new(
        io::ErrorKind::WouldBlock,
        "NDJSON tail is incomplete without a matching operation intent",
    ))
}

pub(crate) fn operation_state_exists(
    path: &Path,
    operation_id: &str,
    record: &[u8],
) -> io::Result<bool> {
    let directory = operation_directory(path)?;
    if !directory.exists() {
        return Ok(false);
    }
    let operation = operation_paths(path, operation_id)?;
    if operation.intent.exists() || operation.commit.exists() {
        return Ok(true);
    }
    Ok(compacted_offset(&operation, operation_id, record)?.is_some())
}
