use super::super::{
    RemoteAccessGrant, RemoteAccessGrantContext, RemoteAccessGrantDisclosureState,
    RemoteAccessGrantError, RemoteAccessGrantParentGrant, RemoteAccessGrantState,
};
use ocentra_schema::remote_capability_fabric::RemoteActorRole;

pub(super) fn parent(
    grant: &mut RemoteAccessGrant,
    context: &RemoteAccessGrantContext<'_>,
) -> Result<RemoteAccessGrantState, RemoteAccessGrantError> {
    if grant.state != RemoteAccessGrantState::Requested {
        return Err(RemoteAccessGrantError::InvalidTransition);
    }
    if !context.parent_authorized {
        return Err(RemoteAccessGrantError::ParentAuthorityRequired);
    }
    if grant.actor_role() == RemoteActorRole::SupportAdmin && !context.parent_grant_approved {
        return Err(RemoteAccessGrantError::SupportAccessRequiresParentGrant);
    }
    if grant.actor_role() == RemoteActorRole::SupportAdmin && grant.support_actor_ref.is_none() {
        return Err(RemoteAccessGrantError::InvalidSerializedState);
    }
    grant.parent_grant = RemoteAccessGrantParentGrant::Granted;
    Ok(RemoteAccessGrantState::ParentConfirmed)
}

pub(super) fn pair(
    grant: &mut RemoteAccessGrant,
    context: &RemoteAccessGrantContext<'_>,
) -> Result<RemoteAccessGrantState, RemoteAccessGrantError> {
    if grant.state != RemoteAccessGrantState::ParentConfirmed {
        return Err(RemoteAccessGrantError::InvalidTransition);
    }
    if !context.parent_authorized {
        return Err(RemoteAccessGrantError::ParentAuthorityRequired);
    }
    if !context.child_disclosed {
        return Err(RemoteAccessGrantError::ChildDisclosureRequired);
    }
    grant.disclosure_state = RemoteAccessGrantDisclosureState::Disclosed;
    Ok(RemoteAccessGrantState::Paired)
}
