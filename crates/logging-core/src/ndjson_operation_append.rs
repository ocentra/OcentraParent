use std::{
    fs::{remove_file, File},
    io,
    path::Path,
};

use crate::{ndjson_append_rollback::rollback_partial_append, ndjson_writer::append_bytes};

pub(crate) fn append_operation_record(
    file: &mut File,
    intent: &Path,
    record: &[u8],
    before_write: io::Result<()>,
) -> io::Result<u64> {
    match before_write.and_then(|_| append_bytes(file, record)) {
        Ok(committed_offset) => Ok(committed_offset),
        Err(error) => {
            remove_file(intent)?;
            rollback_partial_append(file, error)?;
            Err(io::Error::other("NDJSON rollback returned unexpectedly"))
        }
    }
}
