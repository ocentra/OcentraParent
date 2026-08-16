use ocentra_schema::retention_delete_tombstone as contracts;

use super::{RetentionDeleteDerivationError, RetentionDeleteDerivationInput};
use crate::retention_delete_tombstone::RetentionDeleteSignal;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct RetentionDeleteStateRequirements {
    pub tombstone_written: bool,
    pub requires_redaction: bool,
    pub requires_propagation: bool,
    pub requires_replay_protection: bool,
    pub requires_minimal_audit: bool,
}

#[path = "retention_delete_tombstone_row_state_class.rs"]
mod retention_delete_tombstone_row_state_class;
#[path = "retention_delete_tombstone_row_state_requirements.rs"]
mod retention_delete_tombstone_row_state_requirements;
#[path = "retention_delete_tombstone_row_state_signal.rs"]
mod retention_delete_tombstone_row_state_signal;
#[path = "retention_delete_tombstone_row_state_validate.rs"]
mod retention_delete_tombstone_row_state_validate;

pub(super) fn retention_delete_requirements(
    state: contracts::RetentionDeleteState,
) -> RetentionDeleteStateRequirements {
    retention_delete_tombstone_row_state_requirements::retention_delete_requirements(state)
}

pub(super) fn validate_retention_delete_state_requirements(
    state: contracts::RetentionDeleteState,
    requirements: &RetentionDeleteStateRequirements,
    input: &RetentionDeleteDerivationInput,
) -> Result<(), RetentionDeleteDerivationError> {
    retention_delete_tombstone_row_state_validate::validate_retention_delete_state_requirements(
        state,
        requirements,
        input,
    )
}

pub(super) fn signal_state(signal: RetentionDeleteSignal) -> contracts::RetentionDeleteState {
    retention_delete_tombstone_row_state_signal::signal_state(signal)
}

pub(super) fn state_retention_class(
    state: contracts::RetentionDeleteState,
) -> contracts::RetentionDeleteRetentionClass {
    retention_delete_tombstone_row_state_class::state_retention_class(state)
}
