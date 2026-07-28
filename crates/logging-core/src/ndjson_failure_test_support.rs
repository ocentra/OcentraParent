use std::{
    fs::{create_dir_all, write},
    io::{self, Write},
    path::Path,
};

pub fn append_plain_record_with_parent_sync_fault(path: &Path, record: &[u8]) -> io::Result<()> {
    crate::ndjson_writer::append_record_with_parent_sync(path, record, |_| {
        Err(io::Error::other(
            "injected NDJSON parent directory sync failure",
        ))
    })
}

pub fn operation_intent_after_failed_rollback(
    path: &Path,
    intent: &Path,
    record: &[u8],
) -> io::Result<(bool, String)> {
    let parent = path.parent().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "NDJSON stream path has no parent directory",
        )
    })?;
    create_dir_all(parent)?;
    write(intent, b"owned operation intent")?;
    let mut file = crate::ndjson_writer::open_append_file(path)?;
    file.write_all(b"{\"partial\":")?;
    file.sync_data()?;
    let error = crate::ndjson_operation_append::append_operation_record_with_rollback(
        &mut file,
        intent,
        record,
        Err(io::Error::other("injected NDJSON append failure")),
        |_file| Err(io::Error::other("injected NDJSON rollback failure")),
    )
    .err()
    .ok_or_else(|| io::Error::other("injected NDJSON rollback failure was not observed"))?;
    Ok((intent.exists(), error.to_string()))
}
