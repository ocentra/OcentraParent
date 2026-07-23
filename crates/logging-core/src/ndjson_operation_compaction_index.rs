use std::{
    fs::OpenOptions,
    io::{self, BufRead, BufReader, Seek, SeekFrom, Write},
    path::Path,
};

use serde::{Deserialize, Serialize};

use crate::{
    ndjson_operation_compaction_cache::{
        commit_file_identity, with_commit_index, CachedCommitIndex, CommitFileIdentity,
    },
    ndjson_tail_recovery::recover_partial_tail,
};

#[derive(Deserialize, Serialize)]
struct CompactCommit {
    key: String,
    marker: String,
}

pub(crate) fn compacted_marker(path: &Path, key: &str) -> io::Result<Option<String>> {
    let mut file = match OpenOptions::new().read(true).write(true).open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error),
    };
    recover_partial_tail(&mut file)?;
    let metadata = file.metadata()?;
    let file_len = metadata.len();
    let identity = commit_file_identity(&metadata);
    with_commit_index(path, |index| {
        refresh_index(&mut file, file_len, identity, index)?;
        if let Some(marker) = index.markers.get(key) {
            return Ok(Some(marker.clone()));
        }
        if !index.might_contain(key) {
            return Ok(None);
        }
        let marker = scan_marker(&mut file, key)?;
        if let Some(marker) = &marker {
            index.record_marker(key.to_owned(), marker.clone());
        }
        Ok(marker)
    })
}

pub(crate) fn append_compacted_marker(path: &Path, key: &str, marker: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(path)?;
    recover_partial_tail(&mut file)?;
    let start = file.seek(SeekFrom::End(0))?;
    serde_json::to_writer(
        &mut file,
        &CompactCommit {
            key: key.to_owned(),
            marker: marker.to_owned(),
        },
    )
    .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    file.write_all(b"\n")?;
    file.sync_data()?;
    let end = file.stream_position()?;
    let identity = commit_file_identity(&file.metadata()?);
    with_commit_index(path, |index| {
        index.prepare(identity, start);
        if index.scanned_len == start {
            index.record_marker(key.to_owned(), marker.to_owned());
            index.scanned_len = end;
        }
        Ok(())
    })
}

fn refresh_index(
    file: &mut std::fs::File,
    file_len: u64,
    identity: CommitFileIdentity,
    index: &mut CachedCommitIndex,
) -> io::Result<()> {
    index.prepare(identity, file_len);
    let start = index.scanned_len;
    file.seek(SeekFrom::Start(start))?;
    for line in BufReader::new(file).lines() {
        let entry: CompactCommit = serde_json::from_str(&line?)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
        index.record_marker(entry.key, entry.marker);
    }
    index.scanned_len = file_len;
    #[cfg(feature = "test-support")]
    {
        index.scanned_bytes += file_len.saturating_sub(start);
    }
    Ok(())
}

fn scan_marker(file: &mut std::fs::File, key: &str) -> io::Result<Option<String>> {
    file.seek(SeekFrom::Start(0))?;
    for line in BufReader::new(file).lines() {
        let entry: CompactCommit = serde_json::from_str(&line?)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
        if entry.key == key {
            return Ok(Some(entry.marker));
        }
    }
    Ok(None)
}
