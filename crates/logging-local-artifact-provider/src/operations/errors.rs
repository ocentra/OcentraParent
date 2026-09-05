#[path = "error_basic.rs"]
mod basic;
#[path = "error_native.rs"]
mod native;

use getrandom::fill;
use ocentra_parent_logging_core::local_artifact_mutation::LocalArtifactMutationError;

use super::{FailureDisposition, ProviderError};
use crate::protocol;

pub(super) fn map_owner_error(error: &LocalArtifactMutationError) -> ProviderError {
    if let Some(mapped) = basic::map(error) {
        return mapped;
    }
    native::map(error)
}

pub(super) fn random_identifier() -> Result<crate::protocol::ProviderIdentifier, ProviderError> {
    let mut bytes = vec![0_u8; 32];
    fill(&mut bytes).map_err(|_error| {
        ProviderError::new(
            crate::protocol::text::SECURE_IDENTIFIER_FAILURE,
            FailureDisposition::Terminate,
        )
    })?;
    let encoded = protocol::hex_encode(&bytes);
    Ok(crate::protocol::ProviderIdentifier::generated(encoded))
}
