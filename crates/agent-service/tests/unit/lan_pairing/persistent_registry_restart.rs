use crate::test_text::TestText;

use std::{
    fs::remove_file,
    sync::atomic::{AtomicUsize, Ordering},
};

use ocentra_parent_agent_protocol::{
    constants,
    logging::{LogFieldValue, LogFields},
    policy_constants,
    transport::{
        AgentCommandName, AgentEventEnvelope, AgentEventName, AgentMessageTarget, AgentRoute,
    },
};

use crate::{
    app::{lan_pairing::LanPairingRuntime, websocket::handle_command_text_for_test},
    lan_pairing_test_assertions::{
        assert_accepted_control, assert_accepted_control_for_intent,
        assert_persistent_status_support_surface, assert_rejection,
        assert_status_selected_route_custody, assert_status_selection,
        assert_status_support_surface, SelectedRouteCustodyExpectation,
    },
    lan_pairing_test_commands::{
        command_for_target, health_command, intent_payload, intent_payload_for_kind,
        paired_runtime, pairing_command, proof_payload, route_select_command, serialize_command,
    },
    lan_pairing_test_multidevice_commands::route_revoke_command,
};

#[tokio::test]
async fn lan_pairing_restart_without_registry_persistence_fails_closed() {
    let before_restart_runtime = paired_runtime().await;
    let before_restart_status = loopback_lan_status(before_restart_runtime).await;
    let restarted_runtime = LanPairingRuntime::empty();
    let restarted_status = loopback_lan_status(restarted_runtime.clone()).await;
    let old_signed_control = handle_command_text_for_test(
        serialize_command(health_command(intent_payload(
            constants::lan_pairing::INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        restarted_runtime,
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;

    assert_lan_pairing_state(
        &before_restart_status,
        LanPairingStateExpectation {
            pairing_state: constants::value::LAN_PAIRING_PAIRED,
            trusted_count: 1.0,
        },
    );
    assert_lan_pairing_state(
        &restarted_status,
        LanPairingStateExpectation {
            pairing_state: constants::value::LAN_PAIRING_UNPAIRED,
            trusted_count: 0.0,
        },
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
    let mut path = std::env::temp_dir();
    path.push(temp_registry_name().0);
    path.set_extension(constants::lan_pairing::REGISTRY_FILE_EXTENSION);
    let _ = remove_file(&path);
    let runtime = LanPairingRuntime::persistent_json(&path);
    let _ = handle_command_text_for_test(
        serialize_command(pairing_command(proof_payload())),
        runtime,
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;
    let restarted_runtime = LanPairingRuntime::persistent_json(&path);
    let restarted_status = loopback_lan_status(restarted_runtime.clone()).await;
    let rejected_before_selection = old_signed_control(restarted_runtime.clone()).await;
    let route_selected = handle_command_text_for_test(
        serialize_command(route_select_command(intent_payload(
            constants::lan_pairing::SELECT_INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        restarted_runtime.clone(),
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;
    let accepted_after_selection = old_signed_control(restarted_runtime).await;
    let _ = remove_file(&path);

    assert_lan_pairing_state(
        &restarted_status,
        LanPairingStateExpectation {
            pairing_state: constants::value::LAN_PAIRING_PAIRED,
            trusted_count: 1.0,
        },
    );
    assert_persistent_status_support_surface(&restarted_status);
    assert_status_selection(
        &restarted_status,
        constants::value::LAN_AUTH_UNPAIRED,
        constants::value::EMPTY,
        constants::value::EMPTY,
        constants::lan_pairing::CHILD_DEVICE_ID,
    );
    assert_status_selected_route_custody(
        &restarted_status,
        SelectedRouteCustodyExpectation {
            authentication_state: constants::value::LAN_AUTH_UNPAIRED,
            selected_child_device_id: constants::value::EMPTY,
            selected_route_id: constants::value::EMPTY,
            trusted_device_ids: constants::lan_pairing::CHILD_DEVICE_ID,
            pairing_id: constants::value::EMPTY,
            trust_state: constants::value::EMPTY,
            stale_at: constants::value::EMPTY,
            offline_at: constants::value::EMPTY,
        },
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
    assert_status_selected_route_custody(
        &route_selected,
        SelectedRouteCustodyExpectation {
            authentication_state: constants::value::LAN_AUTH_PAIRED,
            selected_child_device_id: constants::lan_pairing::CHILD_DEVICE_ID,
            selected_route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK,
            trusted_device_ids: constants::lan_pairing::CHILD_DEVICE_ID,
            pairing_id: constants::lan_pairing::PAIRING_ID,
            trust_state: constants::value::LAN_PAIRING_PAIRED,
            stale_at: constants::value::EMPTY,
            offline_at: constants::value::EMPTY,
        },
    );
    assert_accepted_control(&accepted_after_selection);
}

#[tokio::test]
async fn lan_pairing_persistent_registry_requires_selection_for_rule_and_approval_after_restart() {
    let mut path = std::env::temp_dir();
    path.push(temp_registry_name().0);
    path.set_extension(constants::lan_pairing::REGISTRY_FILE_EXTENSION);
    let _ = remove_file(&path);
    let runtime = LanPairingRuntime::persistent_json(&path);
    let _ = handle_command_text_for_test(
        serialize_command(pairing_command(proof_payload())),
        runtime,
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;
    let restarted_runtime = LanPairingRuntime::persistent_json(&path);
    let rejected_rule_query = signed_control_for_kind(
        restarted_runtime.clone(),
        TestText::from_display(constants::lan_pairing::RULE_QUERY_INTENT_ID),
        TestText::from_display(constants::value::LAN_INTENT_RULE_QUERY),
    )
    .await;
    let route_selected = handle_command_text_for_test(
        serialize_command(route_select_command(intent_payload(
            constants::lan_pairing::SELECT_INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        restarted_runtime.clone(),
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;
    let accepted_rule_query = signed_control_for_kind(
        restarted_runtime.clone(),
        TestText::from_display(constants::lan_pairing::RULE_QUERY_INTENT_ID),
        TestText::from_display(constants::value::LAN_INTENT_RULE_QUERY),
    )
    .await;
    let accepted_approval = signed_control_for_kind(
        restarted_runtime,
        TestText::from_display(constants::lan_pairing::APPROVAL_DECISION_INTENT_ID),
        TestText::from_display(constants::value::LAN_INTENT_APPROVAL_DECISION),
    )
    .await;
    let _ = remove_file(&path);

    assert_rejection(
        &rejected_rule_query,
        constants::value::LAN_REASON_UNSELECTED_DEVICE,
    );
    assert_eq!(
        rejected_rule_query
            .payload
            .get(constants::field::LAN_INTENT_KIND),
        Some(&LogFieldValue::String(
            constants::value::LAN_INTENT_RULE_QUERY.to_string()
        ))
    );
    assert_status_selection(
        &route_selected,
        constants::value::LAN_AUTH_PAIRED,
        constants::lan_pairing::CHILD_DEVICE_ID,
        constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK,
        constants::lan_pairing::CHILD_DEVICE_ID,
    );
    assert_restart_accepted_intent(
        &accepted_rule_query,
        RestartAcceptedIntentExpectation {
            intent_id: constants::lan_pairing::RULE_QUERY_INTENT_ID,
            intent_kind: constants::value::LAN_INTENT_RULE_QUERY,
        },
    );
    assert_restart_accepted_intent(
        &accepted_approval,
        RestartAcceptedIntentExpectation {
            intent_id: constants::lan_pairing::APPROVAL_DECISION_INTENT_ID,
            intent_kind: constants::value::LAN_INTENT_APPROVAL_DECISION,
        },
    );
}

#[tokio::test]
async fn lan_pairing_persistent_registry_keeps_revocation_after_restart() {
    let mut path = std::env::temp_dir();
    path.push(temp_registry_name().0);
    path.set_extension(constants::lan_pairing::REGISTRY_FILE_EXTENSION);
    let _ = remove_file(&path);
    let runtime = LanPairingRuntime::persistent_json(&path);
    let _ = handle_command_text_for_test(
        serialize_command(pairing_command(proof_payload())),
        runtime.clone(),
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;
    let _ = handle_command_text_for_test(
        serialize_command(route_select_command(intent_payload(
            constants::lan_pairing::SELECT_INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        runtime.clone(),
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;
    let _ = handle_command_text_for_test(
        serialize_command(route_revoke_command(intent_payload(
            constants::lan_pairing::REVOKE_INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        runtime,
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;
    let restarted_runtime = LanPairingRuntime::persistent_json(&path);
    let restarted_status = loopback_lan_status(restarted_runtime.clone()).await;
    let revoked_control = old_signed_control(restarted_runtime).await;
    let _ = remove_file(&path);

    assert_lan_pairing_state(
        &restarted_status,
        LanPairingStateExpectation {
            pairing_state: constants::value::LAN_PAIRING_REVOKED,
            trusted_count: 0.0,
        },
    );
    assert_persistent_status_support_surface(&restarted_status);
    assert_status_selected_route_custody(
        &restarted_status,
        SelectedRouteCustodyExpectation {
            authentication_state: constants::value::LAN_AUTH_UNPAIRED,
            selected_child_device_id: constants::value::EMPTY,
            selected_route_id: constants::value::EMPTY,
            trusted_device_ids: constants::value::EMPTY,
            pairing_id: constants::value::EMPTY,
            trust_state: constants::value::EMPTY,
            stale_at: constants::value::EMPTY,
            offline_at: constants::value::EMPTY,
        },
    );
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
        serialize_command(command_for_target(
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
        serialize_command(health_command(intent_payload(
            constants::lan_pairing::INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        runtime,
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await
}

async fn signed_control_for_kind(
    runtime: LanPairingRuntime,
    intent_id: TestText,
    intent_kind: TestText,
) -> AgentEventEnvelope {
    handle_command_text_for_test(
        serialize_command(health_command(intent_payload_for_kind(
            intent_id.0.as_str(),
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
            intent_kind.0.as_str(),
        ))),
        runtime,
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await
}

#[derive(Clone, Copy)]
struct RestartAcceptedIntentExpectation {
    intent_id: &'static str,
    intent_kind: &'static str,
}

fn assert_restart_accepted_intent(
    event: &AgentEventEnvelope,
    expectation: RestartAcceptedIntentExpectation,
) {
    assert_eq!(event.event, AgentEventName::AgentHealthReported);
    assert_accepted_control_for_intent(event, expectation.intent_id);
    assert_eq!(
        event.payload.get(constants::field::LAN_INTENT_KIND),
        Some(&LogFieldValue::String(expectation.intent_kind.to_string()))
    );
}

#[derive(Clone, Copy)]
struct LanPairingStateExpectation {
    pairing_state: &'static str,
    trusted_count: f64,
}

fn assert_lan_pairing_state(event: &AgentEventEnvelope, expectation: LanPairingStateExpectation) {
    assert_eq!(
        event.payload.get(constants::field::LAN_PAIRING_STATE),
        Some(&LogFieldValue::String(
            expectation.pairing_state.to_string()
        ))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_TRUSTED_DEVICE_COUNT),
        Some(&LogFieldValue::Number(expectation.trusted_count))
    );
}

fn temp_registry_name() -> TestText {
    static REGISTRY_COUNTER: AtomicUsize = AtomicUsize::new(0);

    let mut name = String::from(constants::lan_pairing::REGISTRY_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push_str(&REGISTRY_COUNTER.fetch_add(1, Ordering::Relaxed).to_string());
    name.push_str(
        &std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |duration| duration.as_nanos())
            .to_string(),
    );
    TestText(name)
}
