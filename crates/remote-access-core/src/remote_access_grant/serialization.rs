use serde::{de::Error as _, Deserialize, Deserializer};

use ocentra_schema::remote_capability_fabric::RemoteActorRole;
use ocentra_schema::remote_capability_fabric::RemoteRoute;

use super::{
    validation, RemoteAccessGrant, RemoteAccessGrantCapability, RemoteAccessGrantDisclosureState,
    RemoteAccessGrantParentGrant, RemoteAccessGrantState, RemoteAccessGrantStopRecoveryState,
};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct RemoteAccessGrantSnapshot {
    grant_id: String,
    household_ref: String,
    child_device_ref: String,
    route: RemoteRoute,
    parent_actor_ref: String,
    capability: RemoteAccessGrantCapability,
    actor_role: RemoteActorRole,
    state: RemoteAccessGrantState,
    disclosure_state: RemoteAccessGrantDisclosureState,
    #[serde(default = "default_parent_grant")]
    parent_grant: RemoteAccessGrantParentGrant,
    audit_ref: String,
    #[serde(default)]
    attempts: Option<Vec<super::RemoteAccessGrantAuditMilestone>>,
    #[serde(default)]
    support_actor_ref: Option<String>,
    #[serde(default)]
    terminal_milestone: Option<super::RemoteAccessGrantAuditMilestone>,
    #[serde(default)]
    stop_recovery_milestone: Option<super::RemoteAccessGrantAuditMilestone>,
    #[serde(default)]
    reconnect_request_recovery_milestone: Option<super::RemoteAccessGrantAuditMilestone>,
    #[serde(default)]
    restart_recovery_milestone: Option<super::RemoteAccessGrantAuditMilestone>,
    #[serde(default)]
    superseded_by: Option<String>,
    #[serde(default)]
    stop_recovery: RemoteAccessGrantStopRecoveryState,
    #[serde(default)]
    restart_recovery_at: Option<usize>,
    #[serde(default)]
    restart_recovery_history: Vec<usize>,
}

fn default_parent_grant() -> RemoteAccessGrantParentGrant {
    RemoteAccessGrantParentGrant::NotGranted
}

fn record_restart_recovery_boundary(history: &mut Vec<usize>, boundary: Option<usize>) {
    let Some(boundary) = boundary else {
        return;
    };
    if !history.contains(&boundary) {
        history.push(boundary);
    }
}

impl<'de> Deserialize<'de> for RemoteAccessGrant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let snapshot = RemoteAccessGrantSnapshot::deserialize(deserializer)?;
        // A persisted live grant cannot prove that the current parent authority
        // and device trust checks still hold after a restart.  Resume it at the
        // reconnect gate so a fresh, typed context must authorize live access.
        let persisted_state = snapshot.state;
        let attempts_present = snapshot.attempts.is_some();
        let has_reconnect_request_recovery =
            snapshot.reconnect_request_recovery_milestone.is_some();
        if snapshot.state != RemoteAccessGrantState::Requested
            && (!attempts_present || snapshot.attempts.as_ref().is_some_and(Vec::is_empty))
        {
            return Err(D::Error::custom(
                "persisted non-requested grant must retain transition history",
            ));
        }
        let mut attempts = snapshot.attempts.unwrap_or_default();
        for attempt in &mut attempts {
            if attempt.child_device_ref.trim().is_empty() {
                attempt.child_device_ref = snapshot.child_device_ref.clone();
            }
        }
        let mut restart_recovery_history = snapshot.restart_recovery_history;
        let (state, restart_recovery_at) = if persisted_state == RemoteAccessGrantState::Active {
            record_restart_recovery_boundary(
                &mut restart_recovery_history,
                snapshot.restart_recovery_at,
            );
            (
                RemoteAccessGrantState::ReconnectPending,
                Some(attempts.len()),
            )
        } else {
            (persisted_state, snapshot.restart_recovery_at)
        };
        let grant = RemoteAccessGrant {
            grant_id: snapshot.grant_id,
            household_ref: snapshot.household_ref,
            child_device_ref: snapshot.child_device_ref,
            route: snapshot.route,
            parent_actor_ref: snapshot.parent_actor_ref,
            support_actor_ref: snapshot.support_actor_ref,
            capability: snapshot.capability,
            actor_role: snapshot.actor_role,
            state,
            disclosure_state: snapshot.disclosure_state,
            parent_grant: snapshot.parent_grant,
            audit_ref: snapshot.audit_ref,
            attempts,
            terminal_milestone: snapshot.terminal_milestone,
            stop_recovery_milestone: snapshot.stop_recovery_milestone,
            reconnect_request_recovery_milestone: snapshot.reconnect_request_recovery_milestone,
            restart_recovery_milestone: (persisted_state != RemoteAccessGrantState::Active
                || has_reconnect_request_recovery)
                .then_some(snapshot.restart_recovery_milestone)
                .flatten(),
            superseded_by: snapshot.superseded_by,
            stop_recovery: snapshot.stop_recovery,
            restart_recovery_at,
            restart_recovery_history,
            pending_supersession: None,
        };
        validation::serialized(&grant).map_err(D::Error::custom)?;
        Ok(grant)
    }
}
