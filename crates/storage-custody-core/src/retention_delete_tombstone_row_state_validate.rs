use ocentra_schema::retention_delete_tombstone as contracts;

use super::{
    RetentionDeleteDerivationError, RetentionDeleteDerivationInput,
    RetentionDeleteStateRequirements,
};

pub(super) fn validate_retention_delete_state_requirements(
    state: contracts::RetentionDeleteState,
    requirements: &RetentionDeleteStateRequirements,
    input: &RetentionDeleteDerivationInput,
) -> Result<(), RetentionDeleteDerivationError> {
    self::retention_delete_tombstone_row_state_validate_tombstone::validate_tombstone_state(
        requirements,
        input,
    )?;
    self::retention_delete_tombstone_row_state_validate_propagation::validate_propagation_state(
        state,
        requirements,
        input,
    )?;
    self::retention_delete_tombstone_row_state_validate_audit::validate_audit_state(
        state,
        requirements,
        input,
    )?;
    Ok(())
}

#[path = "retention_delete_tombstone_row_state_validate_audit.rs"]
mod retention_delete_tombstone_row_state_validate_audit;
#[path = "retention_delete_tombstone_row_state_validate_propagation.rs"]
mod retention_delete_tombstone_row_state_validate_propagation;
#[path = "retention_delete_tombstone_row_state_validate_tombstone.rs"]
mod retention_delete_tombstone_row_state_validate_tombstone;
