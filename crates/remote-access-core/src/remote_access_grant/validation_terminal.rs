use super::{
    validation, RemoteAccessGrant, RemoteAccessGrantAuditOutcome, RemoteAccessGrantError,
    RemoteAccessGrantTransition,
};

pub(super) fn validate(grant: &RemoteAccessGrant) -> Result<(), RemoteAccessGrantError> {
    let Some(attempt) = grant.terminal_milestone.as_ref() else {
        return Ok(());
    };
    if !has_terminal_capacity(grant) || has_denied_attempt(grant) {
        return invalid();
    }
    if !is_reserved_transition(attempt.transition) || !is_accepted(attempt) {
        return invalid();
    }
    if !matches_grant_state(grant, attempt) || !matches_identity(grant, attempt) {
        return invalid();
    }
    Ok(())
}

fn has_terminal_capacity(grant: &RemoteAccessGrant) -> bool {
    grant.attempts.len() == super::MAX_REPLAY_ATTEMPTS
}

fn has_denied_attempt(grant: &RemoteAccessGrant) -> bool {
    grant
        .attempts
        .iter()
        .any(|entry| entry.outcome == RemoteAccessGrantAuditOutcome::Denied)
}

fn is_reserved_transition(transition: RemoteAccessGrantTransition) -> bool {
    matches!(
        transition,
        RemoteAccessGrantTransition::Revoke
            | RemoteAccessGrantTransition::RemoveDevice
            | RemoteAccessGrantTransition::Deny
            | RemoteAccessGrantTransition::Fail
            | RemoteAccessGrantTransition::Supersede
    )
}

fn is_accepted(attempt: &super::RemoteAccessGrantAuditMilestone) -> bool {
    attempt.outcome == RemoteAccessGrantAuditOutcome::Accepted && attempt.error.is_none()
}

fn matches_grant_state(
    grant: &RemoteAccessGrant,
    attempt: &super::RemoteAccessGrantAuditMilestone,
) -> bool {
    validation::is_terminal(grant.state)
        && attempt.resulting_state == grant.state
        && validation::accepted_resulting_state(attempt.transition) == attempt.resulting_state
}

fn matches_identity(
    grant: &RemoteAccessGrant,
    attempt: &super::RemoteAccessGrantAuditMilestone,
) -> bool {
    !attempt.actor_ref.trim().is_empty()
        && !attempt.attempt_ref.trim().is_empty()
        && attempt.grant_id == grant.grant_id
        && attempt.audit_ref == grant.audit_ref
        && attempt.household_ref == grant.household_ref
        && attempt.child_device_ref == grant.child_device_ref
        && attempt.route == grant.route
        && (matches!(
            attempt.transition,
            RemoteAccessGrantTransition::Supersede
                if attempt.replacement_grant_id.as_deref() == grant.superseded_by.as_deref()
        ) || (attempt.transition != RemoteAccessGrantTransition::Supersede
            && attempt.replacement_grant_id.is_none()))
}

fn invalid() -> Result<(), RemoteAccessGrantError> {
    Err(RemoteAccessGrantError::InvalidSerializedState)
}
