use super::grant::{context_for, paired_grant};
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_remote_access_core::remote_access_grant::{
    RemoteAccessGrant, RemoteAccessGrantAuditOutcome, RemoteAccessGrantError,
    RemoteAccessGrantState, RemoteAccessGrantTransition,
};
use ocentra_schema::remote_capability_fabric::{RemoteActorRole, RemoteRoute};

#[test]
fn accepted_access_start_replay_rechecks_current_parent_authority() {
    let mut grant = paired_grant();
    let first = grant.transition_with_audit(
        RemoteAccessGrantTransition::Activate,
        context_for("attempt-replay-activate-authority"),
    );
    first.result.expect_value("activate grant");

    let mut restored: RemoteAccessGrant = serde_json::from_value(
        serde_json::to_value(&grant).expect_value("serialize active replay history"),
    )
    .expect_value("deserialize active replay history");
    let mut revoked_authority = context_for("attempt-replay-activate-authority");
    revoked_authority.parent_authorized = false;
    let retry = restored.transition(RemoteAccessGrantTransition::Activate, revoked_authority);

    assert_eq!(
        retry.result,
        Err(RemoteAccessGrantError::ParentAuthorityRequired)
    );
    assert_eq!(retry.audit.outcome, RemoteAccessGrantAuditOutcome::Denied);
    assert_eq!(restored.state(), RemoteAccessGrantState::ReconnectPending);
}

#[test]
fn denied_wrong_route_attempt_survives_serialization() {
    let mut grant = paired_grant();
    let mut wrong_route = context_for("attempt-wrong-route-round-trip");
    wrong_route.route = RemoteRoute::CloudRelay;
    let denied = grant.transition(RemoteAccessGrantTransition::Activate, wrong_route);
    assert_eq!(denied.result, Err(RemoteAccessGrantError::WrongRoute));

    let mut restored: RemoteAccessGrant = serde_json::from_value(
        serde_json::to_value(&grant).expect_value("serialize wrong-route denial"),
    )
    .expect_value("deserialize wrong-route denial");
    assert_eq!(restored.state(), RemoteAccessGrantState::Paired);
    let retry = restored.transition(RemoteAccessGrantTransition::Activate, wrong_route);
    assert_eq!(retry.result, Err(RemoteAccessGrantError::WrongRoute));
}

#[test]
fn replay_history_is_bounded_and_oversized_snapshots_are_rejected() -> Result<(), String> {
    let mut grant = RemoteAccessGrant::request(
        "grant-replay-window",
        "household-alpha",
        "child-alpha",
        RemoteRoute::LocalNetwork,
        "parent-alpha",
        RemoteActorRole::ParentOwner,
        "audit-replay-window",
    )
    .expect_value("grant request");

    for index in 0..65 {
        let attempt_ref = Box::leak(format!("attempt-replay-window-{index}").into_boxed_str());
        let mut wrong_actor = context_for(attempt_ref);
        wrong_actor.actor_ref = "parent-other";
        let report = grant.transition(RemoteAccessGrantTransition::ConfirmParent, wrong_actor);
        assert_eq!(report.result, Err(RemoteAccessGrantError::WrongActor));
    }

    let encoded = serde_json::to_value(&grant).map_err(|error| error.to_string())?;
    let attempts = encoded["attempts"]
        .as_array()
        .ok_or("bounded replay history must serialize as an array")?;
    assert_eq!(attempts.len(), 64);

    let mut oversized = encoded;
    let first_attempt = oversized["attempts"]
        .as_array()
        .and_then(|attempts| attempts.first())
        .cloned()
        .ok_or("bounded history must retain an entry")?;
    oversized["attempts"]
        .as_array_mut()
        .ok_or("bounded replay history must serialize as an array")?
        .push(first_attempt);
    let error_message = serde_json::from_value::<RemoteAccessGrant>(oversized)
        .map(|_| String::from("oversized replay history accepted"))
        .unwrap_or_else(|error| error.to_string());
    assert_eq!(
        error_message,
        "serialized grant state violates lifecycle invariants"
    );
    Ok(())
}

#[test]
fn malformed_denied_milestone_cannot_manufacture_an_active_result() {
    let grant = super::grant::paired_grant();
    let mut encoded = serde_json::to_value(&grant).expect_value("serialize grant");
    let attempt = encoded["attempts"]
        .as_array_mut()
        .and_then(|attempts| attempts.first_mut())
        .expect_value("grant must retain an audit attempt");
    attempt["outcome"] = serde_json::json!("denied");
    attempt["error"] = serde_json::Value::Null;
    attempt["resultingState"] = serde_json::json!("active");

    let error = serde_json::from_value::<
        ocentra_remote_access_core::remote_access_grant::RemoteAccessGrant,
    >(encoded)
    .err()
    .expect_value("malformed denied milestone must be rejected")
    .to_string();
    assert_eq!(
        error.split(" at ").next(),
        Some("serialized grant state violates lifecycle invariants")
    );
}
