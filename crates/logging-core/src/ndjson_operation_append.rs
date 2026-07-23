use std::{
    fs::{remove_file, File},
    io,
    path::Path,
};

use crate::ndjson_writer::{append_bytes, rollback_append};

pub(crate) fn append_operation_record(
    #[cfg(windows)] file: &File,
    #[cfg(not(windows))] file: &mut File,
    intent: &Path,
    expected_offset: u64,
    record: &[u8],
    before_write: io::Result<()>,
) -> io::Result<u64> {
    match before_write.and_then(|_| append_bytes(file, record)) {
        Ok(committed_offset) => Ok(committed_offset),
        Err(error) => {
            remove_file(intent)?;
            rollback_append(file, expected_offset, error)?;
            Err(io::Error::other("NDJSON rollback returned unexpectedly"))
        }
    }
}
