use std::io::{self, Read, Seek, SeekFrom};

use sha2::{Digest, Sha256};

pub(crate) fn marker_content(operation_id: &str, record: &[u8], offset: u64) -> String {
    format!(
        "{operation_id}\n{:x}\n{offset}\n{}\n",
        Sha256::digest(record),
        record.len()
    )
}

pub(crate) fn record_matches_at<R>(reader: &mut R, offset: u64, record: &[u8]) -> io::Result<bool>
where
    R: Read + Seek,
{
    reader.seek(SeekFrom::Start(offset))?;
    let mut candidate = vec![0; record.len()];
    match reader.read_exact(&mut candidate) {
        Ok(()) => Ok(candidate == record),
        Err(error) if error.kind() == io::ErrorKind::UnexpectedEof => Ok(false),
        Err(error) => Err(error),
    }
}

pub(crate) fn marker_offset(marker: &[u8], operation_id: &str, record: &[u8]) -> io::Result<u64> {
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
