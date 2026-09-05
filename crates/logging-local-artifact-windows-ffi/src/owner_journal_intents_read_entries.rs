use std::fs;
use std::path::Path;

use crate::error::{io_error, ArtifactError};

pub(super) fn collect(directory: &Path) -> Result<super::IntentEntryNames, ArtifactError> {
    let mut entries = Vec::new();
    for entry in fs::read_dir(directory).map_err(io_error)? {
        let entry = entry.map_err(io_error)?;
        let file_name = entry.file_name();
        let name = file_name
            .to_str()
            .ok_or(ArtifactError::RecoveryRequired)?
            .to_owned();
        entries.push(name);
    }
    Ok(super::IntentEntryNames(entries))
}
