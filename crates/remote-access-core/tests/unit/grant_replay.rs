use super::grant::{context_for, paired_grant};
use ocentra_eventing::envelope::DomainEvent;
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
fn replay_history_rejects_new_attempts_when_full_and_preserves_old_identities() -> Result<(), String>
{
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

    for index in 0..64 {
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

    let first_attempt = "attempt-replay-window-0";
    let mut first_retry = context_for(first_attempt);
    first_retry.actor_ref = "parent-other";
    let retry = grant.transition(RemoteAccessGrantTransition::ConfirmParent, first_retry);
    assert_eq!(retry.result, Err(RemoteAccessGrantError::WrongActor));

    let new_attempt = context_for("attempt-replay-window-64");
    let exhausted = grant.transition(RemoteAccessGrantTransition::ConfirmParent, new_attempt);
    assert_eq!(
        exhausted.result,
        Err(RemoteAccessGrantError::ReplayWindowExhausted)
    );
    assert_eq!(grant.state(), RemoteAccessGrantState::Requested);
    assert_eq!(
        grant
            .transition_with_audit(RemoteAccessGrantTransition::ConfirmParent, new_attempt)
            .result,
        Err(RemoteAccessGrantError::ReplayWindowExhausted)
    );

    let safety = grant.transition(
        RemoteAccessGrantTransition::Revoke,
        context_for("attempt-replay-window-revoke"),
    );
    assert_eq!(safety.result, Ok(RemoteAccessGrantState::Revoked));
    assert_eq!(grant.state(), RemoteAccessGrantState::Revoked);

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
fn accepted_milestone_result_state_must_match_its_transition() {
    let mut grant = paired_grant();
    grant
        .transition(
            RemoteAccessGrantTransition::Activate,
            context_for("attempt-accepted-state-activate"),
        )
        .result
        .expect_value("activate grant");
    grant
        .transition(
            RemoteAccessGrantTransition::Pause,
            context_for("attempt-accepted-state-pause"),
        )
        .result
        .expect_value("pause grant");

    let mut encoded = serde_json::to_value(&grant).expect_value("serialize grant");
    encoded["attempts"]
        .as_array_mut()
        .and_then(|attempts| attempts.last_mut())
        .expect_value("pause milestone")["resultingState"] = serde_json::json!("active");
    let error = serde_json::from_value::<RemoteAccessGrant>(encoded)
        .err()
        .expect_value("accepted state mismatch must be rejected")
        .to_string();
    assert_eq!(
        error.split(" at ").next(),
        Some("serialized grant state violates lifecycle invariants")
    );
}

#[test]
fn serialized_accepted_milestones_must_follow_reachable_history() {
    let grant = paired_grant();
    let mut encoded = serde_json::to_value(&grant).expect_value("serialize grant");
    encoded["attempts"]
        .as_array_mut()
        .and_then(|attempts| attempts.first_mut())
        .expect_value("confirm milestone")["transition"] = serde_json::json!("activate");
    encoded["attempts"]
        .as_array_mut()
        .and_then(|attempts| attempts.first_mut())
        .expect_value("activate milestone")["resultingState"] = serde_json::json!("active");
    encoded["state"] = serde_json::json!("active");

    let error = serde_json::from_value::<RemoteAccessGrant>(encoded)
        .err()
        .expect_value("unreachable accepted history must be rejected")
        .to_string();
    assert_eq!(
        error.split(" at ").next(),
        Some("serialized grant state violates lifecycle invariants")
    );
}

#[test]
fn replay_identity_binds_the_child_device_context() {
    let mut grant = paired_grant();
    let mut wrong_device = context_for("attempt-child-identity");
    wrong_device.child_device_ref = "child-other";
    assert_eq!(
        grant
            .transition(RemoteAccessGrantTransition::Activate, wrong_device)
            .result,
        Err(RemoteAccessGrantError::WrongDevice)
    );

    let retry = grant.transition(
        RemoteAccessGrantTransition::Activate,
        context_for("attempt-child-identity"),
    );
    assert_eq!(retry.result, Ok(RemoteAccessGrantState::Active));
}

#[test]
fn activation_replay_is_rejected_while_system_recovery_is_pending() {
    let mut grant = paired_grant();
    grant
        .transition_with_audit(
            RemoteAccessGrantTransition::Activate,
            context_for("attempt-recovery-activation"),
        )
        .result
        .expect_value("activate grant");

    let mut system_stop = context_for("attempt-recovery-system-stop");
    system_stop.actor_ref = "system-failure";
    system_stop.parent_authorized = false;
    system_stop.transition_authority =
        ocentra_remote_access_core::remote_access_grant::RemoteAccessGrantTransitionAuthority::SystemFailure;
    grant
        .transition(RemoteAccessGrantTransition::Stop, system_stop)
        .result
        .expect_value("system stop");

    assert_eq!(
        grant
            .transition(
                RemoteAccessGrantTransition::Activate,
                context_for("attempt-recovery-activation"),
            )
            .result,
        Err(RemoteAccessGrantError::ReconnectDenied)
    );
}

#[test]
fn replay_denial_identity_includes_the_invalid_context() {
    let mut grant = paired_grant();
    let denied = grant.transition_with_audit(
        RemoteAccessGrantTransition::Reconnect,
        context_for("attempt-context-denied"),
    );
    assert_eq!(denied.result, Err(RemoteAccessGrantError::ReconnectDenied));

    let mut different_route = context_for("attempt-context-denied");
    different_route.route = RemoteRoute::CloudRelay;
    let different =
        grant.transition_with_audit(RemoteAccessGrantTransition::Reconnect, different_route);
    assert_eq!(
        different.result,
        Err(RemoteAccessGrantError::InvalidTransition)
    );
    assert_ne!(
        denied.audit.idempotency_key(),
        different.audit.idempotency_key()
    );
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
