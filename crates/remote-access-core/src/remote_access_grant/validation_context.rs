use ocentra_schema::remote_capability_fabric::RemoteDeviceTrustState;

use super::{
    RemoteAccessGrant, RemoteAccessGrantContext, RemoteAccessGrantError,
    RemoteAccessGrantTransition, RemoteAccessGrantTransitionAuthority,
};

impl RemoteAccessGrant {
    pub fn request_with_support_actor(
        request: super::RemoteAccessGrantRequest,
    ) -> Result<Self, RemoteAccessGrantError> {
        let grant = Self {
            grant_id: request.grant_id,
            household_ref: request.household_ref,
            child_device_ref: request.child_device_ref,
            route: request.route,
            parent_actor_ref: request.parent_actor_ref,
            support_actor_ref: request.support_actor_ref,
            capability: super::RemoteAccessGrantCapability::LiveView,
            actor_role: request.actor_role,
            state: super::RemoteAccessGrantState::Requested,
            disclosure_state: super::RemoteAccessGrantDisclosureState::Undisclosed,
            parent_grant: super::RemoteAccessGrantParentGrant::NotGranted,
            audit_ref: request.audit_ref,
            attempts: Vec::new(),
            terminal_milestone: None,
            stop_recovery_milestone: None,
            reconnect_request_recovery_milestone: None,
            restart_recovery_milestone: None,
            superseded_by: None,
            stop_recovery: super::RemoteAccessGrantStopRecoveryState::NotRequired,
            restart_recovery_at: None,
            restart_recovery_history: Vec::new(),
            pending_supersession: None,
        };
        super::validation::fields(&grant)?;
        super::validation::actor_role(&grant.actor_role)?;
        validate_support_actor(&grant)?;
        Ok(grant)
    }
}

pub(super) fn validate_support_actor(
    grant: &RemoteAccessGrant,
) -> Result<(), RemoteAccessGrantError> {
    if grant.actor_role == super::RemoteActorRole::SupportAdmin {
        if grant
            .support_actor_ref
            .as_deref()
            .is_none_or(|actor| actor.trim().is_empty())
        {
            return Err(RemoteAccessGrantError::InvalidSerializedState);
        }
    } else if grant.support_actor_ref.is_some() {
        return Err(RemoteAccessGrantError::InvalidSerializedState);
    }
    Ok(())
}

pub(super) fn context(
    grant: &RemoteAccessGrant,
    transition: RemoteAccessGrantTransition,
    context: &RemoteAccessGrantContext<'_>,
) -> Result<(), RemoteAccessGrantError> {
    if !has_valid_replay_identity(context) {
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
    let support_actor = grant.actor_role()
        == ocentra_schema::remote_capability_fabric::RemoteActorRole::SupportAdmin
        && grant.parent_grant() == super::RemoteAccessGrantParentGrant::Granted
        && grant.support_actor_ref.as_deref() == Some(context.actor_ref);
    if context.actor_ref != grant.parent_actor_ref
        && !support_actor
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
            | RemoteAccessGrantTransition::Pause
            | RemoteAccessGrantTransition::RequestReconnect
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

pub(super) fn has_valid_replay_identity(context: &RemoteAccessGrantContext<'_>) -> bool {
    !context.household_ref.trim().is_empty()
        && !context.actor_ref.trim().is_empty()
        && !context.child_device_ref.trim().is_empty()
        && !context.attempt_ref.trim().is_empty()
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
