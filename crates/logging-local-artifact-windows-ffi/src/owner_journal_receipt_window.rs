use std::fs;
use std::path::Path;

use crate::error::{io_error, ArtifactError};

use super::*;

pub(super) fn enforce(root: &Path) -> Result<(), ArtifactError> {
    let directory = root
        .join(BRIDGE_DIRECTORY)
        .join(MUTATION_OWNER_DIRECTORY)
        .join(RECEIPTS_DIRECTORY);
    let mut retained = 0usize;
    for entry in fs::read_dir(&directory).map_err(io_error)? {
        let entry = entry.map_err(io_error)?;
        let file_name = entry.file_name();
        let name = file_name.to_str().ok_or(ArtifactError::RecoveryRequired)?;
        classify_entry(name, &mut retained)?;
    }
    if retained >= MAX_RETAINED_RECEIPTS {
        return Err(ArtifactError::SizeLimit);
    }
    Ok(())
}

fn classify_entry<N>(name: &N, retained: &mut usize) -> Result<(), ArtifactError>
where
    N: descriptors::generated_names::ReceiptTempNameInput + ?Sized,
{
    let name = name.to_string();
    if name.ends_with(JSON_SUFFIX) && name.len() > JSON_SUFFIX.len() {
        validate_request_id(&name[..name.len() - JSON_SUFFIX.len()])?;
        *retained = retained.checked_add(1).ok_or(ArtifactError::SizeLimit)?;
        return Ok(());
    }
    if generated_receipt_temp_name(&name) {
        return Err(ArtifactError::RecoveryRequired);
    }
    Err(ArtifactError::RecoveryRequired)
}

fn generated_receipt_temp_name<N>(name: &N) -> bool
where
    N: descriptors::generated_names::ReceiptTempNameInput + ?Sized,
{
    name.is_generated_receipt_temp_name()
}
