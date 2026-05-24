use std::fs::remove_file;

use ocentra_parent_agent_protocol::{
    constants, policy_constants, AgentCommandName, AgentEventEnvelope, AgentEventName,
    AgentMessageTarget, AgentRoute, LogFieldValue, LogFields,
};

use crate::{
    lan_pairing::LanPairingRuntime,
    lan_pairing_test_support::{
        assert_accepted_control, assert_persistent_status_support_surface, assert_rejection,
        assert_selected_device_reachability, assert_status_selection,
        assert_status_support_surface, command_for_target, health_command,
        health_command_for_target, intent_payload, intent_payload_for_kind,
        intent_payload_for_pairing, local_network_target, paired_runtime, pairing_command,
        proof_payload, route_revoke_command, route_select_command, serialize_command,
        status_command,
    },
    websocket::handle_command_text_for_test,
};

#[tokio::test]
async fn lan_pairing_rejects_anonymous_wrong_origin_wrong_device_and_revoked_routes() {
    let runtime = paired_runtime().await;

    let anonymous = handle_command_text_for_test(
        &serialize_command(health_command(LogFields::new())),
        runtime.clone(),
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;
    let wrong_origin = handle_command_text_for_test(
        &serialize_command(health_command(intent_payload(
            constants::lan_pairing::INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        runtime.clone(),
        Some(constants::lan_pairing::WRONG_ORIGIN.to_string()),
    )
    .await;
    let wrong_device = handle_command_text_for_test(
        &serialize_command(command_for_target(
            AgentCommandName::AgentHealthCheck,
            local_network_target(constants::peer::LOCAL_DEV_AGENT),
            intent_payload(
                constants::lan_pairing::INTENT_ID,
                constants::peer::LOCAL_DEV_AGENT,
                constants::lan_pairing::PROOF_DIGEST,
                constants::lan_pairing::EXPIRES_AT,
            ),
        )),
        runtime.clone(),
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;
    let unsupported_route = handle_command_text_for_test(
        &serialize_command(health_command(intent_payload_for_pairing(
            constants::lan_pairing::INTENT_ID,
            constants::lan_pairing::PAIRING_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::ROUTE_ID_UNSUPPORTED,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
            constants::value::LAN_INTENT_HEALTH_QUERY,
        ))),
        runtime.clone(),
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;

    assert_rejection(&anonymous, constants::value::LAN_REASON_ANONYMOUS);
    assert_rejection(&wrong_origin, constants::value::LAN_REASON_WRONG_ORIGIN);
    assert_eq!(
        wrong_origin.payload.get(constants::field::ORIGIN),
        Some(&LogFieldValue::String(
            constants::lan_pairing::WRONG_ORIGIN.to_string()
        ))
    );
    assert_rejection(&wrong_device, constants::value::LAN_REASON_WRONG_DEVICE);
    assert_rejection(
        &unsupported_route,
        constants::value::LAN_REASON_UNSUPPORTED_ROUTE,
    );
}

#[tokio::test]
async fn lan_pairing_accepts_typed_rule_query_rule_update_and_approval_intents_child_side() {
    let runtime = paired_runtime().await;
    let rule_query = handle_command_text_for_test(
        &serialize_command(health_command(intent_payload_for_kind(
            constants::lan_pairing::RULE_QUERY_INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
            constants::value::LAN_INTENT_RULE_QUERY,
        ))),
        runtime.clone(),
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;
    let rule_update = handle_command_text_for_test(
        &serialize_command(health_command(intent_payload_for_kind(
            constants::lan_pairing::RULE_UPDATE_INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
            constants::value::LAN_INTENT_RULE_UPDATE,
        ))),
        runtime.clone(),
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;
    let approval_decision = handle_command_text_for_test(
        &serialize_command(health_command(intent_payload_for_kind(
            constants::lan_pairing::APPROVAL_DECISION_INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
            constants::value::LAN_INTENT_APPROVAL_DECISION,
        ))),
        runtime,
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;

    assert_eq!(rule_query.event, AgentEventName::AgentHealthReported);
    assert_eq!(rule_update.event, AgentEventName::AgentHealthReported);
    assert_eq!(approval_decision.event, AgentEventName::AgentHealthReported);
    assert_eq!(
        rule_query.payload.get(constants::field::LAN_INTENT_KIND),
        Some(&LogFieldValue::String(
            constants::value::LAN_INTENT_RULE_QUERY.to_string()
        ))
    );
    assert_eq!(
        rule_update.payload.get(constants::field::LAN_INTENT_KIND),
        Some(&LogFieldValue::String(
            constants::value::LAN_INTENT_RULE_UPDATE.to_string()
        ))
    );
    assert_eq!(
        approval_decision
            .payload
            .get(constants::field::LAN_INTENT_KIND),
        Some(&LogFieldValue::String(
            constants::value::LAN_INTENT_APPROVAL_DECISION.to_string()
        ))
    );
}

#[tokio::test]
async fn lan_pairing_rejects_wrong_command_target_before_child_agent_execution() {
    let runtime = paired_runtime().await;
    let wrong_target = handle_command_text_for_test(
        &serialize_command(health_command_for_target(
            constants::lan_pairing::SECOND_CHILD_DEVICE_ID,
            intent_payload_for_kind(
                constants::lan_pairing::RULE_QUERY_INTENT_ID,
                constants::lan_pairing::CHILD_DEVICE_ID,
                constants::lan_pairing::PROOF_DIGEST,
                constants::lan_pairing::EXPIRES_AT,
                constants::value::LAN_INTENT_RULE_QUERY,
            ),
        )),
        runtime,
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;

    assert_rejection(&wrong_target, constants::value::LAN_REASON_WRONG_DEVICE);
    assert_ne!(wrong_target.event, AgentEventName::AgentHealthReported);
}

#[tokio::test]
async fn lan_pairing_rejects_revoked_route_before_new_control_intent() {
    let runtime = paired_runtime().await;
    let revoked_status = handle_command_text_for_test(
        &serialize_command(route_revoke_command(intent_payload(
            constants::lan_pairing::REVOKE_INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        runtime.clone(),
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;
    let revoked = handle_command_text_for_test(
        &serialize_command(health_command(intent_payload(
            constants::lan_pairing::INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        runtime,
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;

    assert_eq!(
        revoked_status.event,
        AgentEventName::AgentLanPairingStatusReported
    );
    assert_eq!(
        revoked_status
            .payload
            .get(constants::field::LAN_AUDIT_EVENT_TYPE),
        Some(&LogFieldValue::String(
            constants::value::LAN_AUDIT_PAIRING_REVOKED.to_string()
        ))
    );
    assert_eq!(
        revoked_status
            .payload
            .get(constants::field::LAN_PAIRING_STATE),
        Some(&LogFieldValue::String(
            constants::value::LAN_PAIRING_REVOKED.to_string()
        ))
    );
    assert_eq!(
        revoked_status
            .payload
            .get(constants::field::LAN_REVOKED_DEVICE_IDS),
        Some(&LogFieldValue::String(
            constants::lan_pairing::CHILD_DEVICE_ID.to_string()
        ))
    );
    assert_rejection(&revoked, constants::value::LAN_REASON_REVOKED);
}

#[tokio::test]
async fn lan_pairing_status_reports_stale_and_offline_selected_device_state() {
    let stale_runtime = paired_runtime().await;
    assert!(stale_runtime.mark_selected_stale_for_test(constants::lan_pairing::EXPIRED_AT));
    let stale_status = handle_command_text_for_test(
        &serialize_command(command_for_target(
            AgentCommandName::AgentLanPairingStatusGet,
            AgentMessageTarget {
                device_id: constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
                platform: policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS.to_string(),
                route: AgentRoute::Localhost,
            },
            LogFields::new(),
        )),
        stale_runtime.clone(),
        None,
    )
    .await;
    let stale_health = handle_command_text_for_test(
        &serialize_command(health_command(intent_payload(
            constants::lan_pairing::INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        stale_runtime,
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;
    let offline_runtime = paired_runtime().await;
    assert!(offline_runtime.mark_selected_offline_for_test(constants::lan_pairing::OBSERVED_AT));
    let offline_status = handle_command_text_for_test(
        &serialize_command(command_for_target(
            AgentCommandName::AgentLanPairingStatusGet,
            AgentMessageTarget {
                device_id: constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
                platform: policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS.to_string(),
                route: AgentRoute::Localhost,
            },
            LogFields::new(),
        )),
        offline_runtime.clone(),
        None,
    )
    .await;
    let offline_health = handle_command_text_for_test(
        &serialize_command(health_command(intent_payload(
            constants::lan_pairing::INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        offline_runtime,
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;

    assert_selected_device_reachability(&stale_status, constants::value::LAN_REACHABILITY_STALE);
    assert_rejection(&stale_health, constants::value::LAN_REASON_STALE);
    assert_selected_device_reachability(
        &offline_status,
        constants::value::LAN_REACHABILITY_OFFLINE,
    );
    assert_rejection(&offline_health, constants::value::LAN_REASON_OFFLINE);
}

#[tokio::test]
async fn lan_pairing_status_marks_discovery_planned_while_anonymous_control_stays_rejected() {
    let runtime = paired_runtime().await;
    let loopback_status = handle_command_text_for_test(
        &serialize_command(command_for_target(
            AgentCommandName::AgentLanPairingStatusGet,
            AgentMessageTarget {
                device_id: constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
                platform: policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS.to_string(),
                route: AgentRoute::Localhost,
            },
            LogFields::new(),
        )),
        runtime.clone(),
        None,
    )
    .await;
    let anonymous_control = handle_command_text_for_test(
        &serialize_command(health_command(LogFields::new())),
        runtime,
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;

    assert_eq!(
        loopback_status.event,
        AgentEventName::AgentLanPairingStatusReported
    );
    assert_status_support_surface(&loopback_status);
    assert_rejection(&anonymous_control, constants::value::LAN_REASON_ANONYMOUS);
}

#[tokio::test]
async fn lan_pairing_rejects_stale_and_replayed_routes() {
    let runtime = paired_runtime().await;
    let stale = handle_command_text_for_test(
        &serialize_command(health_command(intent_payload(
            constants::lan_pairing::INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRED_AT,
        ))),
        runtime.clone(),
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;
    let first = handle_command_text_for_test(
        &serialize_command(health_command(intent_payload(
            constants::lan_pairing::REPLAYED_INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        runtime.clone(),
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;
    let replayed = handle_command_text_for_test(
        &serialize_command(health_command(intent_payload(
            constants::lan_pairing::REPLAYED_INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        runtime,
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;

    assert_rejection(&stale, constants::value::LAN_REASON_STALE);
    assert_eq!(first.event, AgentEventName::AgentHealthReported);
    assert_accepted_control(&first);
    assert_rejection(&replayed, constants::value::LAN_REASON_REPLAYED);
}

#[tokio::test]
async fn lan_pairing_status_get_is_explicit_for_loopback_and_signed_lan_routes() {
    let loopback = handle_command_text_for_test(
        &serialize_command(command_for_target(
            AgentCommandName::AgentLanPairingStatusGet,
            AgentMessageTarget {
                device_id: constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
                platform: policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS.to_string(),
                route: AgentRoute::Localhost,
            },
            LogFields::new(),
        )),
        LanPairingRuntime::empty(),
        None,
    )
    .await;
    let runtime = paired_runtime().await;
    let anonymous_lan = handle_command_text_for_test(
        &serialize_command(status_command(LogFields::new())),
        runtime.clone(),
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;
    let audited_lan = handle_command_text_for_test(
        &serialize_command(status_command(intent_payload(
            constants::lan_pairing::INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        runtime,
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;

    assert_eq!(
        loopback.event,
        AgentEventName::AgentLanPairingStatusReported
    );
    assert_eq!(
        loopback.payload.get(constants::field::LAN_PAIRING_STATE),
        Some(&LogFieldValue::String(
            constants::value::LAN_PAIRING_UNPAIRED.to_string()
        ))
    );
    assert_status_support_surface(&loopback);
    assert_status_selection(
        &loopback,
        constants::value::LAN_AUTH_UNPAIRED,
        constants::value::EMPTY,
        constants::value::EMPTY,
        constants::value::EMPTY,
    );
    assert_eq!(
        loopback.payload.get(constants::field::LAN_CONTROL_STATE),
        None
    );
    assert_rejection(&anonymous_lan, constants::value::LAN_REASON_ANONYMOUS);
    assert_eq!(
        audited_lan.event,
        AgentEventName::AgentLanPairingStatusReported
    );
    assert_eq!(
        audited_lan.payload.get(constants::field::LAN_PAIRING_STATE),
        Some(&LogFieldValue::String(
            constants::value::LAN_PAIRING_PAIRED.to_string()
        ))
    );
    assert_status_support_surface(&audited_lan);
    assert_accepted_control(&audited_lan);
}

#[tokio::test]
async fn lan_pairing_restart_without_registry_persistence_fails_closed() {
    let before_restart_runtime = paired_runtime().await;
    let before_restart_status = loopback_lan_status(before_restart_runtime).await;
    let restarted_runtime = LanPairingRuntime::empty();
    let restarted_status = loopback_lan_status(restarted_runtime.clone()).await;
    let old_signed_control = handle_command_text_for_test(
        &serialize_command(health_command(intent_payload(
            constants::lan_pairing::INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        restarted_runtime,
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;

    assert_lan_pairing_state(
        &before_restart_status,
        constants::value::LAN_PAIRING_PAIRED,
        1.0,
    );
    assert_lan_pairing_state(
        &restarted_status,
        constants::value::LAN_PAIRING_UNPAIRED,
        0.0,
    );
    assert_status_support_surface(&restarted_status);
    assert_status_selection(
        &restarted_status,
        constants::value::LAN_AUTH_UNPAIRED,
        constants::value::EMPTY,
        constants::value::EMPTY,
        constants::value::EMPTY,
    );
    assert_rejection(&old_signed_control, constants::value::LAN_REASON_ANONYMOUS);
    assert_eq!(
        old_signed_control
            .payload
            .get(constants::field::LAN_EVIDENCE_REFERENCE_IDS),
        Some(&LogFieldValue::String(
            constants::lan_pairing::EVIDENCE_REFERENCE_ID.to_string()
        ))
    );
}

#[tokio::test]
async fn lan_pairing_persistent_registry_restores_trusted_device_unselected_after_restart() {
    let path = temp_registry_path();
    let _ = remove_file(&path);
    let runtime = LanPairingRuntime::persistent_json(&path);
    let _ = handle_command_text_for_test(
        &serialize_command(pairing_command(proof_payload())),
        runtime,
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;
    let restarted_runtime = LanPairingRuntime::persistent_json(&path);
    let restarted_status = loopback_lan_status(restarted_runtime.clone()).await;
    let rejected_before_selection = old_signed_control(restarted_runtime.clone()).await;
    let route_selected = handle_command_text_for_test(
        &serialize_command(route_select_command(intent_payload(
            constants::lan_pairing::SELECT_INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        restarted_runtime.clone(),
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;
    let accepted_after_selection = old_signed_control(restarted_runtime).await;
    let _ = remove_file(&path);

    assert_lan_pairing_state(&restarted_status, constants::value::LAN_PAIRING_PAIRED, 1.0);
    assert_persistent_status_support_surface(&restarted_status);
    assert_status_selection(
        &restarted_status,
        constants::value::LAN_AUTH_UNPAIRED,
        constants::value::EMPTY,
        constants::value::EMPTY,
        constants::lan_pairing::CHILD_DEVICE_ID,
    );
    assert_rejection(
        &rejected_before_selection,
        constants::value::LAN_REASON_UNSELECTED_DEVICE,
    );
    assert_status_selection(
        &route_selected,
        constants::value::LAN_AUTH_PAIRED,
        constants::lan_pairing::CHILD_DEVICE_ID,
        constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK,
        constants::lan_pairing::CHILD_DEVICE_ID,
    );
    assert_accepted_control(&accepted_after_selection);
}

#[tokio::test]
async fn lan_pairing_persistent_registry_keeps_revocation_after_restart() {
    let path = temp_registry_path();
    let _ = remove_file(&path);
    let runtime = LanPairingRuntime::persistent_json(&path);
    let _ = handle_command_text_for_test(
        &serialize_command(pairing_command(proof_payload())),
        runtime.clone(),
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;
    let _ = handle_command_text_for_test(
        &serialize_command(route_select_command(intent_payload(
            constants::lan_pairing::SELECT_INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        runtime.clone(),
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;
    let _ = handle_command_text_for_test(
        &serialize_command(route_revoke_command(intent_payload(
            constants::lan_pairing::REVOKE_INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        runtime,
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;
    let restarted_runtime = LanPairingRuntime::persistent_json(&path);
    let restarted_status = loopback_lan_status(restarted_runtime.clone()).await;
    let revoked_control = old_signed_control(restarted_runtime).await;
    let _ = remove_file(&path);

    assert_lan_pairing_state(
        &restarted_status,
        constants::value::LAN_PAIRING_REVOKED,
        0.0,
    );
    assert_persistent_status_support_surface(&restarted_status);
    assert_eq!(
        restarted_status
            .payload
            .get(constants::field::LAN_REVOKED_DEVICE_IDS),
        Some(&LogFieldValue::String(
            constants::lan_pairing::CHILD_DEVICE_ID.to_string()
        ))
    );
    assert_rejection(&revoked_control, constants::value::LAN_REASON_REVOKED);
}

async fn loopback_lan_status(runtime: LanPairingRuntime) -> AgentEventEnvelope {
    handle_command_text_for_test(
        &serialize_command(command_for_target(
            AgentCommandName::AgentLanPairingStatusGet,
            AgentMessageTarget {
                device_id: constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
                platform: policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS.to_string(),
                route: AgentRoute::Localhost,
            },
            LogFields::new(),
        )),
        runtime,
        None,
    )
    .await
}

async fn old_signed_control(runtime: LanPairingRuntime) -> AgentEventEnvelope {
    handle_command_text_for_test(
        &serialize_command(health_command(intent_payload(
            constants::lan_pairing::INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        runtime,
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await
}

fn assert_lan_pairing_state(event: &AgentEventEnvelope, pairing_state: &str, trusted_count: f64) {
    assert_eq!(
        event.payload.get(constants::field::LAN_PAIRING_STATE),
        Some(&LogFieldValue::String(pairing_state.to_string()))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_TRUSTED_DEVICE_COUNT),
        Some(&LogFieldValue::Number(trusted_count))
    );
}

fn temp_registry_path() -> std::path::PathBuf {
    let mut name = String::from(constants::lan_pairing::REGISTRY_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push_str(
        &std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |duration| duration.as_nanos())
            .to_string(),
    );
    let mut path = std::env::temp_dir();
    path.push(name);
    path.set_extension(constants::lan_pairing::REGISTRY_FILE_EXTENSION);
    path
}
