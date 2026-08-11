use ocentra_schema::remote_capability_fabric::RemoteActorRole;

use super::{
    RemoteAccessGrant, RemoteAccessGrantError, RemoteAccessGrantState,
    RemoteAccessGrantStopRecoveryState,
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

pub(super) fn serialized(grant: &RemoteAccessGrant) -> Result<(), RemoteAccessGrantError> {
    fields(grant)?;
    actor_role(&grant.actor_role)?;
    super::validation_context::validate_support_actor(grant)?;
    super::validation_attempts::validate(grant)?;
    super::validation_terminal::validate(grant)?;
    super::validation_history::validate(grant)?;
    validate_supersession(grant)?;
    super::validation_lifecycle::validate(grant)?;
    validate_recovery(grant)
}

fn validate_supersession(grant: &RemoteAccessGrant) -> Result<(), RemoteAccessGrantError> {
    if grant.state == RemoteAccessGrantState::Superseded
        && grant
            .superseded_by
            .as_deref()
            .is_none_or(|replacement| replacement.trim().is_empty())
    {
        return Err(RemoteAccessGrantError::InvalidSerializedState);
    }
    if grant.state != RemoteAccessGrantState::Superseded && grant.superseded_by.is_some() {
        return Err(RemoteAccessGrantError::InvalidSerializedState);
    }
    Ok(())
}

fn validate_recovery(grant: &RemoteAccessGrant) -> Result<(), RemoteAccessGrantError> {
    if is_terminal(grant.state) {
        return grant
            .restart_recovery_at
            .is_none_or(|index| index <= grant.attempts.len())
            .then_some(())
            .ok_or(RemoteAccessGrantError::InvalidSerializedState);
    }
    if let Some(index) = grant.restart_recovery_at {
        if !matches!(
            grant.state,
            RemoteAccessGrantState::ReconnectPending
                | RemoteAccessGrantState::Active
                | RemoteAccessGrantState::Paused
        ) || index > grant.attempts.len()
            || grant.terminal_milestone.is_some()
        {
            return Err(RemoteAccessGrantError::InvalidSerializedState);
        }
    }
    if grant.restart_recovery_milestone.is_some()
        && (!(grant.state == RemoteAccessGrantState::Active
            || (grant.state == RemoteAccessGrantState::ReconnectPending
                && grant.reconnect_request_recovery_milestone.is_some()))
            || grant.restart_recovery_at != Some(grant.attempts.len())
            || grant.terminal_milestone.is_some())
    {
        return Err(RemoteAccessGrantError::InvalidSerializedState);
    }
    if grant.stop_recovery == RemoteAccessGrantStopRecoveryState::Pending
        && !matches!(
            grant.state,
            RemoteAccessGrantState::Stopped | RemoteAccessGrantState::ReconnectPending
        )
    {
        return Err(RemoteAccessGrantError::InvalidSerializedState);
    }
    Ok(())
}

pub(super) fn is_terminal(state: RemoteAccessGrantState) -> bool {
    matches!(
        state,
        RemoteAccessGrantState::Revoked
            | RemoteAccessGrantState::Removed
            | RemoteAccessGrantState::Denied
            | RemoteAccessGrantState::Failed
            | RemoteAccessGrantState::Superseded
    )
}

const ACCEPTED_RESULTING_STATES: [RemoteAccessGrantState; 12] = [
    RemoteAccessGrantState::ParentConfirmed,
    RemoteAccessGrantState::Paired,
    RemoteAccessGrantState::Active,
    RemoteAccessGrantState::Paused,
    RemoteAccessGrantState::Stopped,
    RemoteAccessGrantState::ReconnectPending,
    RemoteAccessGrantState::Active,
    RemoteAccessGrantState::Revoked,
    RemoteAccessGrantState::Removed,
    RemoteAccessGrantState::Denied,
    RemoteAccessGrantState::Failed,
    RemoteAccessGrantState::Superseded,
];

pub(super) fn accepted_resulting_state(
    transition: super::RemoteAccessGrantTransition,
) -> RemoteAccessGrantState {
    ACCEPTED_RESULTING_STATES[transition as usize]
}
