use ocentra_lan_core::network_inventory::passive_discovery::LanPassiveDiscoveryTriggerReason;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::policy_constants;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandName, AgentEventName, AgentMessageTarget, AgentRoute,
};

use crate::{
    app::lan_pairing::LanPairingRuntime,
    app::websocket::handle_command_text_for_test,
    lan_pairing_test_assertions::{
        assert_rejection, assert_status_selection, assert_status_support_surface,
    },
    lan_pairing_test_commands::{
        command_for_target, health_command, health_command_for_target, intent_payload,
        intent_payload_for_kind, intent_payload_for_pairing, local_network_target,
        serialize_command, status_command,
    },
    lan_pairing_test_multidevice_commands::{self, route_revoke_command},
    test_require_json_decode, test_require_log_string_field,
};

#[path = "lan_pairing/controller_lease.rs"]
mod controller_lease_tests;
#[path = "lan_pairing/device_roles.rs"]
mod device_roles_tests;
#[path = "lan_pairing/lan_ai_job_lease.rs"]
mod lan_ai_job_lease_tests;
#[path = "lan_pairing/lan_ai_job.rs"]
mod lan_ai_job_tests;
#[path = "lan_pairing/lan_ai_provider_heartbeat.rs"]
mod lan_ai_provider_heartbeat_tests;
#[path = "lan_pairing/lan_ai_route_metadata.rs"]
mod lan_ai_route_metadata_tests;
#[path = "lan_pairing_multidevice.rs"]
mod lan_pairing_multidevice_tests;
#[path = "lan_pairing/mdns_advertisement.rs"]
mod mdns_advertisement_tests;

#[tokio::test]
async fn lan_pairing_links_background_runtime_state_tasks() {
    let runtime = LanPairingRuntime::empty();
    crate::lan_pairing_runtime_state::mdns_advertisement::spawn_lan_mdns_advertisement_runtime(
        runtime.clone(),
    );
    let _passive_runtime =
        crate::lan_pairing_runtime_state::passive_discovery::start_lan_passive_discovery_service_runtime(
            &runtime,
        );
    tokio::task::yield_now().await;
    let app_resumed_rows = runtime
        .passive_discovery_history_snapshot()
        .rows
        .iter()
        .filter(|row| row.trigger_reason == LanPassiveDiscoveryTriggerReason::AppResumed)
        .count();
    assert_eq!(app_resumed_rows, 1);
}

#[test]
fn lan_pairing_links_exact_clippy_helpers_to_behavioral_contracts() {
    let epoch: String = crate::time::timestamp_from_epoch_seconds(0);
    let after_epoch: String = crate::time::timestamp_after_epoch_seconds(0, 1);
    assert_eq!(epoch, "1970-01-01T00:00:00.000Z");
    assert_eq!(after_epoch, "1970-01-01T00:00:01.000Z");

    let decoded: serde_json::Value = test_require_json_decode::require_json_decode(
        br#"{"status":"paired"}"#,
        "lan pairing helper JSON",
    );
    assert_eq!(decoded["status"], "paired");

    let mut fields = LogFields::new();
    fields.insert(
        constants::field::LAN_PAIRING_ID.to_string(),
        LogFieldValue::String(constants::lan_pairing::PAIRING_ID.to_string()),
    );
    assert_eq!(
        test_require_log_string_field::require_log_string_field(
            fields.get(constants::field::LAN_PAIRING_ID),
            "lan pairing helper field",
        ),
        constants::lan_pairing::PAIRING_ID
    );

    let second_payload = lan_pairing_test_multidevice_commands::second_proof_payload();
    assert_eq!(
        second_payload.get(constants::field::LAN_PAIRING_ID),
        Some(&LogFieldValue::String(
            constants::lan_pairing::SECOND_PAIRING_ID.to_string(),
        ))
    );
    assert_eq!(
        second_payload.get(constants::field::LAN_ROUTE_ID),
        Some(&LogFieldValue::String(
            constants::lan_pairing::ROUTE_ID_SECOND_LOCAL_NETWORK.to_string(),
        ))
    );

    let runtime = LanPairingRuntime::empty();
    let selected = runtime.selected_target();
    assert_eq!(
        crate::app::lan_pairing::route_trust_state(selected.as_ref()).0,
        constants::value::EMPTY
    );
    assert_eq!(
        crate::lan_pairing_status::route_trust_state_for_selected_target(selected.as_ref()).0,
        constants::value::EMPTY
    );
}

