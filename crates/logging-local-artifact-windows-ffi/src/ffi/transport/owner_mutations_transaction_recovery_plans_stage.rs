use super::*;
use crate::constants::{
    BRIDGE_DIRECTORY, INTENTS_DIRECTORY, MUTATION_OWNER_DIRECTORY, REPLACE_OPERATION,
};

pub(super) fn path(
    session: &MutationSession<'_>,
    request_id: &str,
    index: usize,
    item: &StagedMutation,
) -> Result<Option<PathBuf>, ArtifactError> {
    match item.staged_name.as_deref() {
        Some(name) => {
            let expected = format!("{request_id}.stage-{index}");
            if name != expected || item.operation != REPLACE_OPERATION {
                return Err(ArtifactError::RecoveryRequired);
            }
            Ok(Some(
                session
                    .owner
                    .root_path
                    .join(BRIDGE_DIRECTORY)
                    .join(MUTATION_OWNER_DIRECTORY)
                    .join(INTENTS_DIRECTORY)
                    .join(name),
            ))
        }
        None if item.operation == REPLACE_OPERATION => Err(ArtifactError::RecoveryRequired),
        None => Ok(None),
    }
}

pub(super) fn open(path: Option<&Path>) -> Result<Option<OwnedFile>, ArtifactError> {
    match path {
        Some(path) => optional_mutation_file(path),
        None => Ok(None),
    }
}
