use super::super::{
    RemoteAccessGrant, RemoteAccessGrantActorRole, RemoteAccessGrantContext,
    RemoteAccessGrantDisclosureState, RemoteAccessGrantError, RemoteAccessGrantState,
};

pub(super) fn parent(
    grant: &RemoteAccessGrant,
    context: RemoteAccessGrantContext<'_>,
) -> Result<RemoteAccessGrantState, RemoteAccessGrantError> {
    if grant.state != RemoteAccessGrantState::Requested {
        return Err(RemoteAccessGrantError::InvalidTransition);
    }
    if !context.parent_authorized {
        return Err(RemoteAccessGrantError::ParentAuthorityRequired);
    }
    if grant.actor_role == RemoteAccessGrantActorRole::SupportAdmin {
        return Err(RemoteAccessGrantError::SupportAccessRequiresParentGrant);
    }
    Ok(RemoteAccessGrantState::ParentConfirmed)
}

pub(super) fn pair(
    grant: &mut RemoteAccessGrant,
    context: RemoteAccessGrantContext<'_>,
) -> Result<RemoteAccessGrantState, RemoteAccessGrantError> {
    if grant.state != RemoteAccessGrantState::ParentConfirmed {
        return Err(RemoteAccessGrantError::InvalidTransition);
    }
    if !context.child_disclosed {
        return Err(RemoteAccessGrantError::ChildDisclosureRequired);
    }
    grant.disclosure_state = RemoteAccessGrantDisclosureState::Disclosed;
    Ok(RemoteAccessGrantState::Paired)
}
