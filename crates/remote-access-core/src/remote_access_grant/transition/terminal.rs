use super::super::{
    RemoteAccessGrant, RemoteAccessGrantContext, RemoteAccessGrantError, RemoteAccessGrantState,
};

pub(super) fn revoke(
    grant: &RemoteAccessGrant,
    context: &RemoteAccessGrantContext<'_>,
) -> Result<RemoteAccessGrantState, RemoteAccessGrantError> {
    require_parent_authority(context)?;
    terminal_state(grant)?;
    Ok(RemoteAccessGrantState::Revoked)
}

pub(super) fn remove_device(
    grant: &RemoteAccessGrant,
    context: &RemoteAccessGrantContext<'_>,
) -> Result<RemoteAccessGrantState, RemoteAccessGrantError> {
    require_parent_authority(context)?;
    terminal_state(grant)?;
    Ok(RemoteAccessGrantState::Removed)
}

pub(super) fn deny(
    grant: &RemoteAccessGrant,
    context: &RemoteAccessGrantContext<'_>,
) -> Result<RemoteAccessGrantState, RemoteAccessGrantError> {
    require_parent_authority(context)?;
    terminal_state(grant)?;
    Ok(RemoteAccessGrantState::Denied)
}

pub(super) fn fail(
    grant: &RemoteAccessGrant,
    context: &RemoteAccessGrantContext<'_>,
) -> Result<RemoteAccessGrantState, RemoteAccessGrantError> {
    require_parent_authority(context)?;
    terminal_state(grant)?;
    Ok(RemoteAccessGrantState::Failed)
}

fn require_parent_authority(
    context: &RemoteAccessGrantContext<'_>,
) -> Result<(), RemoteAccessGrantError> {
    context
        .parent_authorized
        .then_some(())
        .ok_or(RemoteAccessGrantError::ParentAuthorityRequired)
}

fn terminal_state(grant: &RemoteAccessGrant) -> Result<(), RemoteAccessGrantError> {
    (!matches!(
        grant.state,
        RemoteAccessGrantState::Revoked
            | RemoteAccessGrantState::Removed
            | RemoteAccessGrantState::Denied
            | RemoteAccessGrantState::Failed
    ))
    .then_some(())
    .ok_or(RemoteAccessGrantError::InvalidTransition)
}
