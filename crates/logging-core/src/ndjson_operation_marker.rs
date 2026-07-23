use std::{
    fs::{create_dir_all, read, File},
    io::{self, Read, Seek, SeekFrom},
    path::{Path, PathBuf},
};

use sha2::{Digest, Sha256};

pub(crate) struct OperationPaths {
    pub(crate) intent: PathBuf,
    pub(crate) commit: PathBuf,
}

pub(crate) fn operation_paths(path: &Path, operation_id: &str) -> io::Result<OperationPaths> {
    let parent = path
        .parent()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "NDJSON path has no parent"))?;
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "NDJSON path has no file name")
        })?;
    let directory = parent.join(format!(".{file_name}.operations"));
    create_dir_all(&directory)?;
    let key = format!("{:x}", Sha256::digest(operation_id.as_bytes()));
    Ok(OperationPaths {
        intent: directory.join(format!("{key}.intent")),
        commit: directory.join(format!("{key}.commit")),
    })
}

pub(crate) fn marker_content(operation_id: &str, record: &[u8], offset: u64) -> String {
    format!(
        "{operation_id}\n{:x}\n{offset}\n{}\n",
        Sha256::digest(record),
        record.len()
    )
}

pub(crate) fn read_intent_offset(
    path: &Path,
    operation_id: &str,
    record: &[u8],
) -> io::Result<Option<u64>> {
    match read(path) {
        Ok(marker) => marker_offset(&marker, operation_id, record).map(Some),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error),
    }
}

pub(crate) fn validate_operation_marker(
    path: &Path,
    operation_id: &str,
    record: &[u8],
) -> io::Result<()> {
    marker_offset(&read(path)?, operation_id, record).map(|_| ())
}

pub(crate) fn record_matches_at(file: &mut File, offset: u64, record: &[u8]) -> io::Result<bool> {
    file.seek(SeekFrom::Start(offset))?;
    let mut candidate = vec![0; record.len()];
    let read = file.read(&mut candidate)?;
    Ok(read == record.len() && candidate == record)
}

fn marker_offset(marker: &[u8], operation_id: &str, record: &[u8]) -> io::Result<u64> {
    let marker = std::str::from_utf8(marker)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    let mut lines = marker.lines();
    let stored_id = lines.next().unwrap_or_default();
    let digest = lines.next().unwrap_or_default();
    let offset = lines
        .next()
        .unwrap_or_default()
        .parse::<u64>()
        .map_err(invalid_marker)?;
    let length = lines
        .next()
        .unwrap_or_default()
        .parse::<usize>()
        .map_err(invalid_marker)?;
    if stored_id == operation_id
        && digest == format!("{:x}", Sha256::digest(record))
        && length == record.len()
    {
        return Ok(offset);
    }
    Err(io::Error::new(
        io::ErrorKind::AlreadyExists,
        "operation id conflicts with a different record",
    ))
}

fn invalid_marker(error: impl std::error::Error + Send + Sync + 'static) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, error)
}
