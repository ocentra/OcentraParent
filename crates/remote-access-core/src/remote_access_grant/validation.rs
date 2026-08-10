use ocentra_schema::remote_capability_fabric::RemoteActorRole;

use super::{
    RemoteAccessGrant, RemoteAccessGrantDisclosureState, RemoteAccessGrantError,
    RemoteAccessGrantParentGrant, RemoteAccessGrantState, RemoteAccessGrantStopRecoveryState,
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
    validate_attempts(grant)?;
    super::validation_terminal::validate(grant)?;
    super::validation_history::validate(grant)?;
    validate_supersession(grant)?;
    validate_lifecycle_evidence(grant)?;
    validate_recovery(grant)
}

fn validate_attempts(grant: &RemoteAccessGrant) -> Result<(), RemoteAccessGrantError> {
    if grant.attempts.len() > super::MAX_REPLAY_ATTEMPTS {
        return Err(RemoteAccessGrantError::InvalidSerializedState);
    }
    if grant.attempts.iter().any(|attempt| {
        attempt.grant_id != grant.grant_id
            || attempt.audit_ref != grant.audit_ref
            || attempt.attempt_ref.trim().is_empty()
            || attempt.child_device_ref.trim().is_empty()
            || (attempt.outcome == super::RemoteAccessGrantAuditOutcome::Accepted
                && attempt.household_ref != grant.household_ref)
            || (attempt.outcome == super::RemoteAccessGrantAuditOutcome::Accepted
                && attempt.child_device_ref != grant.child_device_ref)
            || (attempt.outcome == super::RemoteAccessGrantAuditOutcome::Accepted
                && attempt.route != grant.route)
            || (attempt.outcome == super::RemoteAccessGrantAuditOutcome::Accepted
                && accepted_resulting_state(attempt.transition) != attempt.resulting_state)
            || matches!(
                (attempt.outcome, attempt.error.is_some()),
                (super::RemoteAccessGrantAuditOutcome::Accepted, true)
                    | (super::RemoteAccessGrantAuditOutcome::Denied, false)
            )
    }) {
        return Err(RemoteAccessGrantError::InvalidSerializedState);
    }
    Ok(())
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

fn validate_lifecycle_evidence(grant: &RemoteAccessGrant) -> Result<(), RemoteAccessGrantError> {
    if !is_terminal(grant.state) {
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

fn validate_recovery(grant: &RemoteAccessGrant) -> Result<(), RemoteAccessGrantError> {
    if let Some(index) = grant.restart_recovery_at {
        if grant.state != RemoteAccessGrantState::ReconnectPending
            || index > grant.attempts.len()
            || grant.terminal_milestone.is_some()
        {
            return Err(RemoteAccessGrantError::InvalidSerializedState);
        }
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
