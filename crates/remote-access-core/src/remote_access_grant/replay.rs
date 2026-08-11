use super::replay_identity::audit_route;
use super::{
    RemoteAccessGrant, RemoteAccessGrantAuditMilestone, RemoteAccessGrantAuditOutcome,
    RemoteAccessGrantContext, RemoteAccessGrantState, RemoteAccessGrantTransition,
    RemoteAccessGrantTransitionReport,
};

pub(super) fn previous_attempt(
    grant: &RemoteAccessGrant,
    attempt_ref: &str,
) -> Option<RemoteAccessGrantAuditMilestone> {
    grant
        .stop_recovery_milestone
        .as_ref()
        .filter(|attempt| attempt.attempt_ref == attempt_ref)
        .cloned()
        .or_else(|| {
            grant
                .restart_recovery_milestone
                .as_ref()
                .filter(|attempt| attempt.attempt_ref == attempt_ref)
                .cloned()
        })
        .or_else(|| {
            grant
                .terminal_milestone
                .as_ref()
                .filter(|attempt| attempt.attempt_ref == attempt_ref)
                .cloned()
                .or_else(|| {
                    grant
                        .attempts
                        .iter()
                        .rev()
                        .find(|attempt| attempt.attempt_ref == attempt_ref)
                        .cloned()
                })
        })
}

pub(super) fn is_child_device_retry(
    grant: &RemoteAccessGrant,
    previous: &RemoteAccessGrantAuditMilestone,
    transition: RemoteAccessGrantTransition,
    context: &RemoteAccessGrantContext<'_>,
) -> bool {
    previous.outcome == RemoteAccessGrantAuditOutcome::Denied
        && previous.error == Some(super::RemoteAccessGrantError::WrongDevice)
        && previous.transition == transition
        && previous.household_ref == context.household_ref
        && previous.actor_ref == context.actor_ref
        && previous.route == context.route
        && previous.child_device_ref != context.child_device_ref
        && context.child_device_ref == grant.child_device_ref
}

pub(super) fn matches_supersession_replacement(
    grant: &RemoteAccessGrant,
    previous: &RemoteAccessGrantAuditMilestone,
    replacement_grant_id: &str,
) -> bool {
    previous.transition != RemoteAccessGrantTransition::Supersede
        || previous.outcome != RemoteAccessGrantAuditOutcome::Accepted
        || previous
            .replacement_grant_id
            .as_deref()
            .or(grant.superseded_by.as_deref())
            .is_none_or(|recorded| recorded == replacement_grant_id)
}

pub(super) fn replacement_scope_matches(
    grant: &RemoteAccessGrant,
    replacement: &RemoteAccessGrant,
) -> bool {
    grant.grant_id != replacement.grant_id
        && grant.household_ref == replacement.household_ref
        && grant.child_device_ref == replacement.child_device_ref
        && grant.route == replacement.route
        && grant.capability == replacement.capability
}

pub(super) fn transition_report(
    grant: &RemoteAccessGrant,
    transition: RemoteAccessGrantTransition,
    context: RemoteAccessGrantContext<'_>,
    result: Result<RemoteAccessGrantState, super::RemoteAccessGrantError>,
) -> RemoteAccessGrantTransitionReport {
    let (outcome, error, resulting_state) = match result {
        Ok(state) => (RemoteAccessGrantAuditOutcome::Accepted, None, state),
        Err(error) => (
            RemoteAccessGrantAuditOutcome::Denied,
            Some(error),
            grant.state,
        ),
    };
    RemoteAccessGrantTransitionReport {
        result: match error {
            Some(error) => Err(error),
            None => Ok(resulting_state),
        },
        audit: RemoteAccessGrantAuditMilestone {
            grant_id: grant.grant_id.clone(),
            household_ref: super::replay_identity::audit_household(grant, &context, error),
            actor_ref: context.actor_ref.to_owned(),
            child_device_ref: context.child_device_ref.to_owned(),
            route: audit_route(grant, transition, &context),
            attempt_ref: context.attempt_ref.to_owned(),
            transition,
            outcome,
            resulting_state,
            error,
            replacement_grant_id: if transition == RemoteAccessGrantTransition::Supersede {
                grant.superseded_by.clone()
            } else {
                None
            },
            audit_ref: grant.audit_ref.clone(),
        },
    }
}
