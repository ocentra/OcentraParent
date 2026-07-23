use std::{
    fs::{remove_file, File},
    io,
    path::Path,
};

use crate::ndjson_writer::append_bytes;

pub(crate) fn append_operation_record(
    file: &mut File,
    intent: &Path,
    record: &[u8],
    before_write: io::Result<()>,
) -> io::Result<u64> {
    append_operation_record_with_rollback(file, intent, record, before_write, |file| {
        crate::ndjson_writer::recover_partial_tail(file)?;
        file.sync_data()
    })
}

pub(crate) fn append_operation_record_with_rollback(
    file: &mut File,
    intent: &Path,
    record: &[u8],
    before_write: io::Result<()>,
    rollback: impl FnOnce(&mut File) -> io::Result<()>,
) -> io::Result<u64> {
    match before_write.and_then(|_| append_bytes(file, record)) {
        Ok(committed_offset) => Ok(committed_offset),
        Err(error) => {
            if let Err(rollback_error) = rollback(file) {
                return Err(io::Error::new(
                    rollback_error.kind(),
                    format!(
                        "NDJSON append failed ({error}); rollback also failed ({rollback_error})"
                    ),
                ));
            }
            remove_file(intent)?;
            Err(error)
        }
    }
}
