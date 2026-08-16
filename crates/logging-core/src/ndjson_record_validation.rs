use std::io;

pub(crate) fn validate_record(record: &[u8]) -> io::Result<()> {
    if record.is_empty() || record.last() != Some(&b'\n') {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "NDJSON records must end with a newline",
        ));
    }
    let payload = &record[..record.len() - 1];
    if payload.contains(&b'\n') || payload.contains(&b'\r') {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "NDJSON records must contain exactly one physical line",
        ));
    }
    serde_json::from_slice::<serde_json::Value>(payload)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error))?;
    Ok(())
}
