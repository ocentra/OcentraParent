use ocentra_schema::remote_capability_fabric::RemoteDeviceTrustState;

use super::{
    RemoteAccessGrant, RemoteAccessGrantContext, RemoteAccessGrantError,
    RemoteAccessGrantTransition, RemoteAccessGrantTransitionAuthority,
};

pub(super) fn context(
    grant: &RemoteAccessGrant,
    transition: RemoteAccessGrantTransition,
    context: &RemoteAccessGrantContext<'_>,
) -> Result<(), RemoteAccessGrantError> {
    if context.attempt_ref.trim().is_empty() {
        return Err(RemoteAccessGrantError::EmptyField);
    }
    if context.household_ref != grant.household_ref {
        return Err(RemoteAccessGrantError::WrongHousehold);
    }
    let system_failure =
        context.transition_authority == RemoteAccessGrantTransitionAuthority::SystemFailure;
    if system_failure
        && !matches!(
            transition,
            RemoteAccessGrantTransition::Revoke
                | RemoteAccessGrantTransition::RemoveDevice
                | RemoteAccessGrantTransition::Stop
                | RemoteAccessGrantTransition::Fail
        )
    {
        return Err(RemoteAccessGrantError::WrongActor);
    }
    if context.actor_ref != grant.parent_actor_ref
        && !(system_failure || (context.parent_authorized && is_parent_terminal(transition)))
    {
        return Err(RemoteAccessGrantError::WrongActor);
    }
    if context.child_device_ref != grant.child_device_ref {
        return Err(RemoteAccessGrantError::WrongDevice);
    }
    if matches!(
        transition,
        RemoteAccessGrantTransition::Pair
            | RemoteAccessGrantTransition::Activate
            | RemoteAccessGrantTransition::Reconnect
    ) && !context.parent_authorized
    {
        return Err(RemoteAccessGrantError::ParentAuthorityRequired);
    }
    if transition != RemoteAccessGrantTransition::RemoveDevice && context.route != grant.route {
        return Err(RemoteAccessGrantError::WrongRoute);
    }
    if matches!(
        transition,
        RemoteAccessGrantTransition::Pair
            | RemoteAccessGrantTransition::Activate
            | RemoteAccessGrantTransition::Reconnect
    ) && context.device_trust_state != RemoteDeviceTrustState::Trusted
    {
        return Err(RemoteAccessGrantError::DeviceTrustRequired);
    }
    Ok(())
}

fn is_parent_terminal(transition: RemoteAccessGrantTransition) -> bool {
    matches!(
        transition,
        RemoteAccessGrantTransition::Revoke
            | RemoteAccessGrantTransition::RemoveDevice
            | RemoteAccessGrantTransition::Stop
            | RemoteAccessGrantTransition::Deny
            | RemoteAccessGrantTransition::Fail
            | RemoteAccessGrantTransition::Supersede
    )
}