#[test]
fn empty_lan_pairing_runtime_starts_unpaired_without_selected_or_revoked_devices() {
    let runtime = LanPairingRuntime::empty();

    assert_eq!(runtime.trusted_device_count(), 0);
    assert!(runtime.trusted_device_ids().is_empty());
    assert!(runtime.revoked_device_ids().is_empty());
    assert_eq!(runtime.selected_target(), None);
    assert!(!runtime.has_revoked_pairing());
}

#[test]
fn unpaired_lan_pairing_runtime_cannot_expose_a_selected_child_route() {
    let runtime = LanPairingRuntime::empty();

    assert_eq!(runtime.trusted_device_count(), 0);
    assert!(runtime.trusted_device_ids().is_empty());
    assert!(runtime.revoked_device_ids().is_empty());
    assert!(!runtime.has_revoked_pairing());
    assert_eq!(runtime.selected_target(), None);
}
#[path = "lan_pairing/persistent_registry_restart.rs"]
mod persistent_registry_restart_tests;
#[path = "lan_pairing/selected_route_restart.rs"]
mod selected_route_restart_tests;
#[path = "lan_pairing/signed_child_agent.rs"]
mod signed_child_agent_tests;

#[tokio::test]
async fn empty_runtime_rejects_lan_control_variants_without_pairing_authority() {
    let runtime = LanPairingRuntime::empty();

    let anonymous = handle_command_text_for_test(
        serialize_command(health_command(LogFields::new())),
        runtime.clone(),
        Some(crate::test_text::TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;
    let wrong_origin = handle_command_text_for_test(
        serialize_command(health_command(intent_payload(
            constants::lan_pairing::INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        runtime.clone(),
        Some(crate::test_text::TestText::from_display(
            constants::lan_pairing::WRONG_ORIGIN,
        )),
    )
    .await;
    let wrong_device = handle_command_text_for_test(
        serialize_command(command_for_target(
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
        Some(crate::test_text::TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;
    let unsupported_route = handle_command_text_for_test(
        serialize_command(health_command(intent_payload_for_pairing(
            constants::lan_pairing::INTENT_ID,
            constants::lan_pairing::PAIRING_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::ROUTE_ID_UNSUPPORTED,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
            constants::value::LAN_INTENT_HEALTH_QUERY,
        ))),
        runtime.clone(),
        Some(crate::test_text::TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;

    assert_rejection(&anonymous, constants::value::LAN_REASON_ANONYMOUS);
    assert_rejection(&wrong_origin, constants::value::LAN_REASON_ANONYMOUS);
    assert_eq!(
        wrong_origin.payload.get(constants::field::ORIGIN),
        Some(&LogFieldValue::String(
            constants::lan_pairing::WRONG_ORIGIN.to_string()
        ))
    );
    assert_rejection(&wrong_device, constants::value::LAN_REASON_ANONYMOUS);
    assert_rejection(&unsupported_route, constants::value::LAN_REASON_ANONYMOUS);
}

#[tokio::test]
async fn unpaired_lan_pairing_rejects_typed_child_intents_before_execution() {
    let runtime = LanPairingRuntime::empty();
    let rule_query = handle_command_text_for_test(
        serialize_command(health_command(intent_payload_for_kind(
            constants::lan_pairing::RULE_QUERY_INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
            constants::value::LAN_INTENT_RULE_QUERY,
        ))),
        runtime.clone(),
        Some(crate::test_text::TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;
    let rule_update = handle_command_text_for_test(
        serialize_command(health_command(intent_payload_for_kind(
            constants::lan_pairing::RULE_UPDATE_INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
            constants::value::LAN_INTENT_RULE_UPDATE,
        ))),
        runtime.clone(),
        Some(crate::test_text::TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;
    let approval_decision = handle_command_text_for_test(
        serialize_command(health_command(intent_payload_for_kind(
            constants::lan_pairing::APPROVAL_DECISION_INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
            constants::value::LAN_INTENT_APPROVAL_DECISION,
        ))),
        runtime,
        Some(crate::test_text::TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;

    for event in [&rule_query, &rule_update, &approval_decision] {
        assert_rejection(event, constants::value::LAN_REASON_ANONYMOUS);
    }
}

#[tokio::test]
async fn lan_pairing_rejects_wrong_command_target_before_child_agent_execution() {
    let runtime = LanPairingRuntime::empty();
    let wrong_target = handle_command_text_for_test(
        serialize_command(health_command_for_target(
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
        Some(crate::test_text::TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;

    assert_rejection(&wrong_target, constants::value::LAN_REASON_WRONG_DEVICE);
    assert_ne!(wrong_target.event, AgentEventName::AgentHealthReported);
}

#[tokio::test]
async fn unpaired_lan_pairing_rejects_revoke_and_control_without_upgrading_state() {
    let runtime = LanPairingRuntime::empty();
    let revoked_status = handle_command_text_for_test(
        serialize_command(route_revoke_command(intent_payload(
            constants::lan_pairing::REVOKE_INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        runtime.clone(),
        Some(crate::test_text::TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;
    let revoked = handle_command_text_for_test(
        serialize_command(health_command(intent_payload(
            constants::lan_pairing::INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        runtime,
        Some(crate::test_text::TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;

    assert_rejection(&revoked_status, constants::value::LAN_REASON_ANONYMOUS);
    assert_rejection(&revoked, constants::value::LAN_REASON_ANONYMOUS);
}

#[test]
fn unpaired_runtime_reachability_mutators_cannot_fabricate_a_selected_device() {
    let runtime = LanPairingRuntime::empty();

    assert!(!runtime.mark_selected_stale_for_test());
    assert!(!runtime.mark_selected_offline_for_test());
    assert_eq!(runtime.selected_target(), None);
}

#[tokio::test]
async fn lan_pairing_status_marks_websocket_ceremony_while_anonymous_control_stays_rejected() {
    let runtime = LanPairingRuntime::empty();
    let loopback_status = handle_command_text_for_test(
        serialize_command(command_for_target(
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
        serialize_command(health_command(LogFields::new())),
        runtime,
        Some(crate::test_text::TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;

    assert_eq!(
        loopback_status.event,
        AgentEventName::AgentLanPairingStatusReported
    );
    assert_status_support_surface(&loopback_status);
    for field in [
        constants::field::ACTIVITY_DIGEST,
        constants::field::BROWSER_EVIDENCE_ID,
        constants::field::CIPHERTEXT,
        constants::field::DATABASE_READY,
        constants::field::ENTRIES,
        constants::field::LAN_EVIDENCE_REFERENCE_IDS,
        constants::field::PROCESS_PATH,
        constants::field::PROFILE_PATH_REF,
        constants::field::URL,
        constants::field::WINDOW_TITLE,
    ] {
        assert_eq!(loopback_status.payload.get(field), None);
    }
    assert_rejection(&anonymous_control, constants::value::LAN_REASON_ANONYMOUS);
}

#[tokio::test]
async fn unpaired_lan_pairing_rejects_stale_then_remains_anonymous_without_consuming_replay() {
    let runtime = LanPairingRuntime::empty();
    let stale = handle_command_text_for_test(
        serialize_command(health_command(intent_payload(
            constants::lan_pairing::INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRED_AT,
        ))),
        runtime.clone(),
        Some(crate::test_text::TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;
    let first = handle_command_text_for_test(
        serialize_command(health_command(intent_payload(
            constants::lan_pairing::REPLAYED_INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        runtime.clone(),
        Some(crate::test_text::TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;
    let replayed = handle_command_text_for_test(
        serialize_command(health_command(intent_payload(
            constants::lan_pairing::REPLAYED_INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        runtime,
        Some(crate::test_text::TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;

    assert_rejection(&stale, constants::value::LAN_REASON_ANONYMOUS);
    assert_rejection(&first, constants::value::LAN_REASON_ANONYMOUS);
    assert_rejection(&replayed, constants::value::LAN_REASON_ANONYMOUS);
}

#[tokio::test]
async fn lan_pairing_status_get_reports_unpaired_and_rejects_unauthorized_audited_lan_reads() {
    let loopback = handle_command_text_for_test(
        serialize_command(command_for_target(
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
    let runtime = LanPairingRuntime::empty();
    let anonymous_lan = handle_command_text_for_test(
        serialize_command(status_command(LogFields::new())),
        runtime.clone(),
        Some(crate::test_text::TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;
    let audited_lan = handle_command_text_for_test(
        serialize_command(status_command(intent_payload(
            constants::lan_pairing::INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        runtime,
        Some(crate::test_text::TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
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
    assert_eq!(
        anonymous_lan.event,
        AgentEventName::AgentLanPairingStatusReported
    );
    assert_eq!(
        anonymous_lan
            .payload
            .get(constants::field::LAN_PAIRING_STATE),
        Some(&LogFieldValue::String(
            constants::value::LAN_PAIRING_UNPAIRED.to_string()
        ))
    );
    assert_status_support_surface(&anonymous_lan);
    assert_status_selection(
        &anonymous_lan,
        constants::value::LAN_AUTH_UNPAIRED,
        constants::value::EMPTY,
        constants::value::EMPTY,
        constants::value::EMPTY,
    );
    assert_eq!(
        anonymous_lan
            .payload
            .get(constants::field::LAN_CONTROL_STATE),
        None
    );
    assert_rejection(&audited_lan, constants::value::LAN_REASON_ANONYMOUS);
}
