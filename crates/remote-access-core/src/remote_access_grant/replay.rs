use super::replay_identity::{audit_route, replay_denial_audit_ref, same_attempt_identity};
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
) -> Option<RemoteAccessGrantError> {
    live_access_start_state(transition)?;
    matches!(
        grant.state,
        RemoteAccessGrantState::Revoked
            | RemoteAccessGrantState::Removed
            | RemoteAccessGrantState::Denied
            | RemoteAccessGrantState::Failed
    )
    .then_some(RemoteAccessGrantError::InvalidTransition)
}

pub(super) fn existing_report(
    grant: &RemoteAccessGrant,
    previous: RemoteAccessGrantAuditMilestone,
    transition: RemoteAccessGrantTransition,
    context: RemoteAccessGrantContext<'_>,
) -> RemoteAccessGrantTransitionReport {
    if previous.transition != transition
        || !same_attempt_identity(grant, &previous, transition, &context)
    {
        return denied_report(
            grant,
            transition,
            context,
            RemoteAccessGrantError::InvalidTransition,
        );
    }
    if previous.outcome == RemoteAccessGrantAuditOutcome::Accepted {
        if let Err(error) = super::validation::context(grant, transition, &context) {
            return denied_report(grant, transition, context, error);
        }
        if let Some(error) = access_start_replay_error(grant, transition) {
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
    let route = audit_route(grant, transition, &context);
    RemoteAccessGrantTransitionReport {
        result: Err(error),
        audit: RemoteAccessGrantAuditMilestone {
            grant_id: grant.grant_id.clone(),
            household_ref: grant.household_ref.clone(),
            actor_ref: context.actor_ref.to_owned(),
            route,
            attempt_ref: context.attempt_ref.to_owned(),
            transition,
            outcome: RemoteAccessGrantAuditOutcome::Denied,
            resulting_state: grant.state,
            error: Some(error),
            audit_ref: replay_denial_audit_ref(grant, transition),
        },
    }
}
