use std::io;

use crate::{
    artifact_publish_lock::remove_temporary,
    artifact_publish_platform::sync_parent,
    ndjson_operation_compaction_index::{append_compacted_marker, compacted_marker},
    ndjson_operation_marker::marker_offset,
    ndjson_operation_marker_state::OperationPaths,
};

pub(crate) fn compact_commit(
    paths: &OperationPaths,
    operation_id: &str,
    record: &[u8],
) -> io::Result<u64> {
    let marker = std::fs::read(&paths.commit)?;
    let offset = marker_offset(&marker, operation_id, record)?;
    if compacted_offset(paths, operation_id, record)?.is_none() {
        let marker = std::str::from_utf8(&marker)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
        append_compacted_marker(&paths.compacted, &paths.key, marker)?;
    }
    sync_parent(&paths.compacted)?;
    remove_temporary(&paths.commit)?;
    sync_parent(&paths.compacted)?;
    Ok(offset)
}

pub(crate) fn compacted_offset(
    paths: &OperationPaths,
    operation_id: &str,
    record: &[u8],
) -> io::Result<Option<u64>> {
    compacted_marker(&paths.compacted, &paths.key)?
        .map(|marker| marker_offset(marker.as_bytes(), operation_id, record))
        .transpose()
}
