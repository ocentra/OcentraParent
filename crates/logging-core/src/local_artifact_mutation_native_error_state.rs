use super::super::super::LocalArtifactMutationError;
use super::ArtifactError;

#[path = "local_artifact_mutation_native_error_state_identity.rs"]
mod identity;
#[path = "local_artifact_mutation_native_error_state_target.rs"]
mod target;

pub(super) fn map(error: &ArtifactError) -> Option<LocalArtifactMutationError> {
    identity::map(error).or_else(|| target::map(error))
}
