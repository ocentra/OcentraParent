use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::policy_constants;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandName, AgentEventName, AgentMessageTarget, AgentRoute,
};

#[macro_use]
#[path = "../support/lan_root_harness.rs"]
mod lan_root_harness;
declare_lan_root_harness!();
#[path = "../unit/lan_pairing_test_assertions.rs"]
mod lan_pairing_test_assertions;
#[path = "../unit/lan_pairing_test_commands.rs"]
mod lan_pairing_test_commands;

use crate::{
    app::lan_pairing::LanPairingRuntime,
    app::websocket::handle_command_text_for_test,
    lan_pairing_test_assertions::{
        assert_accepted_control, assert_accepted_control_for_intent, assert_rejection,
        assert_selected_device_reachability, assert_status_selected_route_custody,
        assert_status_selection, assert_status_support_surface, SelectedRouteCustodyExpectation,
    },
    lan_pairing_test_commands::{
        command_for_target, health_command, health_command_for_target, intent_payload,
        intent_payload_for_kind, intent_payload_for_pairing, local_network_target, paired_runtime,
        route_revoke_command, serialize_command, status_command,
    },
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
#[path = "lan_pairing/mdns_advertisement.rs"]
mod mdns_advertisement_tests;
#[path = "lan_pairing/persistent_registry_restart.rs"]
mod persistent_registry_restart_tests;
#[path = "lan_pairing/selected_route_restart.rs"]
mod selected_route_restart_tests;
#[path = "lan_pairing/signed_child_agent.rs"]
mod signed_child_agent_tests;

#[tokio::test]
async fn lan_pairing_rejects_anonymous_wrong_origin_wrong_device_and_revoked_routes() {
    let runtime = paired_runtime().await;

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
async fn lan_pairing_rejects_revoked_route_before_new_control_intent() {
    let runtime = paired_runtime().await;
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
    assert!(stale_runtime.mark_selected_stale_for_test());
    let stale_status = handle_command_text_for_test(
        serialize_command(command_for_target(
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
        serialize_command(health_command(intent_payload(
            constants::lan_pairing::INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        stale_runtime,
        Some(crate::test_text::TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;
    let offline_runtime = paired_runtime().await;
    assert!(offline_runtime.mark_selected_offline_for_test());
    let offline_status = handle_command_text_for_test(
        serialize_command(command_for_target(
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
        serialize_command(health_command(intent_payload(
            constants::lan_pairing::INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        offline_runtime,
        Some(crate::test_text::TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;

    assert_selected_device_reachability(&stale_status, constants::value::LAN_REACHABILITY_STALE);
    assert_status_selected_route_custody(
        &stale_status,
        SelectedRouteCustodyExpectation {
            authentication_state: constants::value::LAN_AUTH_PAIRED,
            selected_child_device_id: constants::lan_pairing::CHILD_DEVICE_ID,
            selected_route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK,
            trusted_device_ids: constants::lan_pairing::CHILD_DEVICE_ID,
            pairing_id: constants::lan_pairing::PAIRING_ID,
            trust_state: constants::value::LAN_PAIRING_PAIRED,
            stale_at: constants::lan_pairing::EXPIRED_AT,
            offline_at: constants::value::EMPTY,
        },
    );
    assert_rejection(&stale_health, constants::value::LAN_REASON_STALE);
    assert_selected_device_reachability(
        &offline_status,
        constants::value::LAN_REACHABILITY_OFFLINE,
    );
    assert_status_selected_route_custody(
        &offline_status,
        SelectedRouteCustodyExpectation {
            authentication_state: constants::value::LAN_AUTH_PAIRED,
            selected_child_device_id: constants::lan_pairing::CHILD_DEVICE_ID,
            selected_route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK,
            trusted_device_ids: constants::lan_pairing::CHILD_DEVICE_ID,
            pairing_id: constants::lan_pairing::PAIRING_ID,
            trust_state: constants::value::LAN_PAIRING_PAIRED,
            stale_at: constants::value::EMPTY,
            offline_at: constants::lan_pairing::OBSERVED_AT,
        },
    );
    assert_rejection(&offline_health, constants::value::LAN_REASON_OFFLINE);
}

#[tokio::test]
async fn lan_pairing_status_reports_online_selected_device_state() {
    let runtime = paired_runtime().await;
    let status = handle_command_text_for_test(
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
    .await;

    assert_selected_device_reachability(&status, constants::value::LAN_REACHABILITY_ONLINE);
}

#[tokio::test]
async fn lan_pairing_status_marks_websocket_ceremony_while_anonymous_control_stays_rejected() {
    let runtime = paired_runtime().await;
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
async fn lan_pairing_rejects_stale_and_replayed_routes() {
    let runtime = paired_runtime().await;
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

    assert_rejection(&stale, constants::value::LAN_REASON_STALE);
    assert_eq!(first.event, AgentEventName::AgentHealthReported);
    assert_accepted_control_for_intent(&first, constants::lan_pairing::REPLAYED_INTENT_ID);
    assert_rejection(&replayed, constants::value::LAN_REASON_REPLAYED);
}

#[tokio::test]
async fn lan_pairing_status_get_is_explicit_for_loopback_and_signed_lan_routes() {
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
    let runtime = paired_runtime().await;
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
            constants::value::LAN_PAIRING_PAIRED.to_string()
        ))
    );
    assert_status_support_surface(&anonymous_lan);
    assert_status_selected_route_custody(
        &anonymous_lan,
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
    assert_eq!(
        anonymous_lan
            .payload
            .get(constants::field::LAN_CONTROL_STATE),
        None
    );
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
