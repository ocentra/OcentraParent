use std::fmt::Display;
use std::path::Path;

use crate::error::ArtifactError;

use super::*;

pub(super) fn write<R>(
    root: &Path,
    request_id: R,
    record: &IntentRecord,
    intent_directory: &OwnedFile,
) -> Result<(), ArtifactError>
where
    R: Display + descriptors::names::TempNameInput,
{
    intents::write_intent(root, request_id, record, intent_directory)
}

pub(super) fn replace<R, S>(
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
    intents::replace_intent(root, request_id, record, intent_directory, suffix)
}

pub(super) fn remove<R>(
    root: &Path,
    request_id: R,
    intent_directory: &OwnedFile,
) -> Result<(), ArtifactError>
where
    R: Display,
{
    intents::remove_intent(root, request_id, intent_directory)
}
