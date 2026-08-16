use ocentra_schema::retention_delete_tombstone as contracts;

use super::{
    RetentionDeleteDerivationError, RetentionDeleteDerivationInput,
    RetentionDeleteStateRequirements,
};

pub(super) fn validate_audit_state(
    state: contracts::RetentionDeleteState,
    requirements: &RetentionDeleteStateRequirements,
    input: &RetentionDeleteDerivationInput,
) -> Result<(), RetentionDeleteDerivationError> {
    if requirements.requires_minimal_audit && !input.audit_payload_redacted {
        return Err(RetentionDeleteDerivationError::AuditMustBeMinimal);
    }
    if state == contracts::RetentionDeleteState::HardDeleted && !input.hard_delete_eligible {
        return Err(RetentionDeleteDerivationError::HardDeleteNotEligible);
    }
    Ok(())
}
