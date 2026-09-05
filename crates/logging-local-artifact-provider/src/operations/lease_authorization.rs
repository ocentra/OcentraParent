use ocentra_parent_logging_core::local_artifact_mutation::LocalArtifactMutationSession;

use super::super::{FailureDisposition, LeaseRequirement, LeaseState, ProviderError};
use crate::protocol::ProviderIdentifier;

pub(super) fn authorize_lease(
    lease: &LeaseState,
    supplied: Option<&ProviderIdentifier>,
    requirement: LeaseRequirement,
) -> Result<(), ProviderError> {
    match (lease.current.as_ref(), supplied, requirement) {
        (Some(current), Some(value), _) if current == value => Ok(()),
        (Some(_), _, _) | (None, Some(_), _) => Err(ProviderError::new(
            crate::protocol::text::LEASE_NOT_CURRENT,
            FailureDisposition::Continue,
        )),
        (None, None, LeaseRequirement::Required) => Err(ProviderError::new(
            crate::protocol::text::LEASE_REQUIRED,
            FailureDisposition::Continue,
        )),
        (None, None, LeaseRequirement::Optional) => Ok(()),
    }
}

pub(super) fn verify_session_current(
    session: &LocalArtifactMutationSession<'_>,
) -> Result<(), ProviderError> {
    session
        .verify_current()
        .map_err(|error| super::super::map_owner_error(&error))
}
