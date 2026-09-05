use std::path::Path;

use crate::error::ArtifactError;

use super::*;

pub(super) fn unreferenced(
    directory: &Path,
    intent_directory: &OwnedFile,
    entries: &super::IntentEntryNames,
    referenced: &super::ReferencedTemps,
) -> Result<(), ArtifactError> {
    let mut removed = false;
    for name in entries.0.iter().filter(|name| !name.ends_with(JSON_SUFFIX)) {
        if referenced.0.contains(name) {
            continue;
        }
        validate_generated_name(name)?;
        delete_entry(&directory.join(name))?;
        removed = true;
    }
    if removed {
        intent_directory.sync_directory()?;
    }
    Ok(())
}

fn validate_generated_name<N>(name: &N) -> Result<(), ArtifactError>
where
    N: descriptors::generated_names::GeneratedNameInput + ?Sized,
{
    if generated_intent_temp_name(name) || generated_intent_stage_name(name) {
        return Ok(());
    }
    Err(ArtifactError::RecoveryRequired)
}

pub(super) fn insert_temp<N>(
    name: &N,
    referenced: &mut super::ReferencedTemps,
) -> Result<(), ArtifactError>
where
    N: descriptors::generated_names::GeneratedNameInput + ?Sized,
{
    if !generated_intent_temp_name(name) {
        return Err(ArtifactError::RecoveryRequired);
    }
    referenced.0.insert(name.to_string());
    Ok(())
}

pub(super) fn insert_stage<N>(
    name: &N,
    referenced: &mut super::ReferencedTemps,
) -> Result<(), ArtifactError>
where
    N: descriptors::generated_names::GeneratedNameInput + ?Sized,
{
    if !generated_intent_stage_name(name) {
        return Err(ArtifactError::RecoveryRequired);
    }
    referenced.0.insert(name.to_string());
    Ok(())
}

fn delete_entry(path: &Path) -> Result<(), ArtifactError> {
    let file = OwnedFile::open_existing_file(path)?;
    file.mark_deleted()
}
