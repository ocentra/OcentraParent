use super::super::LocalArtifactMutationError;

use ocentra_parent_logging_local_artifact_windows_ffi::error::ArtifactError;

#[path = "local_artifact_mutation_native_error_input.rs"]
mod input;
#[path = "local_artifact_mutation_native_error_state.rs"]
mod state;

pub(crate) fn error_from_native(error: ArtifactError) -> LocalArtifactMutationError {
    if let Some(mapped) = input::map(&error) {
        return mapped;
    }
    if let Some(mapped) = state::map(&error) {
        return mapped;
    }
    match error {
        ArtifactError::DurabilityFailure | ArtifactError::DurabilityFailureWith(_) => {
            LocalArtifactMutationError::DurabilityFailure
        }
        ArtifactError::Io(message) => LocalArtifactMutationError::Native(message),
        _ => LocalArtifactMutationError::Native("unclassified native error".to_owned()),
    }
}
