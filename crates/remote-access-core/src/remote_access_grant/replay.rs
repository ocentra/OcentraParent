use super::{
    RemoteAccessGrant, RemoteAccessGrantAuditMilestone, RemoteAccessGrantAuditOutcome,
    RemoteAccessGrantContext, RemoteAccessGrantError, RemoteAccessGrantState,
    RemoteAccessGrantTransition, RemoteAccessGrantTransitionReport,
};

pub(super) fn report(audit: RemoteAccessGrantAuditMilestone) -> RemoteAccessGrantTransitionReport {
    let result = match audit.error {
        Some(error) => Err(error),
        None => Ok(audit.resulting_state),
    };
    RemoteAccessGrantTransitionReport { result, audit }
}

pub(super) fn live_access_start_state(
    transition: RemoteAccessGrantTransition,
) -> Option<RemoteAccessGrantState> {
    match transition {
        RemoteAccessGrantTransition::Pair => Some(RemoteAccessGrantState::Paired),
        RemoteAccessGrantTransition::Activate | RemoteAccessGrantTransition::Reconnect => {
            Some(RemoteAccessGrantState::Active)
        }
        _ => None,
    }
}

pub(super) fn access_start_replay_error(
    grant: &RemoteAccessGrant,
    transition: RemoteAccessGrantTransition,
    context: &RemoteAccessGrantContext<'_>,
) -> Option<RemoteAccessGrantError> {
    let expected_state = live_access_start_state(transition)?;
    if let Err(error) = super::validation::context(grant, transition, context) {
        return Some(error);
    }
    (grant.state != expected_state).then_some(RemoteAccessGrantError::InvalidTransition)
}

pub(super) fn existing_report(
    grant: &RemoteAccessGrant,
    previous: RemoteAccessGrantAuditMilestone,
    transition: RemoteAccessGrantTransition,
    context: RemoteAccessGrantContext<'_>,
) -> RemoteAccessGrantTransitionReport {
    if previous.outcome == RemoteAccessGrantAuditOutcome::Accepted {
        if let Some(error) = access_start_replay_error(grant, transition, &context) {
            return denied_report(grant, transition, context, error);
        }
    }
    report(previous)
}

pub(super) fn denied_report(
    grant: &RemoteAccessGrant,
    transition: RemoteAccessGrantTransition,
    context: RemoteAccessGrantContext<'_>,
    error: RemoteAccessGrantError,
) -> RemoteAccessGrantTransitionReport {
    RemoteAccessGrantTransitionReport {
        result: Err(error),
        audit: RemoteAccessGrantAuditMilestone {
            grant_id: grant.grant_id.clone(),
            household_ref: grant.household_ref.clone(),
            actor_ref: context.actor_ref.to_owned(),
            route: context.route,
            attempt_ref: context.attempt_ref.to_owned(),
            transition,
            outcome: RemoteAccessGrantAuditOutcome::Denied,
            resulting_state: grant.state,
            error: Some(error),
            audit_ref: grant.audit_ref.clone(),
        },
    }
}
