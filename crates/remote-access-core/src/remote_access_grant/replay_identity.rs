use super::{
    RemoteAccessGrant, RemoteAccessGrantAuditMilestone, RemoteAccessGrantContext,
    RemoteAccessGrantTransition,
};
use ocentra_schema::remote_capability_fabric::RemoteRoute;

pub(super) fn audit_route(
    grant: &RemoteAccessGrant,
    transition: RemoteAccessGrantTransition,
    context: &RemoteAccessGrantContext<'_>,
) -> RemoteRoute {
    if transition == RemoteAccessGrantTransition::RemoveDevice {
        grant.route()
    } else {
        context.route
    }
}

pub(super) fn replay_denial_audit_ref(
    grant: &RemoteAccessGrant,
    transition: RemoteAccessGrantTransition,
) -> String {
    format!(
        "{}:replay-denied:{}",
        grant.audit_ref(),
        transition_key(transition)
    )
}

pub(super) fn same_attempt_identity(
    grant: &RemoteAccessGrant,
    previous: &RemoteAccessGrantAuditMilestone,
    transition: RemoteAccessGrantTransition,
    context: &RemoteAccessGrantContext<'_>,
) -> bool {
    previous.grant_id == grant.grant_id()
        && previous.household_ref == grant.household_ref()
        && previous.actor_ref == context.actor_ref
        && previous.attempt_ref == context.attempt_ref
        && previous.route == audit_route(grant, transition, context)
}

fn transition_key(transition: RemoteAccessGrantTransition) -> &'static str {
    const KEYS: [&str; 11] = [
        "confirm-parent",
        "pair",
        "activate",
        "pause",
        "stop",
        "request-reconnect",
        "reconnect",
        "revoke",
        "remove-device",
        "deny",
        "fail",
    ];
    KEYS[transition as usize]
}
