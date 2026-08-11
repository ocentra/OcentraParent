use super::{
    validation::accepted_resulting_state, RemoteAccessGrant, RemoteAccessGrantAuditMilestone,
    RemoteAccessGrantAuditOutcome, RemoteAccessGrantError,
};

pub(super) fn validate(grant: &RemoteAccessGrant) -> Result<(), RemoteAccessGrantError> {
    if grant.attempts.len() > super::MAX_REPLAY_ATTEMPTS || has_invalid_attempt(grant) {
        return Err(RemoteAccessGrantError::InvalidSerializedState);
    }
    if has_duplicate_attempt_ref(grant) {
        return Err(RemoteAccessGrantError::InvalidSerializedState);
    }
    Ok(())
}

fn has_invalid_attempt(grant: &RemoteAccessGrant) -> bool {
    grant.attempts.iter().any(|attempt| {
        attempt.grant_id != grant.grant_id
            || attempt.audit_ref != grant.audit_ref
            || attempt.attempt_ref.trim().is_empty()
            || attempt.child_device_ref.trim().is_empty()
            || (attempt.outcome == RemoteAccessGrantAuditOutcome::Accepted
                && attempt.household_ref != grant.household_ref)
            || (attempt.outcome == RemoteAccessGrantAuditOutcome::Accepted
                && attempt.child_device_ref != grant.child_device_ref)
            || (attempt.outcome == RemoteAccessGrantAuditOutcome::Accepted
                && attempt.route != grant.route)
            || (attempt.outcome == RemoteAccessGrantAuditOutcome::Accepted
                && accepted_resulting_state(attempt.transition) != attempt.resulting_state)
            || has_invalid_supersession_replacement(grant, attempt)
            || matches!(
                (attempt.outcome, attempt.error.is_some()),
                (RemoteAccessGrantAuditOutcome::Accepted, true)
                    | (RemoteAccessGrantAuditOutcome::Denied, false)
            )
    })
}

fn has_invalid_supersession_replacement(
    grant: &RemoteAccessGrant,
    attempt: &RemoteAccessGrantAuditMilestone,
) -> bool {
    if attempt.transition != super::RemoteAccessGrantTransition::Supersede {
        return attempt.replacement_grant_id.is_some();
    }
    attempt.outcome == RemoteAccessGrantAuditOutcome::Accepted
        && attempt.replacement_grant_id.as_deref() != grant.superseded_by.as_deref()
}

fn has_duplicate_attempt_ref(grant: &RemoteAccessGrant) -> bool {
    grant.attempts.iter().enumerate().any(|(index, attempt)| {
        grant.attempts[..index].iter().any(|prior| {
            prior.attempt_ref == attempt.attempt_ref
                && !is_corrected_child_device_retry(prior, attempt)
        })
    })
}

fn is_corrected_child_device_retry(
    prior: &RemoteAccessGrantAuditMilestone,
    current: &RemoteAccessGrantAuditMilestone,
) -> bool {
    prior.outcome == RemoteAccessGrantAuditOutcome::Denied
        && prior.error == Some(RemoteAccessGrantError::WrongDevice)
        && current.outcome == RemoteAccessGrantAuditOutcome::Accepted
        && prior.transition == current.transition
        && prior.household_ref == current.household_ref
        && prior.actor_ref == current.actor_ref
        && prior.route == current.route
        && prior.child_device_ref != current.child_device_ref
}
