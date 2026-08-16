use super::{
    RemoteAccessGrant, RemoteAccessGrantAuditMilestone, RemoteAccessGrantContext,
    RemoteAccessGrantError, RemoteAccessGrantTransition,
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
    context: &RemoteAccessGrantContext<'_>,
    error: RemoteAccessGrantError,
) -> String {
    format!(
        "{}:replay-denied:{}:{}:{}:{}:{}:{}:{}:{}",
        encode_component(grant.audit_ref()),
        encode_component(transition_key(transition)),
        encode_component(grant.grant_id()),
        encode_component(context.household_ref),
        encode_component(context.actor_ref),
        encode_component(context.child_device_ref),
        encode_component(route_key(audit_route(grant, transition, context))),
        encode_component(context.attempt_ref),
        encode_component(error_key(error)),
    )
}

pub(super) fn encode_component(value: &str) -> String {
    format!("{}:{}", value.len(), value)
}

pub(super) fn same_attempt_identity(
    grant: &RemoteAccessGrant,
    previous: &RemoteAccessGrantAuditMilestone,
    transition: RemoteAccessGrantTransition,
    context: &RemoteAccessGrantContext<'_>,
) -> bool {
    previous.grant_id == grant.grant_id()
        && previous.household_ref == context.household_ref
        && previous.actor_ref == context.actor_ref
        && previous.child_device_ref == context.child_device_ref
        && previous.attempt_ref == context.attempt_ref
        && previous.route == audit_route(grant, transition, context)
}

pub(super) fn audit_household(
    grant: &RemoteAccessGrant,
    context: &RemoteAccessGrantContext<'_>,
    error: Option<RemoteAccessGrantError>,
) -> String {
    if error == Some(RemoteAccessGrantError::WrongHousehold) {
        context.household_ref.to_owned()
    } else {
        grant.household_ref.clone()
    }
}

pub(super) fn transition_key(transition: RemoteAccessGrantTransition) -> &'static str {
    const KEYS: [&str; 12] = [
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
        "supersede",
    ];
    KEYS[transition as usize]
}

const ROUTE_KEYS: [&str; 3] = ["localhost", "local-network", "cloud-relay"];

fn route_key(route: RemoteRoute) -> &'static str {
    ROUTE_KEYS[route as usize]
}

const ERROR_KEYS: [&str; 15] = [
    "empty-field",
    "wrong-household",
    "wrong-actor",
    "wrong-device",
    "wrong-route",
    "device-trust-required",
    "parent-authority-required",
    "child-disclosure-required",
    "support-access-requires-parent-grant",
    "invalid-transition",
    "invalid-serialized-state",
    "reconnect-denied",
    "superseding-grant-required",
    "superseding-grant-mismatch",
    "replay-window-exhausted",
];

fn error_key(error: RemoteAccessGrantError) -> &'static str {
    ERROR_KEYS[error as usize]
}
