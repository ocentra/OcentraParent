use ocentra_schema::retention_delete_tombstone as contracts;

use super::{
    RetentionDeleteDerivationError, RetentionDeleteDerivationInput,
    RetentionDeleteStateRequirements,
};

pub(super) fn validate_propagation_state(
    state: contracts::RetentionDeleteState,
    requirements: &RetentionDeleteStateRequirements,
    input: &RetentionDeleteDerivationInput,
) -> Result<(), RetentionDeleteDerivationError> {
    if matches!(state, contracts::RetentionDeleteState::PropagationPending)
        && input.propagation_complete
    {
        return Err(RetentionDeleteDerivationError::PropagationStillPending);
    }
    if requirements.requires_propagation && !input.propagation_complete {
        return Err(RetentionDeleteDerivationError::PropagationStillPending);
    }
    if requirements.requires_replay_protection && input.replay_ref.is_none() {
        return Err(RetentionDeleteDerivationError::MissingReplayRef);
    }
    if requirements.requires_replay_protection && !input.replay_blocked {
        return Err(RetentionDeleteDerivationError::ReplayProtectionRequired);
    }
    Ok(())
}
