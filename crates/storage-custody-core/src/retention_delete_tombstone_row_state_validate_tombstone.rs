use super::{
    RetentionDeleteDerivationError, RetentionDeleteDerivationInput,
    RetentionDeleteStateRequirements,
};

pub(super) fn validate_tombstone_state(
    requirements: &RetentionDeleteStateRequirements,
    input: &RetentionDeleteDerivationInput,
) -> Result<(), RetentionDeleteDerivationError> {
    if requirements.tombstone_written && input.tombstone_ref.is_none() {
        return Err(RetentionDeleteDerivationError::MissingTombstoneRef);
    }
    if requirements.requires_redaction && !input.local_payload_redacted {
        return Err(RetentionDeleteDerivationError::LocalPayloadMustBeRedacted);
    }
    Ok(())
}
