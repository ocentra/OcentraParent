use super::super::super::super::LocalArtifactMutationError;
use super::ArtifactError;

pub(super) fn map(error: &ArtifactError) -> Option<LocalArtifactMutationError> {
    match error {
        ArtifactError::LockConflict => Some(LocalArtifactMutationError::LockConflict),
        ArtifactError::NotFound => Some(LocalArtifactMutationError::NotFound),
        ArtifactError::AlreadyExists => Some(LocalArtifactMutationError::AlreadyExists),
        ArtifactError::SizeLimit => Some(LocalArtifactMutationError::SizeLimit),
        ArtifactError::RecoveryRequired => Some(LocalArtifactMutationError::RecoveryRequired),
        _ => None,
    }
}
