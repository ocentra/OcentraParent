use std::fs;
use std::path::Path;

use crate::error::{io_error, ArtifactError};

use super::*;

pub(super) fn reconcile(root: &Path, receipt_directory: &OwnedFile) -> Result<(), ArtifactError> {
    let directory = root
        .join(BRIDGE_DIRECTORY)
        .join(MUTATION_OWNER_DIRECTORY)
        .join(RECEIPTS_DIRECTORY);
    let mut removed = false;
    for entry in fs::read_dir(&directory).map_err(io_error)? {
        let entry = entry.map_err(io_error)?;
        let file_name = entry.file_name();
        let name = file_name.to_str().ok_or(ArtifactError::RecoveryRequired)?;
        if name.ends_with(JSON_SUFFIX) && name.len() > JSON_SUFFIX.len() {
            validate_request_id(&name[..name.len() - JSON_SUFFIX.len()])?;
            continue;
        }
        if !generated_receipt_temp_name(name) {
            return Err(ArtifactError::RecoveryRequired);
        }
        delete_entry(&directory.join(name))?;
        removed = true;
    }
    if removed {
        receipt_directory.sync_directory()?;
    }
    Ok(())
}

fn delete_entry(path: &Path) -> Result<(), ArtifactError> {
    let file = OwnedFile::open_existing_file(path)?;
    file.mark_deleted()
}
