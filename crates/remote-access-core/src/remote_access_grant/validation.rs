use ocentra_schema::remote_capability_fabric::RemoteActorRole;

use super::{
    RemoteAccessGrant, RemoteAccessGrantContext, RemoteAccessGrantDisclosureState,
    RemoteAccessGrantError, RemoteAccessGrantParentGrant, RemoteAccessGrantState,
    RemoteAccessGrantTransition,
};

pub(super) fn fields(grant: &RemoteAccessGrant) -> Result<(), RemoteAccessGrantError> {
    if [
        grant.grant_id.as_str(),
        grant.household_ref.as_str(),
        grant.child_device_ref.as_str(),
        grant.parent_actor_ref.as_str(),
        grant.audit_ref.as_str(),
    ]
    .into_iter()
    .any(|value| value.trim().is_empty())
    {
        return Err(RemoteAccessGrantError::EmptyField);
    }
    Ok(())
}

pub(super) fn actor_role(role: &RemoteActorRole) -> Result<(), RemoteAccessGrantError> {
    if role == &RemoteActorRole::ChildAgent {
        return Err(RemoteAccessGrantError::WrongActor);
    }
    Ok(())
}

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
    if context.actor_ref != grant.parent_actor_ref
        && (!context.parent_authorized
            || !matches!(
                transition,
                RemoteAccessGrantTransition::Revoke | RemoteAccessGrantTransition::RemoveDevice
            ))
    {
        return Err(RemoteAccessGrantError::WrongActor);
    }
    if context.child_device_ref != grant.child_device_ref {
        return Err(RemoteAccessGrantError::WrongDevice);
    }
    if context.route != grant.route {
        return Err(RemoteAccessGrantError::WrongRoute);
    }
    Ok(())
}

pub(super) fn serialized(grant: &RemoteAccessGrant) -> Result<(), RemoteAccessGrantError> {
    fields(grant)?;
    actor_role(&grant.actor_role)?;
    let terminal = matches!(
        grant.state,
        RemoteAccessGrantState::Revoked | RemoteAccessGrantState::Removed
    );
    if !terminal {
        let expected_disclosure = [
            RemoteAccessGrantDisclosureState::Undisclosed,
            RemoteAccessGrantDisclosureState::Undisclosed,
            RemoteAccessGrantDisclosureState::Disclosed,
            RemoteAccessGrantDisclosureState::Disclosed,
            RemoteAccessGrantDisclosureState::Disclosed,
            RemoteAccessGrantDisclosureState::Disclosed,
            RemoteAccessGrantDisclosureState::Disclosed,
            RemoteAccessGrantDisclosureState::Disclosed,
            RemoteAccessGrantDisclosureState::Disclosed,
        ][grant.state as usize];
        let expected_parent_grant = [
            RemoteAccessGrantParentGrant::NotGranted,
            RemoteAccessGrantParentGrant::Granted,
            RemoteAccessGrantParentGrant::Granted,
            RemoteAccessGrantParentGrant::Granted,
            RemoteAccessGrantParentGrant::Granted,
            RemoteAccessGrantParentGrant::Granted,
            RemoteAccessGrantParentGrant::Granted,
            RemoteAccessGrantParentGrant::Granted,
            RemoteAccessGrantParentGrant::Granted,
        ][grant.state as usize];
        if grant.disclosure_state != expected_disclosure
            || grant.parent_grant != expected_parent_grant
        {
            return Err(RemoteAccessGrantError::InvalidSerializedState);
        }
    }
    if grant.disclosure_state == RemoteAccessGrantDisclosureState::Disclosed
        && grant.parent_grant != RemoteAccessGrantParentGrant::Granted
    {
        return Err(RemoteAccessGrantError::InvalidSerializedState);
    }
    Ok(())
}
