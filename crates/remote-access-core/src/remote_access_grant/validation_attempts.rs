use super::{
    validation::accepted_resulting_state, RemoteAccessGrant, RemoteAccessGrantAuditOutcome,
    RemoteAccessGrantError,
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
            || matches!(
                (attempt.outcome, attempt.error.is_some()),
                (RemoteAccessGrantAuditOutcome::Accepted, true)
                    | (RemoteAccessGrantAuditOutcome::Denied, false)
            )
    })
}

fn has_duplicate_attempt_ref(grant: &RemoteAccessGrant) -> bool {
    grant.attempts.iter().enumerate().any(|(index, attempt)| {
        grant.attempts[..index]
            .iter()
            .any(|prior| prior.attempt_ref == attempt.attempt_ref)
    })
}
