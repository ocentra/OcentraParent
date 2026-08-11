use super::validation_history_support;
use super::{
    validation, RemoteAccessGrant, RemoteAccessGrantAuditOutcome, RemoteAccessGrantError,
    RemoteAccessGrantState, RemoteAccessGrantTransition,
};

pub(super) fn validate(grant: &RemoteAccessGrant) -> Result<(), RemoteAccessGrantError> {
    if validation_history_support::empty_history(grant)? {
        return Ok(());
    }
    let mut state = RemoteAccessGrantState::Requested;
    for (index, attempt) in grant.attempts.iter().enumerate() {
        state = validation_history_support::before_attempt(grant, state, index)?;
        if attempt.outcome == RemoteAccessGrantAuditOutcome::Denied {
            if attempt.resulting_state != state {
                return Err(RemoteAccessGrantError::InvalidSerializedState);
            }
            continue;
        }
        if !transition_allowed_from(state, attempt.transition)
            || validation::accepted_resulting_state(attempt.transition) != attempt.resulting_state
        {
            return Err(RemoteAccessGrantError::InvalidSerializedState);
        }
        state = attempt.resulting_state;
    }
    state = validation_history_support::after_attempts(grant, state)?;
    state = validation_history_support::restart_recovery(grant, state)?;
    state = validation_history_support::stop_recovery(grant, state)?;
    state = validation_history_support::terminal(grant, state)?;
    (state == grant.state)
        .then_some(())
        .ok_or(RemoteAccessGrantError::InvalidSerializedState)
}

pub(super) fn transition_allowed_from(
    state: RemoteAccessGrantState,
    transition: RemoteAccessGrantTransition,
) -> bool {
    (ALLOWED_PREDECESSORS[transition as usize] & (1 << state as u16)) != 0
}

const NON_TERMINAL_STATES: u16 = (1 << RemoteAccessGrantState::Requested as u16)
    | (1 << RemoteAccessGrantState::ParentConfirmed as u16)
    | (1 << RemoteAccessGrantState::Paired as u16)
    | (1 << RemoteAccessGrantState::Active as u16)
    | (1 << RemoteAccessGrantState::Paused as u16)
    | (1 << RemoteAccessGrantState::Stopped as u16)
    | (1 << RemoteAccessGrantState::ReconnectPending as u16);

const ALLOWED_PREDECESSORS: [u16; 12] = [
    1 << RemoteAccessGrantState::Requested as u16,
    1 << RemoteAccessGrantState::ParentConfirmed as u16,
    (1 << RemoteAccessGrantState::Paired as u16) | (1 << RemoteAccessGrantState::Paused as u16),
    1 << RemoteAccessGrantState::Active as u16,
    (1 << RemoteAccessGrantState::Paired as u16)
        | (1 << RemoteAccessGrantState::Active as u16)
        | (1 << RemoteAccessGrantState::Paused as u16)
        | (1 << RemoteAccessGrantState::ReconnectPending as u16),
    (1 << RemoteAccessGrantState::Paused as u16) | (1 << RemoteAccessGrantState::Stopped as u16),
    1 << RemoteAccessGrantState::ReconnectPending as u16,
    NON_TERMINAL_STATES,
    NON_TERMINAL_STATES,
    NON_TERMINAL_STATES,
    NON_TERMINAL_STATES,
    NON_TERMINAL_STATES,
];
