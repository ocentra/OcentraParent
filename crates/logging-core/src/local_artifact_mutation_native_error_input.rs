use super::super::super::LocalArtifactMutationError;
use super::ArtifactError;

pub(super) fn map(error: &ArtifactError) -> Option<LocalArtifactMutationError> {
    match error {
        ArtifactError::UnsupportedPlatform => Some(LocalArtifactMutationError::UnsupportedPlatform),
        ArtifactError::UnsupportedOperation(_) => {
            Some(LocalArtifactMutationError::UnsupportedOperation)
        }
        ArtifactError::InvalidPath(_) => Some(LocalArtifactMutationError::InvalidPath),
        ArtifactError::InvalidRequestId => Some(LocalArtifactMutationError::InvalidRequestId),
        _ => None,
    }
}
