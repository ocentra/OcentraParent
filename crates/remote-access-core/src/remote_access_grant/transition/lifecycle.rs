use super::super::{
    RemoteAccessGrant, RemoteAccessGrantContext, RemoteAccessGrantError, RemoteAccessGrantState,
};

pub(super) fn activate(
    grant: &RemoteAccessGrant,
    context: &RemoteAccessGrantContext<'_>,
) -> Result<RemoteAccessGrantState, RemoteAccessGrantError> {
    if !matches!(
        grant.state,
        RemoteAccessGrantState::Paired | RemoteAccessGrantState::Paused
    ) {
        return Err(RemoteAccessGrantError::InvalidTransition);
    }
    if !context.parent_authorized {
        return Err(RemoteAccessGrantError::ParentAuthorityRequired);
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
    grant: &mut RemoteAccessGrant,
    context: &RemoteAccessGrantContext<'_>,
) -> Result<RemoteAccessGrantState, RemoteAccessGrantError> {
    if !matches!(
        grant.state,
        RemoteAccessGrantState::Active | RemoteAccessGrantState::Paused
    ) {
        return Err(RemoteAccessGrantError::InvalidTransition);
    }
    if !context.parent_authorized
        && context.transition_authority
            != super::super::RemoteAccessGrantTransitionAuthority::SystemFailure
    {
        return Err(RemoteAccessGrantError::ParentAuthorityRequired);
    }
    grant.stop_recovery = if context.transition_authority
        == super::super::RemoteAccessGrantTransitionAuthority::SystemFailure
    {
        super::super::RemoteAccessGrantStopRecoveryState::Pending
    } else {
        super::super::RemoteAccessGrantStopRecoveryState::NotRequired
    };
    Ok(RemoteAccessGrantState::Stopped)
}
