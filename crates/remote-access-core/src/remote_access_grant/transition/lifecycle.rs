use super::super::{RemoteAccessGrant, RemoteAccessGrantError, RemoteAccessGrantState};

pub(super) fn activate(
    grant: &RemoteAccessGrant,
) -> Result<RemoteAccessGrantState, RemoteAccessGrantError> {
    if !matches!(
        grant.state,
        RemoteAccessGrantState::Paired
            | RemoteAccessGrantState::ReconnectPending
            | RemoteAccessGrantState::Paused
    ) {
        return Err(RemoteAccessGrantError::InvalidTransition);
    }
    Ok(RemoteAccessGrantState::Active)
}

pub(super) fn pause(
    grant: &RemoteAccessGrant,
) -> Result<RemoteAccessGrantState, RemoteAccessGrantError> {
    if grant.state != RemoteAccessGrantState::Active {
        return Err(RemoteAccessGrantError::InvalidTransition);
    }
    Ok(RemoteAccessGrantState::Paused)
}

pub(super) fn stop(
    grant: &RemoteAccessGrant,
) -> Result<RemoteAccessGrantState, RemoteAccessGrantError> {
    if !matches!(
        grant.state,
        RemoteAccessGrantState::Active | RemoteAccessGrantState::Paused
    ) {
        return Err(RemoteAccessGrantError::InvalidTransition);
    }
    Ok(RemoteAccessGrantState::Stopped)
}
