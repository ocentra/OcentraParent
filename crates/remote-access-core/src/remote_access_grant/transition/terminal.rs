use super::super::{
    RemoteAccessGrant, RemoteAccessGrantContext, RemoteAccessGrantError, RemoteAccessGrantState,
    RemoteAccessGrantTransitionAuthority,
};

pub(super) fn apply(
    grant: &mut RemoteAccessGrant,
    transition: super::super::RemoteAccessGrantTransition,
    context: &RemoteAccessGrantContext<'_>,
) -> Result<RemoteAccessGrantState, RemoteAccessGrantError> {
    match transition {
        super::super::RemoteAccessGrantTransition::Revoke => revoke(grant, context),
        super::super::RemoteAccessGrantTransition::RemoveDevice => remove_device(grant, context),
        super::super::RemoteAccessGrantTransition::Deny => deny(grant, context),
        super::super::RemoteAccessGrantTransition::Fail => fail(grant, context),
        super::super::RemoteAccessGrantTransition::Supersede => supersede(grant, context),
        _ => Err(RemoteAccessGrantError::InvalidTransition),
    }
}

pub(super) fn revoke(
    grant: &mut RemoteAccessGrant,
    context: &RemoteAccessGrantContext<'_>,
) -> Result<RemoteAccessGrantState, RemoteAccessGrantError> {
    require_safety_authority(context)?;
    terminal_state(grant)?;
    Ok(RemoteAccessGrantState::Revoked)
}

pub(super) fn remove_device(
    grant: &mut RemoteAccessGrant,
    context: &RemoteAccessGrantContext<'_>,
) -> Result<RemoteAccessGrantState, RemoteAccessGrantError> {
    require_safety_authority(context)?;
    terminal_state(grant)?;
    Ok(RemoteAccessGrantState::Removed)
}

pub(super) fn deny(
    grant: &mut RemoteAccessGrant,
    context: &RemoteAccessGrantContext<'_>,
) -> Result<RemoteAccessGrantState, RemoteAccessGrantError> {
    require_parent_authority(context)?;
    terminal_state(grant)?;
    Ok(RemoteAccessGrantState::Denied)
}

pub(super) fn fail(
    grant: &mut RemoteAccessGrant,
    context: &RemoteAccessGrantContext<'_>,
) -> Result<RemoteAccessGrantState, RemoteAccessGrantError> {
    if context.transition_authority != RemoteAccessGrantTransitionAuthority::SystemFailure {
        require_parent_authority(context)?;
    }
    terminal_state(grant)?;
    Ok(RemoteAccessGrantState::Failed)
}

pub(super) fn supersede(
    grant: &mut RemoteAccessGrant,
    context: &RemoteAccessGrantContext<'_>,
) -> Result<RemoteAccessGrantState, RemoteAccessGrantError> {
    require_parent_authority(context)?;
    let replacement = grant
        .pending_supersession
        .as_deref()
        .filter(|replacement| !replacement.trim().is_empty())
        .map(str::to_owned)
        .ok_or(RemoteAccessGrantError::SupersedingGrantRequired)?;
    terminal_state(grant)?;
    grant.superseded_by = Some(replacement);
    Ok(RemoteAccessGrantState::Superseded)
}

fn require_safety_authority(
    context: &RemoteAccessGrantContext<'_>,
) -> Result<(), RemoteAccessGrantError> {
    (context.parent_authorized
        || context.transition_authority == RemoteAccessGrantTransitionAuthority::SystemFailure)
        .then_some(())
        .ok_or(RemoteAccessGrantError::ParentAuthorityRequired)
}

fn require_parent_authority(
    context: &RemoteAccessGrantContext<'_>,
) -> Result<(), RemoteAccessGrantError> {
    context
        .parent_authorized
        .then_some(())
        .ok_or(RemoteAccessGrantError::ParentAuthorityRequired)
}

fn terminal_state(grant: &mut RemoteAccessGrant) -> Result<(), RemoteAccessGrantError> {
    if matches!(
        grant.state,
        RemoteAccessGrantState::Revoked
            | RemoteAccessGrantState::Removed
            | RemoteAccessGrantState::Denied
            | RemoteAccessGrantState::Failed
            | RemoteAccessGrantState::Superseded
    ) {
        return Err(RemoteAccessGrantError::InvalidTransition);
    }
    grant.stop_recovery = super::super::RemoteAccessGrantStopRecoveryState::NotRequired;
    grant.restart_recovery_at = None;
    Ok(())
}
