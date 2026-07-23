use std::{
    fs::{create_dir_all, read, remove_file},
    io,
    path::{Path, PathBuf},
};

use sha2::{Digest, Sha256};

use crate::ndjson_operation_marker::marker_offset;

pub(crate) struct OperationPaths {
    pub(crate) key: String,
    pub(crate) intent: PathBuf,
    pub(crate) commit: PathBuf,
    pub(crate) compacted: PathBuf,
}

pub(crate) fn operation_paths(path: &Path, operation_id: &str) -> io::Result<OperationPaths> {
    let directory = operation_directory(path)?;
    create_dir_all(&directory)?;
    let key = format!("{:x}", Sha256::digest(operation_id.as_bytes()));
    Ok(OperationPaths {
        key: key.clone(),
        intent: directory.join(format!("{key}.intent")),
        commit: directory.join(format!("{key}.commit")),
        compacted: directory.join("commits.ndjson"),
    })
}

pub(crate) fn operation_directory(path: &Path) -> io::Result<PathBuf> {
    let parent = path
        .parent()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "NDJSON path has no parent"))?;
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "NDJSON path has no file name")
        })?;
    Ok(parent.join(format!(".{file_name}.operations")))
}

pub(crate) fn read_intent_offset(
    path: &Path,
    operation_id: &str,
    record: &[u8],
) -> io::Result<Option<u64>> {
    match read(path) {
        Ok(marker) if marker_is_complete(&marker) => {
            marker_offset(&marker, operation_id, record).map(Some)
        }
        Ok(_) => {
            remove_file(path)?;
            Ok(None)
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error),
    }
}

pub(crate) fn read_commit_offset(
    path: &Path,
    operation_id: &str,
    record: &[u8],
) -> io::Result<Option<u64>> {
    let marker = match read(path) {
        Ok(marker) => marker,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error),
    };
    if marker_is_complete(&marker) {
        return marker_offset(&marker, operation_id, record).map(Some);
    }
    remove_file(path)?;
    Ok(None)
}

pub(crate) fn marker_is_complete(marker: &[u8]) -> bool {
    let Ok(marker) = std::str::from_utf8(marker) else {
        return false;
    };
    let lines = marker.lines().collect::<Vec<_>>();
    lines.len() == 4
        && !lines[0].is_empty()
        && lines[1].len() == 64
        && lines[1].bytes().all(|byte| byte.is_ascii_hexdigit())
        && lines[2].parse::<u64>().is_ok()
        && lines[3].parse::<usize>().is_ok()
        && marker.ends_with('\n')
}
