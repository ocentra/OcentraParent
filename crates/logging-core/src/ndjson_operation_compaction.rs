use std::{
    fs::OpenOptions,
    io::{self, BufRead, BufReader, Seek, SeekFrom, Write},
};

use serde::{Deserialize, Serialize};

use crate::{
    artifact_publish_lock::remove_temporary, ndjson_operation_marker::marker_offset,
    ndjson_operation_marker_state::OperationPaths, ndjson_tail_recovery::recover_partial_tail,
};

#[derive(Deserialize, Serialize)]
struct CompactCommit {
    key: String,
    marker: String,
}

pub(crate) fn compact_commit(
    paths: &OperationPaths,
    operation_id: &str,
    record: &[u8],
) -> io::Result<u64> {
    let marker = std::fs::read(&paths.commit)?;
    let offset = marker_offset(&marker, operation_id, record)?;
    if compacted_offset(paths, operation_id, record)?.is_none() {
        append_compacted(paths, &marker)?;
    }
    remove_temporary(&paths.commit)?;
    Ok(offset)
}

pub(crate) fn compacted_offset(
    paths: &OperationPaths,
    operation_id: &str,
    record: &[u8],
) -> io::Result<Option<u64>> {
    let mut file = match OpenOptions::new()
        .read(true)
        .write(true)
        .open(&paths.compacted)
    {
        Ok(file) => file,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error),
    };
    recover_partial_tail(&mut file)?;
    file.seek(SeekFrom::Start(0))?;
    for line in BufReader::new(&mut file).lines() {
        let entry: CompactCommit = serde_json::from_str(&line?)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
        if entry.key == paths.key {
            return marker_offset(entry.marker.as_bytes(), operation_id, record).map(Some);
        }
    }
    Ok(None)
}

fn append_compacted(paths: &OperationPaths, marker: &[u8]) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&paths.compacted)?;
    recover_partial_tail(&mut file)?;
    file.seek(SeekFrom::End(0))?;
    let entry = CompactCommit {
        key: paths.key.clone(),
        marker: std::str::from_utf8(marker)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?
            .to_owned(),
    };
    serde_json::to_writer(&mut file, &entry)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    file.write_all(b"\n")?;
    file.sync_data()
}
