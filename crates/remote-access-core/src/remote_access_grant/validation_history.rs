use super::{
    validation, RemoteAccessGrant, RemoteAccessGrantAuditOutcome, RemoteAccessGrantError,
    RemoteAccessGrantState, RemoteAccessGrantTransition,
};

pub(super) fn validate(grant: &RemoteAccessGrant) -> Result<(), RemoteAccessGrantError> {
    if grant.attempts.is_empty() {
        // Older persisted snapshots did not retain replay milestones. Their
        // lifecycle evidence is still checked by the surrounding validators;
        // a non-empty history, when present, must be ordered and reachable.
        return Ok(());
    }
    let mut state = RemoteAccessGrantState::Requested;
    for attempt in &grant.attempts {
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
    (state == grant.state)
        .then_some(())
        .ok_or(RemoteAccessGrantError::InvalidSerializedState)
}

fn transition_allowed_from(
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
