use ocentra_parent_logging_core::local_artifact_mutation::LocalArtifactMutationError;

use super::{FailureDisposition, ProviderError};
use crate::protocol::text;

pub(super) fn map(error: &LocalArtifactMutationError) -> Option<ProviderError> {
    if matches!(error, LocalArtifactMutationError::UnsupportedPlatform) {
        return Some(continue_error(text::UNSUPPORTED_PROVIDER));
    }
    if matches!(error, LocalArtifactMutationError::InvalidPath) {
        return Some(continue_error(text::CONTAINMENT_FAILURE));
    }
    if matches!(error, LocalArtifactMutationError::InvalidRequestId) {
        return Some(continue_error(text::MUTATION_REQUEST_ID_INVALID));
    }
    if matches!(error, LocalArtifactMutationError::RequestIdConflict) {
        return Some(continue_error(text::RETAINED_REQUEST_CONFLICT));
    }
    if matches!(error, LocalArtifactMutationError::RootIdentityChanged) {
        return Some(continue_error(text::ROOT_IDENTITY_CHANGED));
    }
    if matches!(error, LocalArtifactMutationError::AncestorIdentityChanged) {
        return Some(continue_error(text::ANCESTOR_IDENTITY_CHANGED));
    }
    if matches!(error, LocalArtifactMutationError::LinkOrReparseDetected) {
        return Some(continue_error(text::LINK_OR_REPARSE));
    }
    if matches!(error, LocalArtifactMutationError::HardlinkDetected) {
        return Some(continue_error(text::HARDLINK_CHANGED));
    }
    if matches!(error, LocalArtifactMutationError::OwnershipChanged) {
        return Some(continue_error(text::OWNERSHIP_CHANGED));
    }
    if matches!(error, LocalArtifactMutationError::LockConflict) {
        return Some(continue_error(text::LOCK_CONFLICT));
    }
    if matches!(error, LocalArtifactMutationError::NotFound) {
        return Some(continue_error(text::ARTIFACT_NOT_FOUND));
    }
    if matches!(error, LocalArtifactMutationError::AlreadyExists) {
        return Some(continue_error(text::ARTIFACT_ALREADY_EXISTS));
    }
    None
}

fn continue_error(value: text::ErrorText) -> ProviderError {
    ProviderError::new(value, FailureDisposition::Continue)
}
