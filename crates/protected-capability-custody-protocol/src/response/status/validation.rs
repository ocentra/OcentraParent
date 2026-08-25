use crate::request::RequestKind;
use crate::types::{OpaquePreparedToken, ProtocolError};

use super::super::{ObservedGenerations, ResponseStatus};

pub(crate) fn validate_result(
    request_kind: RequestKind,
    status: ResponseStatus,
    observed_generations: Option<ObservedGenerations>,
    opaque_token: Option<&OpaquePreparedToken>,
) -> Result<(), ProtocolError> {
    if !status.is_compatible_with(request_kind) {
        return Err(ProtocolError::InvalidStatusForRequest);
    }
    if status.requires_observed_generations() != observed_generations.is_some() {
        return Err(ProtocolError::InvalidEpoch);
    }
    if matches!(status, ResponseStatus::Prepared) != opaque_token.is_some() {
        return Err(if matches!(status, ResponseStatus::Prepared) {
            ProtocolError::InvalidOpaqueToken
        } else {
            ProtocolError::UnexpectedOpaqueToken
        });
    }
    Ok(())
}
