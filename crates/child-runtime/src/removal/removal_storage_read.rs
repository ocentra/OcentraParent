use std::fs;
use std::io;

use super::{
    removal_record::{empty_record, ChildAgentRemovalRecord},
    removal_validation::validate_record,
    ChildAgentRemovalBoundary,
};

impl ChildAgentRemovalBoundary {
    pub(super) fn read_record_unlocked(&self) -> io::Result<ChildAgentRemovalRecord> {
        match fs::read(&self.path) {
            Ok(bytes) => decode_record(&bytes),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(empty_record()),
            Err(error) => Err(error),
        }
    }
}

fn decode_record(bytes: &[u8]) -> io::Result<ChildAgentRemovalRecord> {
    let record: ChildAgentRemovalRecord = serde_json::from_slice(bytes)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    if record.version != super::REMOVAL_STATE_VERSION {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "unsupported child removal state version",
        ));
    }
    validate_record(&record)?;
    Ok(record)
}
