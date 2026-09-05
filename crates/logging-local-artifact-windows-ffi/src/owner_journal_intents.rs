use std::fmt::Display;
use std::path::Path;

use crate::error::ArtifactError;

use super::*;

pub(super) fn write_intent<R>(
    root: &Path,
    request_id: R,
    record: &IntentRecord,
    intent_directory: &OwnedFile,
) -> Result<(), ArtifactError>
where
    R: Display + descriptors::names::TempNameInput,
{
    record.validate()?;
    let bytes = serde_json::to_vec(record).map_err(|_| ArtifactError::RecoveryRequired)?;
    if u64::try_from(bytes.len()).map_err(|_| ArtifactError::SizeLimit)? > MAX_INTENT_BYTES {
        return Err(ArtifactError::SizeLimit);
    }
    let intent_name = format!("{request_id}{JSON_SUFFIX}");
    let temp_name = intent_temp_name::<_, _, String>(&request_id, &INTENT_OPERATION);
    let temp_path = root
        .join(BRIDGE_DIRECTORY)
        .join(MUTATION_OWNER_DIRECTORY)
        .join(INTENTS_DIRECTORY)
        .join(&temp_name);
    let mut file = OwnedFile::create_new_file(&temp_path)?;
    file.write_bytes(&bytes)?;
    file.sync_file()?;
    file.rename_into(intent_directory, &intent_name)?;
    intent_directory.sync_directory()
}

pub(super) fn replace_intent<R, S>(
    root: &Path,
    request_id: R,
    record: &IntentRecord,
    intent_directory: &OwnedFile,
    suffix: S,
) -> Result<(), ArtifactError>
where
    R: Display + descriptors::names::TempNameInput,
    S: Display,
{
    record.validate()?;
    let bytes = serde_json::to_vec(record).map_err(|_| ArtifactError::RecoveryRequired)?;
    if u64::try_from(bytes.len()).map_err(|_| ArtifactError::SizeLimit)? > MAX_INTENT_BYTES {
        return Err(ArtifactError::SizeLimit);
    }
    let temp_name = intent_temp_name::<_, _, String>(&request_id, &suffix);
    let path = root
        .join(BRIDGE_DIRECTORY)
        .join(MUTATION_OWNER_DIRECTORY)
        .join(INTENTS_DIRECTORY)
        .join(&temp_name);
    let mut temp = OwnedFile::create_new_replace_file(&path)?;
    temp.write_bytes(&bytes)?;
    temp.sync_file()?;
    let intent_name = format!("{request_id}{JSON_SUFFIX}");
    temp.replace_into(intent_directory, &intent_name)?;
    intent_directory.sync_directory()
}

pub(super) fn remove_intent<R>(
    root: &Path,
    request_id: R,
    intent_directory: &OwnedFile,
) -> Result<(), ArtifactError>
where
    R: Display,
{
    let path = intent_path(root, &request_id);
    let file = match OwnedFile::open_existing_file(path.as_path()) {
        Ok(file) => file,
        Err(ArtifactError::NotFound) => return Ok(()),
        Err(error) => return Err(error),
    };
    file.mark_deleted()?;
    intent_directory.sync_directory()
}
