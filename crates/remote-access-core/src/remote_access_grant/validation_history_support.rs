use super::{
    validation, validation_history, RemoteAccessGrant, RemoteAccessGrantAuditOutcome,
    RemoteAccessGrantError, RemoteAccessGrantState,
};

pub(super) fn empty_history(grant: &RemoteAccessGrant) -> Result<bool, RemoteAccessGrantError> {
    if !grant.attempts.is_empty() || grant.terminal_milestone.is_some() {
        return Ok(false);
    }
    match grant.restart_recovery_at {
        None => Ok(true),
        Some(0) if grant.state == RemoteAccessGrantState::ReconnectPending => Ok(true),
        Some(_) => Err(RemoteAccessGrantError::InvalidSerializedState),
    }
}

pub(super) fn before_attempt(
    grant: &RemoteAccessGrant,
    state: RemoteAccessGrantState,
    index: usize,
) -> Result<RemoteAccessGrantState, RemoteAccessGrantError> {
    if grant.restart_recovery_at != Some(index) {
        return Ok(state);
    }
    (state == RemoteAccessGrantState::Active)
        .then_some(RemoteAccessGrantState::ReconnectPending)
        .ok_or(RemoteAccessGrantError::InvalidSerializedState)
}

pub(super) fn after_attempts(
    grant: &RemoteAccessGrant,
    state: RemoteAccessGrantState,
) -> Result<RemoteAccessGrantState, RemoteAccessGrantError> {
    if grant.restart_recovery_at != Some(grant.attempts.len()) {
        return Ok(state);
    }
    (state == RemoteAccessGrantState::Active)
        .then_some(RemoteAccessGrantState::ReconnectPending)
        .ok_or(RemoteAccessGrantError::InvalidSerializedState)
}

pub(super) fn terminal(
    grant: &RemoteAccessGrant,
    state: RemoteAccessGrantState,
) -> Result<RemoteAccessGrantState, RemoteAccessGrantError> {
    let Some(attempt) = grant.terminal_milestone.as_ref() else {
        return Ok(state);
    };
    if !validation_history::transition_allowed_from(state, attempt.transition)
        || attempt.outcome != RemoteAccessGrantAuditOutcome::Accepted
        || validation::accepted_resulting_state(attempt.transition) != attempt.resulting_state
    {
        return Err(RemoteAccessGrantError::InvalidSerializedState);
    }
    Ok(attempt.resulting_state)
}
