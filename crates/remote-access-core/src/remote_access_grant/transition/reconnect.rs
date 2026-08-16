use super::super::{
    RemoteAccessGrant, RemoteAccessGrantContext, RemoteAccessGrantDisclosureState,
    RemoteAccessGrantError, RemoteAccessGrantRecoveryProof, RemoteAccessGrantState,
    RemoteAccessGrantStopRecoveryState,
};

pub(super) fn request(
    grant: &RemoteAccessGrant,
    context: &RemoteAccessGrantContext<'_>,
) -> Result<RemoteAccessGrantState, RemoteAccessGrantError> {
    if !matches!(
        grant.state,
        RemoteAccessGrantState::Paused | RemoteAccessGrantState::Stopped
    ) {
        return Err(RemoteAccessGrantError::ReconnectDenied);
    }
    if !context.parent_authorized {
        return Err(RemoteAccessGrantError::ParentAuthorityRequired);
    }
    Ok(RemoteAccessGrantState::ReconnectPending)
}

pub(super) fn complete(
    grant: &mut RemoteAccessGrant,
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
    if grant.stop_recovery == RemoteAccessGrantStopRecoveryState::Pending
        && context.recovery_proof != RemoteAccessGrantRecoveryProof::SystemConditionCleared
    {
        return Err(RemoteAccessGrantError::ReconnectDenied);
    }
    grant.stop_recovery = RemoteAccessGrantStopRecoveryState::NotRequired;
    Ok(RemoteAccessGrantState::Active)
}
