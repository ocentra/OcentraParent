use super::super::{
    RemoteAccessGrant, RemoteAccessGrantContext, RemoteAccessGrantDisclosureState,
    RemoteAccessGrantError, RemoteAccessGrantState,
};

pub(super) fn request(
    grant: &RemoteAccessGrant,
) -> Result<RemoteAccessGrantState, RemoteAccessGrantError> {
    if !matches!(
        grant.state,
        RemoteAccessGrantState::Paused | RemoteAccessGrantState::Stopped
    ) {
        return Err(RemoteAccessGrantError::ReconnectDenied);
    }
    Ok(RemoteAccessGrantState::ReconnectPending)
}

pub(super) fn complete(
    grant: &RemoteAccessGrant,
    context: &RemoteAccessGrantContext<'_>,
) -> Result<RemoteAccessGrantState, RemoteAccessGrantError> {
    if grant.state != RemoteAccessGrantState::ReconnectPending
        || grant.disclosure_state != RemoteAccessGrantDisclosureState::Disclosed
    {
        return Err(RemoteAccessGrantError::ReconnectDenied);
    }
    if !context.parent_authorized {
        return Err(RemoteAccessGrantError::ParentAuthorityRequired);
    }
    Ok(RemoteAccessGrantState::Active)
}
