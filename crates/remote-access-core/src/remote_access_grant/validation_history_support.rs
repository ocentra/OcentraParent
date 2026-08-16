use super::{
    validation, validation_history, RemoteAccessGrant, RemoteAccessGrantAuditOutcome,
    RemoteAccessGrantError, RemoteAccessGrantState,
};

pub(super) fn empty_history(grant: &RemoteAccessGrant) -> Result<bool, RemoteAccessGrantError> {
    if !grant.attempts.is_empty()
        || grant.terminal_milestone.is_some()
        || grant.stop_recovery_milestone.is_some()
        || grant.reconnect_request_recovery_milestone.is_some()
        || grant.restart_recovery_milestone.is_some()
        || !grant.restart_recovery_history.is_empty()
    {
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
    if !has_restart_boundary(grant, index) {
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
    if !has_restart_boundary(grant, grant.attempts.len()) {
        return Ok(state);
    }
    (state == RemoteAccessGrantState::Active)
        .then_some(RemoteAccessGrantState::ReconnectPending)
        .ok_or(RemoteAccessGrantError::InvalidSerializedState)
}

fn has_restart_boundary(grant: &RemoteAccessGrant, index: usize) -> bool {
    grant.restart_recovery_at == Some(index) || grant.restart_recovery_history.contains(&index)
}

pub(super) fn stop_recovery(
    grant: &RemoteAccessGrant,
    state: RemoteAccessGrantState,
) -> Result<RemoteAccessGrantState, RemoteAccessGrantError> {
    let Some(attempt) = grant.stop_recovery_milestone.as_ref() else {
        return Ok(state);
    };
    if attempt.transition != super::RemoteAccessGrantTransition::Stop
        || attempt.outcome != RemoteAccessGrantAuditOutcome::Accepted
        || attempt.error.is_some()
        || (grant.stop_recovery != super::RemoteAccessGrantStopRecoveryState::Pending
            && grant.reconnect_request_recovery_milestone.is_none())
        || !validation_history::transition_allowed_from(state, attempt.transition)
        || validation::accepted_resulting_state(attempt.transition) != attempt.resulting_state
        || attempt.resulting_state != RemoteAccessGrantState::Stopped
        || attempt.grant_id != grant.grant_id
        || attempt.audit_ref != grant.audit_ref
        || attempt.household_ref != grant.household_ref
        || attempt.child_device_ref != grant.child_device_ref
        || attempt.route != grant.route
        || attempt.replacement_grant_id.is_some()
    {
        return Err(RemoteAccessGrantError::InvalidSerializedState);
    }
    Ok(attempt.resulting_state)
}

pub(super) fn restart_recovery(
    grant: &RemoteAccessGrant,
    state: RemoteAccessGrantState,
) -> Result<RemoteAccessGrantState, RemoteAccessGrantError> {
    let Some(attempt) = grant.restart_recovery_milestone.as_ref() else {
        return Ok(state);
    };
    if attempt.transition != super::RemoteAccessGrantTransition::Reconnect
        || attempt.outcome != RemoteAccessGrantAuditOutcome::Accepted
        || attempt.error.is_some()
        || state != RemoteAccessGrantState::ReconnectPending
        || (grant.restart_recovery_at != Some(grant.attempts.len())
            && grant.reconnect_request_recovery_milestone.is_none())
        || attempt.resulting_state != RemoteAccessGrantState::Active
        || attempt.grant_id != grant.grant_id
        || attempt.audit_ref != grant.audit_ref
        || attempt.household_ref != grant.household_ref
        || attempt.child_device_ref != grant.child_device_ref
        || attempt.route != grant.route
        || attempt.replacement_grant_id.is_some()
    {
        return Err(RemoteAccessGrantError::InvalidSerializedState);
    }
    Ok(attempt.resulting_state)
}

pub(super) fn reconnect_request_recovery(
    grant: &RemoteAccessGrant,
    state: RemoteAccessGrantState,
) -> Result<RemoteAccessGrantState, RemoteAccessGrantError> {
    let Some(attempt) = grant.reconnect_request_recovery_milestone.as_ref() else {
        return Ok(state);
    };
    let recovery_completed = grant.restart_recovery_milestone.is_some();
    if attempt.transition != super::RemoteAccessGrantTransition::RequestReconnect
        || attempt.outcome != RemoteAccessGrantAuditOutcome::Accepted
        || attempt.error.is_some()
        || state != RemoteAccessGrantState::Stopped
        || (grant.stop_recovery != super::RemoteAccessGrantStopRecoveryState::Pending
            && !recovery_completed)
        || attempt.resulting_state != RemoteAccessGrantState::ReconnectPending
        || attempt.grant_id != grant.grant_id
        || attempt.audit_ref != grant.audit_ref
        || attempt.household_ref != grant.household_ref
        || attempt.child_device_ref != grant.child_device_ref
        || attempt.route != grant.route
        || attempt.replacement_grant_id.is_some()
    {
        return Err(RemoteAccessGrantError::InvalidSerializedState);
    }
    Ok(attempt.resulting_state)
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
