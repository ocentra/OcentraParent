use crate::constants::OPAQUE_TOKEN_BYTES;
use crate::types::ProtocolError;

use super::super::{ResponseStatus, UntrustedResponseFacts};

pub(crate) fn validate_result(facts: &UntrustedResponseFacts) -> Result<(), ProtocolError> {
    if !facts.status.is_compatible_with(facts.request_kind) {
        return Err(ProtocolError::InvalidStatusForRequest);
    }
    if matches!(facts.status, ResponseStatus::Prepared)
        && facts.opaque_token.len() != OPAQUE_TOKEN_BYTES
    {
        return Err(ProtocolError::InvalidOpaqueToken);
    }
    if !matches!(facts.status, ResponseStatus::Prepared) && !facts.opaque_token.is_empty() {
        return Err(ProtocolError::UnexpectedOpaqueToken);
    }
    Ok(())
}
