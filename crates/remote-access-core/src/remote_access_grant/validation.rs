use super::{RemoteAccessGrant, RemoteAccessGrantContext, RemoteAccessGrantError};

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

pub(super) fn context(
    grant: &RemoteAccessGrant,
    context: RemoteAccessGrantContext<'_>,
) -> Result<(), RemoteAccessGrantError> {
    if context.household_ref != grant.household_ref {
        return Err(RemoteAccessGrantError::WrongHousehold);
    }
    if context.actor_ref != grant.parent_actor_ref {
        return Err(RemoteAccessGrantError::WrongActor);
    }
    if context.child_device_ref != grant.child_device_ref {
        return Err(RemoteAccessGrantError::WrongDevice);
    }
    Ok(())
}
