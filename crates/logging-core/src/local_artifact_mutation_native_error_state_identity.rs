use super::super::super::super::LocalArtifactMutationError;
use super::ArtifactError;

pub(super) fn map(error: &ArtifactError) -> Option<LocalArtifactMutationError> {
    match error {
        ArtifactError::RequestIdConflict => Some(LocalArtifactMutationError::RequestIdConflict),
        ArtifactError::RootIdentityChanged => Some(LocalArtifactMutationError::RootIdentityChanged),
        ArtifactError::AncestorIdentityChanged => {
            Some(LocalArtifactMutationError::AncestorIdentityChanged)
        }
        ArtifactError::LinkOrReparseDetected => {
            Some(LocalArtifactMutationError::LinkOrReparseDetected)
        }
        ArtifactError::HardlinkDetected => Some(LocalArtifactMutationError::HardlinkDetected),
        ArtifactError::OwnershipChanged => Some(LocalArtifactMutationError::OwnershipChanged),
        _ => None,
    }
}
