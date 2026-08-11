use super::grant::{context_for, paired_grant};
use ocentra_eventing::envelope::DomainEvent;
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_remote_access_core::remote_access_grant::{
    RemoteAccessGrant, RemoteAccessGrantAuditOutcome, RemoteAccessGrantError,
    RemoteAccessGrantState, RemoteAccessGrantTransition, RemoteAccessGrantTransitionAuthority,
};
use ocentra_schema::remote_capability_fabric::{RemoteActorRole, RemoteRoute};

#[test]
fn terminal_transition_clears_pending_system_recovery() {
    let mut grant = paired_grant();
    grant
        .transition(
            RemoteAccessGrantTransition::Activate,
            context_for("attempt-terminal-recovery-activate"),
        )
        .result
        .expect_value("activate grant");
    let mut system_stop = context_for("attempt-terminal-recovery-stop");
    system_stop.actor_ref = "system-failure";
    system_stop.parent_authorized = false;
    system_stop.transition_authority = RemoteAccessGrantTransitionAuthority::SystemFailure;
    grant
        .transition(RemoteAccessGrantTransition::Stop, system_stop)
        .result
        .expect_value("system stop");
    let pending = serde_json::to_value(&grant).expect_value("serialize pending recovery");
    assert_eq!(pending["stop_recovery"], serde_json::json!("pending"));

    grant
        .transition(
            RemoteAccessGrantTransition::Revoke,
            context_for("attempt-terminal-recovery-revoke"),
        )
        .result
        .expect_value("revoke pending recovery");
    let terminal = serde_json::to_value(&grant).expect_value("serialize terminal recovery");
    assert_eq!(terminal["stop_recovery"], serde_json::json!("not-required"));
    let restored: RemoteAccessGrant =
        serde_json::from_value(terminal).expect_value("restore terminal recovery");
    assert_eq!(restored, grant);
}

#[test]
fn terminal_invalidation_retains_completed_restart_recovery_history() {
    let mut grant = paired_grant();
    grant
        .transition(
            RemoteAccessGrantTransition::Activate,
            context_for("attempt-terminal-restart-history-activate"),
        )
        .result
        .expect_value("activate grant");
    let mut restored: RemoteAccessGrant =
        serde_json::from_value(serde_json::to_value(&grant).expect_value("serialize active grant"))
            .expect_value("restore active grant at reconnect boundary");
    restored
        .transition(
            RemoteAccessGrantTransition::Reconnect,
            context_for("attempt-terminal-restart-history-reconnect"),
        )
        .result
        .expect_value("complete restarted reconnect");
    restored
        .transition(
            RemoteAccessGrantTransition::Revoke,
            context_for("attempt-terminal-restart-history-revoke"),
        )
        .result
        .expect_value("revoke restored grant");
    let terminal = serde_json::to_value(&restored).expect_value("serialize terminal grant");
    let round_tripped: RemoteAccessGrant =
        serde_json::from_value(terminal).expect_value("restore terminal grant");
    assert_eq!(round_tripped, restored);
}

#[test]
fn malformed_replay_identity_is_not_retained_as_a_denial() {
    let mut grant = paired_grant();
    let encoded_before = serde_json::to_value(&grant).expect_value("serialize initial grant");
    let mut missing_attempt_ref = context_for("attempt-will-be-cleared");
    missing_attempt_ref.attempt_ref = " ";
    assert_eq!(
        grant
            .transition(RemoteAccessGrantTransition::Activate, missing_attempt_ref)
            .result,
        Err(RemoteAccessGrantError::EmptyField)
    );
    let mut missing_device_ref = context_for("attempt-missing-device-ref");
    missing_device_ref.child_device_ref = " ";
    assert_eq!(
        grant
            .transition(RemoteAccessGrantTransition::Activate, missing_device_ref)
            .result,
        Err(RemoteAccessGrantError::EmptyField)
    );
    assert_eq!(
        serde_json::to_value(&grant).expect_value("serialize after invalid identities"),
        encoded_before
    );
}

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
fn supersede_without_replacement_preserves_pending_stop_recovery() {
    let mut grant = paired_grant();
    grant
        .transition(
            RemoteAccessGrantTransition::Activate,
            context_for("attempt-supersede-recovery-activate"),
        )
        .result
        .expect_value("activate grant");
    let mut system_stop = context_for("attempt-supersede-recovery-stop");
    system_stop.actor_ref = "system-failure";
    system_stop.parent_authorized = false;
    system_stop.transition_authority = RemoteAccessGrantTransitionAuthority::SystemFailure;
    grant
        .transition(RemoteAccessGrantTransition::Stop, system_stop)
        .result
        .expect_value("system stop");

    let report = grant.transition(
        RemoteAccessGrantTransition::Supersede,
        context_for("attempt-supersede-recovery-missing"),
    );
    assert_eq!(
        report.result,
        Err(RemoteAccessGrantError::SupersedingGrantRequired)
    );
    assert_eq!(grant.state(), RemoteAccessGrantState::Stopped);
    let encoded = serde_json::to_value(&grant).expect_value("serialize pending recovery");
    assert_eq!(encoded["stop_recovery"], serde_json::json!("pending"));
}

