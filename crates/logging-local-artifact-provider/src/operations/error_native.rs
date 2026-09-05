use ocentra_parent_logging_core::local_artifact_mutation::LocalArtifactMutationError;

use super::{FailureDisposition, ProviderError};
use crate::protocol::text;

pub(super) fn map(error: &LocalArtifactMutationError) -> ProviderError {
    match error {
        LocalArtifactMutationError::SizeLimit => {
            ProviderError::new(text::ARTIFACT_SIZE_LIMIT, FailureDisposition::Continue)
        }
        LocalArtifactMutationError::DurabilityFailure => {
            ProviderError::new(text::DURABILITY_FAILURE, FailureDisposition::Terminate)
        }
        LocalArtifactMutationError::RecoveryRequired => {
            ProviderError::new(text::RECOVERY_UNCERTAINTY, FailureDisposition::Terminate)
        }
        LocalArtifactMutationError::UnsupportedOperation => {
            ProviderError::new(text::UNSUPPORTED_OPERATION, FailureDisposition::Continue)
        }
        LocalArtifactMutationError::Native(_) => {
            ProviderError::new(text::NATIVE_FAILURE, FailureDisposition::Terminate)
        }
        _ => ProviderError::new(text::PROTOCOL_FRAME, FailureDisposition::Terminate),
    }
}
