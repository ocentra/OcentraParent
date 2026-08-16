use std::{
    fs::{create_dir_all, OpenOptions},
    io::{self, Write},
    path::Path,
};

use crate::ndjson_operation::{append_operation_with_fault, FaultPoint};

pub fn append_record_with_external_interleave(
    path: &Path,
    operation_id: &str,
    record: &[u8],
    external_record: &[u8],
) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    let mut file = crate::ndjson_writer::open_append_file(path)?;
    append_operation_with_fault(&mut file, path, operation_id, record, |point| match point {
        FaultPoint::Write => append_external_record(path, external_record),
        FaultPoint::Sync => Ok(()),
    })
}

fn append_external_record(path: &Path, record: &[u8]) -> io::Result<()> {
    let mut file = OpenOptions::new().append(true).open(path)?;
    file.write_all(record)?;
    file.sync_data()
}