#[test]
fn restart_recovery_marker_survives_a_denial_and_another_round_trip() {
    let mut grant = paired_grant();
    grant
        .transition(
            RemoteAccessGrantTransition::Activate,
            context_for("attempt-restart-recovery-activate"),
        )
        .result
        .expect_value("activate grant");

    let mut restored: RemoteAccessGrant =
        serde_json::from_value(serde_json::to_value(&grant).expect_value("serialize active grant"))
            .expect_value("restore active grant");
    assert_eq!(restored.state(), RemoteAccessGrantState::ReconnectPending);

    let mut unauthorized = context_for("attempt-restart-recovery-denied");
    unauthorized.parent_authorized = false;
    let denied = restored.transition(RemoteAccessGrantTransition::Reconnect, unauthorized);
    assert_eq!(
        denied.result,
        Err(RemoteAccessGrantError::ParentAuthorityRequired)
    );

    let round_tripped: RemoteAccessGrant = serde_json::from_value(
        serde_json::to_value(&restored).expect_value("serialize recovery denial"),
    )
    .expect_value("restore recovery denial");
    assert_eq!(
        round_tripped.state(),
        RemoteAccessGrantState::ReconnectPending
    );
    assert_eq!(round_tripped, restored);
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
fn wrong_device_retry_cannot_change_to_a_second_wrong_device() {
    let mut grant = paired_grant();
    let mut first_wrong_device = context_for("attempt-wrong-device-retry");
    first_wrong_device.child_device_ref = "child-other-a";
    assert_eq!(
        grant
            .transition(RemoteAccessGrantTransition::Activate, first_wrong_device)
            .result,
        Err(RemoteAccessGrantError::WrongDevice)
    );

    let mut second_wrong_device = context_for("attempt-wrong-device-retry");
    second_wrong_device.child_device_ref = "child-other-b";
    assert_eq!(
        grant
            .transition(RemoteAccessGrantTransition::Activate, second_wrong_device)
            .result,
        Err(RemoteAccessGrantError::InvalidTransition)
    );

    let mut restored: RemoteAccessGrant = serde_json::from_value(
        serde_json::to_value(&grant).expect_value("serialize wrong-device retry"),
    )
    .expect_value("restore only the original wrong-device denial");
    assert_eq!(restored.state(), RemoteAccessGrantState::Paired);
    assert_eq!(
        restored
            .transition(RemoteAccessGrantTransition::Activate, first_wrong_device)
            .result,
        Err(RemoteAccessGrantError::WrongDevice)
    );
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
fn terminal_invalidation_is_available_after_accepted_history_saturation() {
    let mut grant = paired_grant();
    for index in 0..31 {
        let activate_ref =
            Box::leak(format!("attempt-saturated-activate-{index}").into_boxed_str());
        grant
            .transition(
                RemoteAccessGrantTransition::Activate,
                context_for(activate_ref),
            )
            .result
            .expect_value("activate saturated grant");
        let pause_ref = Box::leak(format!("attempt-saturated-pause-{index}").into_boxed_str());
        grant
            .transition(RemoteAccessGrantTransition::Pause, context_for(pause_ref))
            .result
            .expect_value("pause saturated grant");
    }
    let encoded = serde_json::to_value(&grant).expect_value("serialize saturated grant");
    assert_eq!(encoded["attempts"].as_array().map(Vec::len), Some(64));

    let revoke_context = context_for("attempt-saturated-revoke");
    let revoked = grant.transition(RemoteAccessGrantTransition::Revoke, revoke_context);
    assert_eq!(revoked.result, Ok(RemoteAccessGrantState::Revoked));
    let encoded = serde_json::to_value(&grant).expect_value("serialize revoked grant");
    assert!(encoded["terminal_milestone"].is_object());
    assert_eq!(encoded["attempts"].as_array().map(Vec::len), Some(64));

    let mut restored: RemoteAccessGrant =
        serde_json::from_value(encoded).expect_value("restore saturated terminal grant");
    assert_eq!(restored.state(), RemoteAccessGrantState::Revoked);
    let replay = restored.transition(RemoteAccessGrantTransition::Revoke, revoke_context);
    assert_eq!(replay.result, Ok(RemoteAccessGrantState::Revoked));
}

#[test]
fn system_failure_stop_uses_reserved_replay_capacity_after_history_saturation() {
    let mut grant = paired_grant();
    for index in 0..31 {
        let activate_ref =
            Box::leak(format!("attempt-saturated-stop-activate-{index}").into_boxed_str());
        grant
            .transition(
                RemoteAccessGrantTransition::Activate,
                context_for(activate_ref),
            )
            .result
            .expect_value("activate saturated grant");
        let pause_ref = Box::leak(format!("attempt-saturated-stop-pause-{index}").into_boxed_str());
        grant
            .transition(RemoteAccessGrantTransition::Pause, context_for(pause_ref))
            .result
            .expect_value("pause saturated grant");
    }

    let mut system_stop = context_for("attempt-saturated-system-stop");
    system_stop.actor_ref = "system-failure";
    system_stop.parent_authorized = false;
    system_stop.transition_authority = RemoteAccessGrantTransitionAuthority::SystemFailure;
    let stopped = grant.transition(RemoteAccessGrantTransition::Stop, system_stop);
    assert_eq!(stopped.result, Ok(RemoteAccessGrantState::Stopped));
    let encoded = serde_json::to_value(&grant).expect_value("serialize saturated system stop");
    assert_eq!(encoded["attempts"].as_array().map(Vec::len), Some(64));
    assert_eq!(encoded["stop_recovery"], serde_json::json!("pending"));
    assert!(encoded["stop_recovery_milestone"].is_object());

    let mut restored: RemoteAccessGrant =
        serde_json::from_value(encoded).expect_value("restore saturated system stop");
    let replay = restored.transition(RemoteAccessGrantTransition::Stop, system_stop);
    assert_eq!(replay.result, stopped.result);
    assert_eq!(replay.audit, stopped.audit);

    let revoked = restored.transition(
        RemoteAccessGrantTransition::Revoke,
        context_for("attempt-saturated-revoke-after-system-stop"),
    );
    assert_eq!(revoked.result, Ok(RemoteAccessGrantState::Revoked));
}

#[test]
fn system_recovery_reserves_capacity_for_reconnect_request_and_completion() {
    let mut grant = paired_grant();
    for index in 0..31 {
        let activate_ref =
            Box::leak(format!("attempt-system-recovery-activate-{index}").into_boxed_str());
        grant
            .transition(
                RemoteAccessGrantTransition::Activate,
                context_for(activate_ref),
            )
            .result
            .expect_value("activate saturated grant");
        let pause_ref =
            Box::leak(format!("attempt-system-recovery-pause-{index}").into_boxed_str());
        grant
            .transition(RemoteAccessGrantTransition::Pause, context_for(pause_ref))
            .result
            .expect_value("pause saturated grant");
    }
    assert_eq!(
        serde_json::to_value(&grant).expect_value("serialize saturated grant")["attempts"]
            .as_array()
            .map(Vec::len),
        Some(64)
    );

    let mut system_stop = context_for("attempt-system-recovery-stop");
    system_stop.actor_ref = "system-failure";
    system_stop.parent_authorized = false;
    system_stop.transition_authority = RemoteAccessGrantTransitionAuthority::SystemFailure;
    assert_eq!(
        grant
            .transition(RemoteAccessGrantTransition::Stop, system_stop)
            .result,
        Ok(RemoteAccessGrantState::Stopped)
    );
    assert_eq!(
        grant
            .transition(
                RemoteAccessGrantTransition::RequestReconnect,
                context_for("attempt-system-recovery-request"),
            )
            .result,
        Ok(RemoteAccessGrantState::ReconnectPending)
    );
    let mut recovery_context = context_for("attempt-system-recovery-reconnect");
    recovery_context.recovery_proof =
        ocentra_remote_access_core::remote_access_grant::RemoteAccessGrantRecoveryProof::SystemConditionCleared;
    assert_eq!(
        grant
            .transition(RemoteAccessGrantTransition::Reconnect, recovery_context)
            .result,
        Ok(RemoteAccessGrantState::Active)
    );
    let mut restarted: RemoteAccessGrant = serde_json::from_value(
        serde_json::to_value(&grant).expect_value("serialize recovered saturated grant"),
    )
    .expect_value("restore recovered saturated grant at a fresh reconnect boundary");
    assert_eq!(restarted.state(), RemoteAccessGrantState::ReconnectPending);
    let mut restarted_context = context_for("attempt-system-recovery-restart");
    restarted_context.recovery_proof =
        ocentra_remote_access_core::remote_access_grant::RemoteAccessGrantRecoveryProof::SystemConditionCleared;
    assert_eq!(
        restarted
            .transition(RemoteAccessGrantTransition::Reconnect, restarted_context)
            .result,
        Ok(RemoteAccessGrantState::Active)
    );
    restarted
        .transition(
            RemoteAccessGrantTransition::Revoke,
            context_for("attempt-system-recovery-terminal-revoke"),
        )
        .result
        .expect_value("terminal invalidation must retain saturated recovery evidence");
    let terminal: RemoteAccessGrant = serde_json::from_value(
        serde_json::to_value(&restarted)
            .expect_value("serialize saturated terminal system recovery"),
    )
    .expect_value("restore saturated terminal system recovery");
    assert_eq!(terminal.state(), RemoteAccessGrantState::Revoked);
}

#[test]
fn restart_reconnect_uses_reserved_replay_capacity_after_active_history_saturation() {
    let mut grant = paired_grant();
    for index in 0..30 {
        let activate_ref =
            Box::leak(format!("attempt-saturated-restart-activate-{index}").into_boxed_str());
        grant
            .transition(
                RemoteAccessGrantTransition::Activate,
                context_for(activate_ref),
            )
            .result
            .expect_value("activate saturated restart grant");
        let pause_ref =
            Box::leak(format!("attempt-saturated-restart-pause-{index}").into_boxed_str());
        grant
            .transition(RemoteAccessGrantTransition::Pause, context_for(pause_ref))
            .result
            .expect_value("pause saturated restart grant");
    }
    grant
        .transition(
            RemoteAccessGrantTransition::RequestReconnect,
            context_for("attempt-saturated-restart-request"),
        )
        .result
        .expect_value("request reconnect at the final replay slot");
    grant
        .transition(
            RemoteAccessGrantTransition::Reconnect,
            context_for("attempt-saturated-restart-prep"),
        )
        .result
        .expect_value("complete reconnect at the final replay slot");
    assert_eq!(grant.state(), RemoteAccessGrantState::Active);
    assert_eq!(
        serde_json::to_value(&grant).expect_value("serialize saturated active grant")["attempts"]
            .as_array()
            .map(Vec::len),
        Some(64)
    );

    let mut restored: RemoteAccessGrant = serde_json::from_value(
        serde_json::to_value(&grant).expect_value("restore saturated active grant"),
    )
    .expect_value("restart recovery must remain available after saturation");
    assert_eq!(restored.state(), RemoteAccessGrantState::ReconnectPending);
    let recovery_context = context_for("attempt-saturated-restart-recovery");
    let recovered = restored.transition(RemoteAccessGrantTransition::Reconnect, recovery_context);
    assert_eq!(recovered.result, Ok(RemoteAccessGrantState::Active));
    assert!(
        serde_json::to_value(&restored).expect_value("serialize recovered saturated grant")
            ["restart_recovery_milestone"]
            .is_object()
    );
    let replay = restored.transition(RemoteAccessGrantTransition::Reconnect, recovery_context);
    assert_eq!(replay.result, recovered.result);
    assert_eq!(replay.audit, recovered.audit);

    let mut restarted: RemoteAccessGrant = serde_json::from_value(
        serde_json::to_value(&restored).expect_value("serialize completed saturated recovery"),
    )
    .expect_value("a later restart must receive a fresh reserved recovery slot");
    assert_eq!(restarted.state(), RemoteAccessGrantState::ReconnectPending);
    assert_eq!(
        restarted
            .transition(
                RemoteAccessGrantTransition::Reconnect,
                context_for("attempt-saturated-restart-second-recovery"),
            )
            .result,
        Ok(RemoteAccessGrantState::Active)
    );
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
fn completed_restart_recovery_retains_its_original_reconnect_boundary() {
    let mut grant = paired_grant();
    grant
        .transition(
            RemoteAccessGrantTransition::Activate,
            context_for("attempt-restart-completed-activate"),
        )
        .result
        .expect_value("activate grant");

    let mut restored: RemoteAccessGrant =
        serde_json::from_value(serde_json::to_value(&grant).expect_value("serialize active grant"))
            .expect_value("restore active grant at reconnect boundary");
    restored
        .transition(
            RemoteAccessGrantTransition::Reconnect,
            context_for("attempt-restart-completed-reconnect"),
        )
        .result
        .expect_value("complete recovered reconnect");
    assert_eq!(restored.state(), RemoteAccessGrantState::Active);

    let mut restarted: RemoteAccessGrant = serde_json::from_value(
        serde_json::to_value(&restored).expect_value("serialize completed recovery"),
    )
    .expect_value("restore completed recovery at a fresh reconnect boundary");
    assert_eq!(restarted.state(), RemoteAccessGrantState::ReconnectPending);
    restarted
        .transition(
            RemoteAccessGrantTransition::Reconnect,
            context_for("attempt-restart-completed-second-reconnect"),
        )
        .result
        .expect_value("complete second recovered reconnect");
    assert_eq!(restarted.state(), RemoteAccessGrantState::Active);

    restarted
        .transition(
            RemoteAccessGrantTransition::Pause,
            context_for("attempt-restart-completed-pause"),
        )
        .result
        .expect_value("pause completed recovery");
    let paused_round_trip: RemoteAccessGrant = serde_json::from_value(
        serde_json::to_value(&restarted).expect_value("serialize paused completed recovery"),
    )
    .expect_value("restore paused completed recovery without moving its boundary");
    assert_eq!(paused_round_trip, restarted);
}

#[test]
fn parent_only_request_rejects_support_actor_role() {
    assert_eq!(
        RemoteAccessGrant::request(
            "grant-parent-only-support",
            "household-1",
            "child-device-1",
            RemoteRoute::LocalNetwork,
            "parent-1",
            RemoteActorRole::SupportAdmin,
            "audit-parent-only-support",
        ),
        Err(RemoteAccessGrantError::WrongActor)
    );
}

#[test]
fn corrected_device_retry_cannot_reuse_an_attempt_for_another_transition() {
    let mut grant = paired_grant();
    let mut wrong_device = context_for("attempt-corrected-device");
    wrong_device.child_device_ref = "child-other";
    assert_eq!(
        grant
            .transition(RemoteAccessGrantTransition::Activate, wrong_device)
            .result,
        Err(RemoteAccessGrantError::WrongDevice)
    );

    let mismatched_transition = grant.transition(
        RemoteAccessGrantTransition::Revoke,
        context_for("attempt-corrected-device"),
    );
    assert_eq!(
        mismatched_transition.result,
        Err(RemoteAccessGrantError::InvalidTransition)
    );
    assert_eq!(grant.state(), RemoteAccessGrantState::Paired);

    assert_eq!(
        grant
            .transition(
                RemoteAccessGrantTransition::Activate,
                context_for("attempt-corrected-device"),
            )
            .result,
        Ok(RemoteAccessGrantState::Active)
    );

    let encoded = serde_json::to_value(&grant).expect_value("serialize corrected-device grant");
    let restored: RemoteAccessGrant =
        serde_json::from_value(encoded.clone()).expect_value("restore corrected-device grant");
    assert_eq!(restored.state(), RemoteAccessGrantState::ReconnectPending);
    assert_eq!(
        serde_json::to_value(&restored).expect_value("serialize restored corrected-device grant")
            ["attempts"],
        encoded["attempts"]
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
    let accepted_replay = grant.transition(
        RemoteAccessGrantTransition::Activate,
        context_for("attempt-child-identity"),
    );
    assert_eq!(accepted_replay.result, retry.result);
    assert_eq!(accepted_replay.audit, retry.audit);
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
fn wrong_household_denial_replay_binds_the_attempted_household() {
    let mut grant = paired_grant();
    let mut wrong_household = context_for("attempt-wrong-household");
    wrong_household.household_ref = "household-other";
    let denied = grant.transition(RemoteAccessGrantTransition::Activate, wrong_household);
    assert_eq!(denied.result, Err(RemoteAccessGrantError::WrongHousehold));
    assert_eq!(denied.audit.household_ref, "household-other");

    let replay = grant.transition(RemoteAccessGrantTransition::Activate, wrong_household);
    assert_eq!(replay.result, denied.result);
    assert_eq!(replay.audit, denied.audit);

    let mut another_household = wrong_household;
    another_household.household_ref = "household-third";
    let different = grant.transition(RemoteAccessGrantTransition::Activate, another_household);
    assert_eq!(
        different.result,
        Err(RemoteAccessGrantError::InvalidTransition)
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
