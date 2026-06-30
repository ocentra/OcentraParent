use std::{
    fs::remove_file,
    sync::atomic::{AtomicUsize, Ordering},
    sync::Mutex,
};

use base64::{engine::general_purpose::STANDARD, Engine as _};
use chrono::{Duration, SecondsFormat, Utc};
use ed25519_dalek::{Signer, SigningKey};
use ocentra_lan_core::lan_mdns_advertiser::current_platform_support;
use ocentra_lan_core::lan_mdns_advertiser::LanMdnsPacketSink;
use ocentra_lan_core::lan_pairing::LanMdnsAdvertisementPlatformSupport;
use ocentra_lan_core::lan_pairing::LanSignedChildAgentVerificationContext;
use ocentra_lan_core::network_inventory::passive_discovery::{
    LanPassiveDiscoverySource, LanPassiveDiscoveryTriggerReason,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingRejectionReason, LanSignedChildAgentClaim, LanSignedChildAgentEnvelope,
    LanSignedChildAgentMessageKind,
};
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::policy_constants;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandName, AgentEventEnvelope, AgentEventName, AgentMessageTarget, AgentRoute,
};

use crate::{
    app::lan_pairing::LanPairingRuntime,
    app::lan_pairing_runtime_state::mdns_advertisement::LanMdnsAdvertisementSyncState,
    app::lan_pairing_status::pairing_status_event,
    app::websocket::handle_command_text_for_test,
    lan_pairing_test_assertions::{
        assert_accepted_control, assert_accepted_control_for_intent,
        assert_persistent_status_support_surface, assert_rejection,
        assert_selected_device_reachability, assert_status_selected_route_custody,
        assert_status_selection, assert_status_support_surface, SelectedRouteCustodyExpectation,
    },
    lan_pairing_test_commands::{
        command_for_target, health_command, health_command_for_target, intent_payload,
        intent_payload_for_kind, intent_payload_for_pairing, local_network_target, paired_runtime,
        pairing_command, proof_payload, route_revoke_command, route_select_command,
        serialize_command, status_command,
    },
    lan_runtime_test_support::{
        default_child_mdns_advertisement_fixture, LanChildMdnsAdvertisementFixture,
    },
    test_invariants::{require_ok, require_some},
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
#[path = "lan_pairing/selected_route_restart.rs"]
mod selected_route_restart_tests;

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
    .await;

    assert_selected_device_reachability(&status, constants::value::LAN_REACHABILITY_ONLINE);
}

#[tokio::test]
async fn lan_pairing_status_marks_websocket_ceremony_while_anonymous_control_stays_rejected() {
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
    assert_accepted_control_for_intent(&first, constants::lan_pairing::REPLAYED_INTENT_ID);
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
    let rejected_rule_query = signed_control_for_kind(
        restarted_runtime.clone(),
        constants::lan_pairing::RULE_QUERY_INTENT_ID,
        constants::value::LAN_INTENT_RULE_QUERY,
    )
    .await;
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
    let accepted_rule_query = signed_control_for_kind(
        restarted_runtime.clone(),
        constants::lan_pairing::RULE_QUERY_INTENT_ID,
        constants::value::LAN_INTENT_RULE_QUERY,
    )
    .await;
    let accepted_approval = signed_control_for_kind(
        restarted_runtime,
        constants::lan_pairing::APPROVAL_DECISION_INTENT_ID,
        constants::value::LAN_INTENT_APPROVAL_DECISION,
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
        constants::lan_pairing::RULE_QUERY_INTENT_ID,
        constants::value::LAN_INTENT_RULE_QUERY,
    );
    assert_restart_accepted_intent(
        &accepted_approval,
        constants::lan_pairing::APPROVAL_DECISION_INTENT_ID,
        constants::value::LAN_INTENT_APPROVAL_DECISION,
    );
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

async fn signed_control_for_kind(
    runtime: LanPairingRuntime,
    intent_id: &str,
    intent_kind: &str,
) -> AgentEventEnvelope {
    handle_command_text_for_test(
        &serialize_command(health_command(intent_payload_for_kind(
            intent_id,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
            intent_kind,
        ))),
        runtime,
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await
}

fn assert_restart_accepted_intent(event: &AgentEventEnvelope, intent_id: &str, intent_kind: &str) {
    assert_eq!(event.event, AgentEventName::AgentHealthReported);
    assert_accepted_control_for_intent(event, intent_id);
    assert_eq!(
        event.payload.get(constants::field::LAN_INTENT_KIND),
        Some(&LogFieldValue::String(intent_kind.to_string()))
    );
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
    let mut path = std::env::temp_dir();
    path.push(name);
    path.set_extension(constants::lan_pairing::REGISTRY_FILE_EXTENSION);
    path
}

#[derive(Default)]
struct RecordingMdnsSink {
    packets: Mutex<Vec<Vec<u8>>>,
}

impl RecordingMdnsSink {
    fn packets(&self) -> Vec<Vec<u8>> {
        require_ok(self.packets.lock(), "packets").clone()
    }
}

impl LanMdnsPacketSink for RecordingMdnsSink {
    fn send(&self, packet: &[u8]) -> std::io::Result<()> {
        require_ok(self.packets.lock(), "packets").push(packet.to_vec());
        Ok(())
    }
}

fn packet_contains(packet: &[u8], text: &[u8]) -> bool {
    packet.windows(text.len()).any(|window| window == text)
}

#[test]
fn lan_pairing_runtime_builds_sanitized_mdns_advertisements_and_keeps_hint_only_state() {
    let runtime = LanPairingRuntime::empty();
    let lifecycle = LanPairingRuntime::mdns_advertisement_lifecycle(
        true,
        false,
        ocentra_lan_core::lan_pairing::LanMdnsAdvertisementPlatformSupport::Supported,
    );
    let parent = require_ok(
        runtime.parent_mdns_advertisement(
            "sha256:parent-family-1",
            constants::lan_pairing::SCHEMA_VERSION_TEXT,
            "sha256:family-1",
            ocentra_parent_agent_protocol::lan_pairing::LanMdnsAdvertisementLifecycleState::Start,
            ocentra_parent_agent_protocol::lan_pairing::LanMdnsAdvertisementSupportState::Supported,
        ),
        "parent advertisement",
    );
    let child = require_ok(
        runtime.child_mdns_advertisement(default_child_mdns_advertisement_fixture(
            ocentra_parent_agent_protocol::lan_pairing::LanMdnsAdvertisementLifecycleState::Degraded,
            ocentra_parent_agent_protocol::lan_pairing::LanMdnsAdvertisementSupportState::Degraded,
        )),
        "child advertisement",
    );

    assert_eq!(lifecycle.lifecycle_action.as_str(), "start");
    assert!(lifecycle.hint_only);
    assert_eq!(
        parent.service_type,
        constants::lan_pairing::MDNS_PARENT_SERVICE_TYPE
    );
    assert_eq!(
        child.service_type,
        constants::lan_pairing::MDNS_CHILD_SERVICE_TYPE
    );
    assert_eq!(
        parent.confirmation_state.as_str(),
        constants::lan_pairing::MDNS_TXT_VALUE_HINT_ONLY
    );
    assert_eq!(
        child.confirmation_state.as_str(),
        constants::lan_pairing::MDNS_TXT_VALUE_HINT_ONLY
    );
    assert_eq!(
        parent.pairing_state.as_str(),
        constants::value::LAN_PAIRING_UNPAIRED
    );
    assert_eq!(
        child.pairing_state.as_str(),
        constants::value::LAN_PAIRING_UNPAIRED
    );
    assert!(parent
        .txt_records
        .iter()
        .all(|record| !record.value.contains(' ') && !record.value.contains('@')));
    assert!(child
        .txt_records
        .iter()
        .all(|record| !record.value.contains(' ') && !record.value.contains('@')));
}

#[tokio::test]
async fn lan_pairing_runtime_builds_paired_mdns_advertisements_when_trusted_state_exists() {
    let runtime = paired_runtime().await;
    let parent = require_ok(
        runtime.parent_mdns_advertisement(
            "sha256:parent-family-1",
            constants::lan_pairing::SCHEMA_VERSION_TEXT,
            "sha256:family-1",
            ocentra_parent_agent_protocol::lan_pairing::LanMdnsAdvertisementLifecycleState::Start,
            ocentra_parent_agent_protocol::lan_pairing::LanMdnsAdvertisementSupportState::Supported,
        ),
        "parent advertisement",
    );
    let child = require_ok(
        runtime.child_mdns_advertisement(default_child_mdns_advertisement_fixture(
            ocentra_parent_agent_protocol::lan_pairing::LanMdnsAdvertisementLifecycleState::Update,
            ocentra_parent_agent_protocol::lan_pairing::LanMdnsAdvertisementSupportState::Supported,
        )),
        "child advertisement",
    );

    assert_eq!(
        parent.pairing_state.as_str(),
        constants::value::LAN_PAIRING_PAIRED
    );
    assert_eq!(
        child.pairing_state.as_str(),
        constants::value::LAN_PAIRING_PAIRED
    );
    assert_eq!(
        parent.confirmation_state.as_str(),
        constants::lan_pairing::MDNS_TXT_VALUE_HINT_ONLY
    );
    assert_eq!(
        child.confirmation_state.as_str(),
        constants::lan_pairing::MDNS_TXT_VALUE_HINT_ONLY
    );
}

#[test]
fn unsupported_mdns_platform_reports_degraded_lifecycle() {
    let lifecycle = LanPairingRuntime::mdns_advertisement_lifecycle(
        true,
        true,
        ocentra_lan_core::lan_pairing::LanMdnsAdvertisementPlatformSupport::UnsupportedPlatform,
    );

    assert_eq!(lifecycle.lifecycle_action.as_str(), "degraded");
    assert!(lifecycle.hint_only);
}

#[test]
fn mdns_sync_updates_existing_advertisements_on_subsequent_pass() {
    let runtime = LanPairingRuntime::empty_with_signed_child_agent_context(
        Some("opaque-child-id".to_string()),
        constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
        "sha256:family-1".to_string(),
        constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
    );
    let sink = RecordingMdnsSink::default();
    let mut sync_state = LanMdnsAdvertisementSyncState::default();

    require_ok(
        runtime.sync_mdns_advertisements_with_sink(
            &mut sync_state,
            LanMdnsAdvertisementPlatformSupport::Supported,
            &sink,
        ),
        "initial sync succeeds",
    );
    require_ok(
        runtime.sync_mdns_advertisements_with_sink(
            &mut sync_state,
            LanMdnsAdvertisementPlatformSupport::Supported,
            &sink,
        ),
        "update sync succeeds",
    );

    let packets = sink.packets();

    assert_eq!(packets.len(), 4);
    assert!(packets
        .iter()
        .all(|packet| packet_contains(packet, b"lifecycle-state=start")
            || packet_contains(packet, b"lifecycle-state=update")));
    assert!(packets
        .iter()
        .skip(2)
        .all(|packet| packet_contains(packet, b"lifecycle-state=update")));
    let parent_instance = require_some(sync_state.parent.as_ref(), "parent instance");
    let child_instance = require_some(sync_state.child.as_ref(), "child instance");
    assert_eq!(
        require_some(packets.get(2), "parent update packet"),
        &ocentra_lan_core::lan_mdns_advertiser::encode_advertisement_packet(
            std::slice::from_ref(parent_instance),
            120,
        )
    );
    assert_eq!(
        require_some(packets.get(3), "child update packet"),
        &ocentra_lan_core::lan_mdns_advertiser::encode_advertisement_packet(
            std::slice::from_ref(child_instance),
            120,
        )
    );
}

#[test]
fn mdns_sync_broadcasts_real_parent_and_child_packets_when_runtime_has_context() {
    let runtime = LanPairingRuntime::empty_with_signed_child_agent_context(
        Some("opaque-child-id".to_string()),
        constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
        "sha256:family-1".to_string(),
        constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
    );
    let sink = RecordingMdnsSink::default();
    let mut sync_state = LanMdnsAdvertisementSyncState::default();

    require_ok(
        runtime.sync_mdns_advertisements_with_sink(
            &mut sync_state,
            LanMdnsAdvertisementPlatformSupport::Supported,
            &sink,
        ),
        "mdns sync succeeds",
    );

    let packets = sink.packets();

    assert_eq!(packets.len(), 2);
    assert!(packets
        .iter()
        .any(|packet| { packet_contains(packet, b"_ocentra-parent") }));
    assert!(packets
        .iter()
        .any(|packet| { packet_contains(packet, b"_ocentra-agent") }));
    let parent_instance = require_some(sync_state.parent.as_ref(), "parent instance");
    let child_instance = require_some(sync_state.child.as_ref(), "child instance");
    assert_eq!(
        require_some(packets.first(), "parent advertisement packet"),
        &ocentra_lan_core::lan_mdns_advertiser::encode_advertisement_packet(
            std::slice::from_ref(parent_instance),
            120,
        )
    );
    assert_eq!(
        require_some(packets.get(1), "child advertisement packet"),
        &ocentra_lan_core::lan_mdns_advertiser::encode_advertisement_packet(
            std::slice::from_ref(child_instance),
            120,
        )
    );
}

#[test]
fn mdns_sync_retracts_existing_advertisements_when_platform_support_becomes_degraded() {
    let runtime = LanPairingRuntime::empty_with_signed_child_agent_context(
        Some("opaque-child-id".to_string()),
        constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
        "sha256:family-1".to_string(),
        constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
    );
    let sink = RecordingMdnsSink::default();
    let mut sync_state = LanMdnsAdvertisementSyncState::default();

    require_ok(
        runtime.sync_mdns_advertisements_with_sink(
            &mut sync_state,
            LanMdnsAdvertisementPlatformSupport::Supported,
            &sink,
        ),
        "initial sync succeeds",
    );
    let parent_instance = require_some(sync_state.parent.clone(), "parent instance");
    let child_instance = require_some(sync_state.child.clone(), "child instance");
    require_ok(
        runtime.sync_mdns_advertisements_with_sink(
            &mut sync_state,
            LanMdnsAdvertisementPlatformSupport::Degraded,
            &sink,
        ),
        "degraded sync succeeds",
    );

    let packets = sink.packets();

    assert_eq!(packets.len(), 4);
    assert_eq!(
        require_some(packets.get(2), "parent goodbye"),
        &ocentra_lan_core::lan_mdns_advertiser::encode_advertisement_packet(
            std::slice::from_ref(&parent_instance),
            0,
        )
    );
    assert_eq!(
        require_some(packets.get(3), "child goodbye"),
        &ocentra_lan_core::lan_mdns_advertiser::encode_advertisement_packet(
            std::slice::from_ref(&child_instance),
            0,
        )
    );
    assert!(sync_state.parent.is_none());
    assert!(sync_state.child.is_none());
}

#[test]
fn mdns_sync_sends_goodbye_when_runtime_context_disappears() {
    let runtime = LanPairingRuntime::empty_with_signed_child_agent_context(
        Some("opaque-child-id".to_string()),
        constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
        "sha256:family-1".to_string(),
        constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
    );
    let sink = RecordingMdnsSink::default();
    let mut sync_state = LanMdnsAdvertisementSyncState::default();

    require_ok(
        runtime.sync_mdns_advertisements_with_sink(
            &mut sync_state,
            LanMdnsAdvertisementPlatformSupport::Supported,
            &sink,
        ),
        "initial sync succeeds",
    );
    require_ok(
        LanPairingRuntime::empty().sync_mdns_advertisements_with_sink(
            &mut sync_state,
            LanMdnsAdvertisementPlatformSupport::Supported,
            &sink,
        ),
        "goodbye sync succeeds",
    );

    assert_eq!(sink.packets().len(), 4);
    assert!(sync_state.parent.is_none());
    assert!(sync_state.child.is_none());
}

#[test]
fn mdns_sync_keeps_degraded_platform_manual_without_broadcasting_packets() {
    let runtime = LanPairingRuntime::empty_with_signed_child_agent_context(
        Some("opaque-child-id".to_string()),
        constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
        "sha256:family-1".to_string(),
        constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
    );
    let sink = RecordingMdnsSink::default();
    let mut sync_state = LanMdnsAdvertisementSyncState::default();

    require_ok(
        runtime.sync_mdns_advertisements_with_sink(
            &mut sync_state,
            LanMdnsAdvertisementPlatformSupport::Degraded,
            &sink,
        ),
        "degraded sync succeeds",
    );

    assert!(sink.packets().is_empty());
    assert!(sync_state.parent.is_none());
    assert!(sync_state.child.is_none());
}

#[test]
fn mdns_sync_retracts_stale_state_when_mdns_context_is_invalid() {
    let runtime = LanPairingRuntime::empty_with_signed_child_agent_context(
        Some("opaque-child-id".to_string()),
        constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
        "sha256:family broken".to_string(),
        constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
    );
    let sink = RecordingMdnsSink::default();
    let parent_instance = ocentra_lan_core::lan_mdns_advertiser::LanMdnsAdvertisementInstance {
        service_type: constants::lan_pairing::MDNS_PARENT_SERVICE_TYPE.to_string(),
        instance_name: "parent-stale._ocentra-parent._tcp.local".to_string(),
        txt_records: Vec::new(),
    };
    let child_instance = ocentra_lan_core::lan_mdns_advertiser::LanMdnsAdvertisementInstance {
        service_type: constants::lan_pairing::MDNS_CHILD_SERVICE_TYPE.to_string(),
        instance_name: "child-stale._ocentra-agent._tcp.local".to_string(),
        txt_records: Vec::new(),
    };
    let mut sync_state = LanMdnsAdvertisementSyncState {
        parent: Some(parent_instance.clone()),
        child: Some(child_instance.clone()),
    };

    require_ok(
        runtime.sync_mdns_advertisements_with_sink(
            &mut sync_state,
            LanMdnsAdvertisementPlatformSupport::Supported,
            &sink,
        ),
        "invalid context sync succeeds without advertising",
    );

    assert_eq!(
        sink.packets(),
        vec![
            ocentra_lan_core::lan_mdns_advertiser::encode_advertisement_packet(
                std::slice::from_ref(&parent_instance),
                0,
            ),
            ocentra_lan_core::lan_mdns_advertiser::encode_advertisement_packet(
                std::slice::from_ref(&child_instance),
                0,
            ),
        ]
    );
    assert!(sync_state.parent.is_none());
    assert!(sync_state.child.is_none());
}

#[tokio::test]
async fn lan_pairing_status_surface_reports_live_mdns_support_from_runtime_context() {
    let runtime = LanPairingRuntime::empty_with_signed_child_agent_context(
        Some(constants::lan_pairing::CHILD_DEVICE_ID.to_string()),
        constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
        "sha256:family-1".to_string(),
        constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
    );
    let expected =
        LanPairingRuntime::mdns_advertisement_lifecycle(true, false, current_platform_support());
    let event = pairing_status_event(&runtime, status_command(LogFields::new()));

    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_MDNS_ADVERTISEMENT_LIFECYCLE),
        Some(&LogFieldValue::String(
            expected.lifecycle_action.as_str().to_string()
        ))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_MDNS_ADVERTISEMENT_SUPPORT),
        Some(&LogFieldValue::String(
            expected.platform_support.as_str().to_string()
        ))
    );
}

#[test]
fn lan_pairing_runtime_rejects_sensitive_mdns_txt_atoms() {
    let runtime = LanPairingRuntime::empty();

    assert!(runtime
        .parent_mdns_advertisement(
            "parent family name",
            constants::lan_pairing::SCHEMA_VERSION_TEXT,
            "sha256:family-1",
            ocentra_parent_agent_protocol::lan_pairing::LanMdnsAdvertisementLifecycleState::Start,
            ocentra_parent_agent_protocol::lan_pairing::LanMdnsAdvertisementSupportState::Supported,
        )
        .is_err());
    assert!(runtime
        .child_mdns_advertisement(LanChildMdnsAdvertisementFixture {
            opaque_device_id: "child@example.com",
            ..default_child_mdns_advertisement_fixture(
                ocentra_parent_agent_protocol::lan_pairing::LanMdnsAdvertisementLifecycleState::Start,
                ocentra_parent_agent_protocol::lan_pairing::LanMdnsAdvertisementSupportState::Supported,
            )
        })
        .is_err());
    assert!(runtime
        .child_mdns_advertisement(LanChildMdnsAdvertisementFixture {
            platform: "Windows Laptop",
            ..default_child_mdns_advertisement_fixture(
                ocentra_parent_agent_protocol::lan_pairing::LanMdnsAdvertisementLifecycleState::Start,
                ocentra_parent_agent_protocol::lan_pairing::LanMdnsAdvertisementSupportState::Supported,
            )
        })
        .is_err());
}

#[test]
fn lan_pairing_runtime_rejects_invalid_mdns_protocol_versions() {
    let runtime = LanPairingRuntime::empty();

    assert!(runtime
        .parent_mdns_advertisement(
            "sha256:parent-family-1",
            " ",
            "sha256:family-1",
            ocentra_parent_agent_protocol::lan_pairing::LanMdnsAdvertisementLifecycleState::Start,
            ocentra_parent_agent_protocol::lan_pairing::LanMdnsAdvertisementSupportState::Supported,
        )
        .is_err());
    assert!(runtime
        .child_mdns_advertisement(LanChildMdnsAdvertisementFixture {
            protocol_version: " ",
            ..default_child_mdns_advertisement_fixture(
                ocentra_parent_agent_protocol::lan_pairing::LanMdnsAdvertisementLifecycleState::Start,
                ocentra_parent_agent_protocol::lan_pairing::LanMdnsAdvertisementSupportState::Supported,
            )
        })
        .is_err());
}

#[test]
fn lan_pairing_runtime_accepts_signed_child_agent_hello_and_heartbeat_with_real_signatures() {
    let runtime = LanPairingRuntime::empty();
    let hello = signed_child_agent_envelope(
        LanSignedChildAgentMessageKind::Hello,
        "nonce-service-hello-1",
        1,
        "2026-06-26T10:05:00Z",
    );
    let heartbeat = signed_child_agent_envelope(
        LanSignedChildAgentMessageKind::Heartbeat,
        "nonce-service-heartbeat-1",
        2,
        "2026-06-26T10:05:00Z",
    );

    let verified_hello = require_ok(
        runtime.verify_signed_child_agent_envelope(
            &hello,
            "2026-06-26T10:00:30Z",
            &signed_child_agent_context(Some(constants::lan_pairing::CHILD_DEVICE_ID)),
        ),
        "signed hello verifies through runtime replay guard",
    );
    let verified_heartbeat = require_ok(
        runtime.verify_signed_child_agent_envelope(
            &heartbeat,
            "2026-06-26T10:00:31Z",
            &signed_child_agent_context(Some(constants::lan_pairing::CHILD_DEVICE_ID)),
        ),
        "signed heartbeat verifies through runtime replay guard",
    );

    assert_eq!(
        verified_hello.message_kind,
        LanSignedChildAgentMessageKind::Hello
    );
    assert_eq!(verified_hello.install_id, "child-install-1");
    assert_eq!(verified_hello.family_hash, "sha256:family-1");
    assert_eq!(
        verified_hello.child_profile_hash.as_deref(),
        Some("sha256:child-profile-1")
    );
    assert_eq!(
        verified_hello.platform,
        constants::lan_pairing::PLATFORM_WINDOWS
    );
    assert_eq!(
        verified_hello.hostname,
        constants::lan_pairing::TEST_HOSTNAME
    );
    assert_eq!(verified_hello.agent_version, "1.2.3");
    assert_eq!(
        verified_hello.local_ips,
        vec![constants::lan_pairing::TEST_LAN_IP.to_string()]
    );
    assert_eq!(
        verified_hello.mac_addresses,
        vec![constants::lan_pairing::TEST_LAN_MAC.to_string()]
    );
    assert_eq!(
        verified_hello.capabilities,
        vec![
            constants::lan_pairing::CHILD_AGENT_CAPABILITY_PAIRING_ROUTE.to_string(),
            "future-safe-local-capability".to_string(),
        ]
    );
    assert_eq!(verified_hello.nonce, "nonce-service-hello-1");
    assert_eq!(
        verified_heartbeat.message_kind,
        LanSignedChildAgentMessageKind::Heartbeat
    );
    assert_eq!(verified_heartbeat.install_id, "child-install-1");
    assert_eq!(verified_heartbeat.nonce, "nonce-service-heartbeat-1");
    assert_eq!(
        verified_heartbeat.capabilities,
        vec![
            constants::lan_pairing::CHILD_AGENT_CAPABILITY_PAIRING_ROUTE.to_string(),
            "future-safe-local-capability".to_string(),
        ]
    );
    assert_eq!(runtime.signed_child_agent_replay_observation_count(), 2);
    assert_eq!(
        runtime.verify_signed_child_agent_envelope(
            &heartbeat,
            "2026-06-26T10:00:32Z",
            &signed_child_agent_context(Some(constants::lan_pairing::CHILD_DEVICE_ID,)),
        ),
        Err(ocentra_lan_core::lan_pairing::LanSignedChildAgentVerificationError::Replayed)
    );
}

#[test]
fn lan_pairing_runtime_rejects_invalid_signature_wrong_family_and_expired_signed_child_agent_envelopes(
) {
    let runtime = LanPairingRuntime::empty();
    let context = signed_child_agent_context(Some(constants::lan_pairing::CHILD_DEVICE_ID));
    let mut invalid_signature = signed_child_agent_envelope(
        LanSignedChildAgentMessageKind::Hello,
        "nonce-service-invalid-signature-1",
        11,
        "2026-06-26T10:05:00Z",
    );
    invalid_signature.claim.nonce = "tampered-nonce".to_string();
    let mut wrong_family = signed_child_agent_envelope(
        LanSignedChildAgentMessageKind::Heartbeat,
        "nonce-service-wrong-family-1",
        12,
        "2026-06-26T10:05:00Z",
    );
    wrong_family = signed_child_agent_envelope_with_claim({
        let mut claim = wrong_family.claim;
        claim.family_hash = "sha256:family-2".to_string();
        claim
    });
    let mut expired = signed_child_agent_envelope(
        LanSignedChildAgentMessageKind::Hello,
        "nonce-service-expired-1",
        13,
        "2026-06-26T09:55:00Z",
    );
    expired.claim.expires_at = "2026-06-26T09:54:59Z".to_string();

    assert_eq!(
        runtime.verify_signed_child_agent_envelope(
            &invalid_signature,
            "2026-06-26T10:00:30Z",
            &context,
        ),
        Err(ocentra_lan_core::lan_pairing::LanSignedChildAgentVerificationError::SignatureRejected)
    );
    assert_eq!(
        runtime
            .verify_signed_child_agent_envelope(&wrong_family, "2026-06-26T10:00:31Z", &context,),
        Err(ocentra_lan_core::lan_pairing::LanSignedChildAgentVerificationError::WrongFamily)
    );
    assert_eq!(
        runtime.verify_signed_child_agent_envelope(&expired, "2026-06-26T10:00:32Z", &context,),
        Err(ocentra_lan_core::lan_pairing::LanSignedChildAgentVerificationError::Expired)
    );
}

#[test]
fn signed_child_agent_observation_records_passive_beacon_history_rows() {
    let runtime = LanPairingRuntime::empty_with_signed_child_agent_context(
        Some(constants::lan_pairing::CHILD_DEVICE_ID.to_string()),
        constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
        "sha256:family-1".to_string(),
        constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
    );
    let hello = signed_child_agent_envelope(
        LanSignedChildAgentMessageKind::Hello,
        "nonce-passive-hello-1",
        71,
        "2026-06-26T10:05:00Z",
    );
    let heartbeat = signed_child_agent_envelope(
        LanSignedChildAgentMessageKind::Heartbeat,
        "nonce-passive-heartbeat-1",
        72,
        "2026-06-26T10:05:10Z",
    );

    require_ok(
        runtime.observe_signed_child_agent_envelope(&hello, "2026-06-26T10:00:30Z"),
        "hello observation",
    );
    require_ok(
        runtime.observe_signed_child_agent_envelope(&heartbeat, "2026-06-26T10:00:31Z"),
        "heartbeat observation",
    );

    let snapshot = runtime.passive_discovery_history_snapshot();
    assert_eq!(snapshot.rows.len(), 2);
    assert_eq!(
        snapshot.rows[0].source,
        Some(LanPassiveDiscoverySource::OcentraBeacon)
    );
    assert_eq!(
        snapshot.rows[0].trigger_reason,
        LanPassiveDiscoveryTriggerReason::PassivePacketObserved
    );
    assert_eq!(
        snapshot.rows[0].device_id.as_deref(),
        Some(constants::lan_pairing::CHILD_DEVICE_ID)
    );
    assert_eq!(
        snapshot.rows[1].source,
        Some(LanPassiveDiscoverySource::OcentraBeacon)
    );
    assert_eq!(
        snapshot.rows[1].trigger_reason,
        LanPassiveDiscoveryTriggerReason::PassivePacketObserved
    );
    assert_eq!(
        snapshot.rows[1].device_id.as_deref(),
        Some(constants::lan_pairing::CHILD_DEVICE_ID)
    );
    assert_eq!(
        snapshot.rows[0].summary,
        format!(
            "signed child hello observed: route={}; install-id=child-install-1",
            constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK
        )
    );
    assert_eq!(
        snapshot.rows[1].summary,
        format!(
            "signed child heartbeat observed: route={}; install-id=child-install-1",
            constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK
        )
    );
}

#[tokio::test]
async fn lan_pairing_signed_child_agent_observe_command_verifies_and_reports() {
    let runtime = LanPairingRuntime::empty_with_signed_child_agent_context(
        Some(constants::lan_pairing::CHILD_DEVICE_ID.to_string()),
        constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
        "sha256:family-1".to_string(),
        constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
    );
    let issued_at =
        (Utc::now() - Duration::seconds(1)).to_rfc3339_opts(SecondsFormat::Millis, true);
    let expires_at =
        (Utc::now() + Duration::minutes(5)).to_rfc3339_opts(SecondsFormat::Millis, true);
    let envelope = signed_child_agent_envelope_with_window(
        LanSignedChildAgentMessageKind::Hello,
        "nonce-service-command-1",
        31,
        &issued_at,
        &expires_at,
    );

    let event = handle_command_text_for_test(
        &serialize_command(command_for_target(
            AgentCommandName::AgentLanPairingSignedChildAgentObserve,
            local_network_target(constants::lan_pairing::CHILD_DEVICE_ID),
            signed_child_agent_payload(&envelope),
        )),
        runtime.clone(),
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;
    let replay = handle_command_text_for_test(
        &serialize_command(command_for_target(
            AgentCommandName::AgentLanPairingSignedChildAgentObserve,
            local_network_target(constants::lan_pairing::CHILD_DEVICE_ID),
            signed_child_agent_payload(&envelope),
        )),
        runtime.clone(),
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;

    assert_eq!(
        event.event,
        AgentEventName::AgentLanPairingSignedChildAgentReported
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_SIGNED_CHILD_AGENT_VERIFICATION),
        Some(&LogFieldValue::String(
            constants::value::LAN_SIGNED_CHILD_AGENT_VERIFICATION_ACCEPTED.to_string()
        ))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_SIGNED_CHILD_AGENT_MESSAGE_KIND),
        Some(&LogFieldValue::String("hello".to_string()))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_SIGNED_CHILD_AGENT_REPLAY_OBSERVED_COUNT),
        Some(&LogFieldValue::Number(1.0))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_SIGNED_CHILD_AGENT_STATUS),
        Some(&LogFieldValue::String(
            constants::lan_pairing::PRODUCTION_PROOF_STATE_MANUAL_REQUIRED.to_string()
        ))
    );
    assert_rejection(&replay, constants::value::LAN_REASON_REPLAYED);
}

#[tokio::test]
async fn lan_pairing_signed_child_agent_observe_rejects_when_parent_context_is_unconfigured() {
    let runtime = LanPairingRuntime::empty();
    let issued_at =
        (Utc::now() - Duration::seconds(1)).to_rfc3339_opts(SecondsFormat::Millis, true);
    let expires_at =
        (Utc::now() + Duration::minutes(5)).to_rfc3339_opts(SecondsFormat::Millis, true);
    let envelope = signed_child_agent_envelope_with_window(
        LanSignedChildAgentMessageKind::Hello,
        "nonce-service-command-no-context-1",
        32,
        &issued_at,
        &expires_at,
    );

    let event = handle_command_text_for_test(
        &serialize_command(command_for_target(
            AgentCommandName::AgentLanPairingSignedChildAgentObserve,
            local_network_target(constants::lan_pairing::CHILD_DEVICE_ID),
            signed_child_agent_payload(&envelope),
        )),
        runtime,
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;

    assert_rejection(
        &event,
        constants::value::LAN_REASON_SIGNED_CHILD_AGENT_CONTEXT_UNAVAILABLE,
    );
}

#[tokio::test]
async fn lan_pairing_signed_child_agent_observe_rejects_when_child_context_is_unpaired() {
    let runtime = LanPairingRuntime::empty_with_signed_child_agent_context(
        None,
        constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
        "sha256:family-1".to_string(),
        constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
    );
    let issued_at =
        (Utc::now() - Duration::seconds(1)).to_rfc3339_opts(SecondsFormat::Millis, true);
    let expires_at =
        (Utc::now() + Duration::minutes(5)).to_rfc3339_opts(SecondsFormat::Millis, true);
    let envelope = signed_child_agent_envelope_with_window(
        LanSignedChildAgentMessageKind::Hello,
        "nonce-service-command-missing-child-1",
        33,
        &issued_at,
        &expires_at,
    );

    let event = handle_command_text_for_test(
        &serialize_command(command_for_target(
            AgentCommandName::AgentLanPairingSignedChildAgentObserve,
            local_network_target(constants::lan_pairing::CHILD_DEVICE_ID),
            signed_child_agent_payload(&envelope),
        )),
        runtime,
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;

    assert_rejection(
        &event,
        constants::value::LAN_REASON_SIGNED_CHILD_AGENT_CONTEXT_UNAVAILABLE,
    );
}

#[test]
fn lan_pairing_runtime_rejects_malformed_signed_child_agent_envelope() {
    let runtime = LanPairingRuntime::empty();
    let envelope = LanSignedChildAgentEnvelope {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        claim: LanSignedChildAgentClaim {
            schema_version: constants::lan_pairing::SCHEMA_VERSION,
            message_kind: LanSignedChildAgentMessageKind::Hello,
            child_device_id: constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
            parent_device_id: constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
            install_id: "child-install-1".to_string(),
            family_hash: "sha256:family-1".to_string(),
            child_profile_hash: Some("sha256:child-profile-1".to_string()),
            platform: constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
            hostname: constants::lan_pairing::TEST_HOSTNAME.to_string(),
            agent_version: "1.2.3".to_string(),
            local_ips: vec![constants::lan_pairing::TEST_LAN_IP.to_string()],
            mac_addresses: vec![constants::lan_pairing::TEST_LAN_MAC.to_string()],
            capabilities: vec![
                constants::lan_pairing::CHILD_AGENT_CAPABILITY_PAIRING_ROUTE.to_string()
            ],
            route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
            nonce: "nonce-service-wrapper-1".to_string(),
            sequence: 1,
            issued_at: "2026-06-26T10:00:00Z".to_string(),
            expires_at: "2026-06-26T10:05:00Z".to_string(),
        },
        public_key_base64: "not-base64".to_string(),
        public_key_id: "bad-key".to_string(),
        signature_base64: "not-base64".to_string(),
        signature_algorithm: constants::lan_pairing::SIGNED_CHILD_AGENT_SIGNATURE_ALGORITHM_ED25519
            .to_string(),
    };
    let context = LanSignedChildAgentVerificationContext {
        expected_parent_device_id: constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
        expected_family_hash: "sha256:family-1".to_string(),
        expected_route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        expected_child_device_id: Some(constants::lan_pairing::CHILD_DEVICE_ID.to_string()),
    };

    assert_eq!(
        runtime.verify_signed_child_agent_envelope(&envelope, "2026-06-26T10:00:30Z", &context,),
        Err(ocentra_lan_core::lan_pairing::LanSignedChildAgentVerificationError::InvalidPublicKey)
    );
}

#[test]
fn lan_pairing_runtime_rejects_signed_child_agent_wrong_parent_wrong_route_empty_nonce_and_schema_version(
) {
    let runtime = LanPairingRuntime::empty_with_signed_child_agent_context(
        Some(constants::lan_pairing::CHILD_DEVICE_ID.to_string()),
        constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
        "sha256:family-1".to_string(),
        constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
    );
    let observed_at = "2026-06-26T10:00:30Z";

    assert_eq!(
        runtime.observe_signed_child_agent_envelope(
            &signed_child_agent_envelope_with_claim({
                let mut claim = signed_child_agent_envelope(
                    LanSignedChildAgentMessageKind::Hello,
                    "nonce-runtime-empty-1",
                    41,
                    "2026-06-26T10:05:00Z",
                )
                .claim;
                claim.nonce = String::new();
                claim
            }),
            observed_at,
        ),
        Err(LanPairingRejectionReason::Malformed)
    );

    assert_eq!(
        runtime.observe_signed_child_agent_envelope(
            &signed_child_agent_envelope_with_claim({
                let mut claim = signed_child_agent_envelope(
                    LanSignedChildAgentMessageKind::Heartbeat,
                    "nonce-runtime-parent-1",
                    42,
                    "2026-06-26T10:05:00Z",
                )
                .claim;
                claim.parent_device_id = "sha256:other-parent".to_string();
                claim
            }),
            observed_at,
        ),
        Err(LanPairingRejectionReason::WrongDevice)
    );

    assert_eq!(
        runtime.observe_signed_child_agent_envelope(
            &signed_child_agent_envelope_with_claim({
                let mut claim = signed_child_agent_envelope(
                    LanSignedChildAgentMessageKind::Hello,
                    "nonce-runtime-route-1",
                    43,
                    "2026-06-26T10:05:00Z",
                )
                .claim;
                claim.route_id = constants::lan_pairing::ROUTE_ID_SECOND_LOCAL_NETWORK.to_string();
                claim
            }),
            observed_at,
        ),
        Err(LanPairingRejectionReason::UnsupportedRoute)
    );

    assert_eq!(
        runtime.observe_signed_child_agent_envelope(
            &signed_child_agent_envelope_with_claim({
                let mut claim = signed_child_agent_envelope(
                    LanSignedChildAgentMessageKind::Heartbeat,
                    "nonce-runtime-schema-1",
                    44,
                    "2026-06-26T10:05:00Z",
                )
                .claim;
                claim.schema_version = constants::lan_pairing::SCHEMA_VERSION + 1;
                claim
            }),
            observed_at,
        ),
        Err(LanPairingRejectionReason::Malformed)
    );
}

fn signed_child_agent_context(
    expected_child_device_id: Option<&str>,
) -> LanSignedChildAgentVerificationContext {
    LanSignedChildAgentVerificationContext {
        expected_parent_device_id: constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
        expected_family_hash: "sha256:family-1".to_string(),
        expected_route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        expected_child_device_id: expected_child_device_id.map(str::to_string),
    }
}

fn signed_child_agent_envelope(
    message_kind: LanSignedChildAgentMessageKind,
    nonce: &str,
    sequence: u64,
    expires_at: &str,
) -> LanSignedChildAgentEnvelope {
    signed_child_agent_envelope_with_window(
        message_kind,
        nonce,
        sequence,
        "2026-06-26T10:00:00Z",
        expires_at,
    )
}

fn signed_child_agent_envelope_with_window(
    message_kind: LanSignedChildAgentMessageKind,
    nonce: &str,
    sequence: u64,
    issued_at: &str,
    expires_at: &str,
) -> LanSignedChildAgentEnvelope {
    let signing_key = SigningKey::from_bytes(&[7_u8; 32]);
    let verifying_key = signing_key.verifying_key();
    let claim = LanSignedChildAgentClaim {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        message_kind,
        child_device_id: constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
        parent_device_id: constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
        install_id: "child-install-1".to_string(),
        family_hash: "sha256:family-1".to_string(),
        child_profile_hash: Some("sha256:child-profile-1".to_string()),
        platform: constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
        hostname: constants::lan_pairing::TEST_HOSTNAME.to_string(),
        agent_version: "1.2.3".to_string(),
        local_ips: vec![constants::lan_pairing::TEST_LAN_IP.to_string()],
        mac_addresses: vec![constants::lan_pairing::TEST_LAN_MAC.to_string()],
        capabilities: vec![
            constants::lan_pairing::CHILD_AGENT_CAPABILITY_PAIRING_ROUTE.to_string(),
            "future-safe-local-capability".to_string(),
        ],
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        nonce: nonce.to_string(),
        sequence,
        issued_at: issued_at.to_string(),
        expires_at: expires_at.to_string(),
    };
    let payload = serde_json::to_vec(&claim)
        .unwrap_or_else(|_| unreachable!("signed child claim serializes"));
    let signature = signing_key.sign(&payload);

    LanSignedChildAgentEnvelope {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        claim,
        public_key_base64: STANDARD.encode(verifying_key.as_bytes()),
        public_key_id: ocentra_lan_core::lan_pairing::signed_child_agent_public_key_id(
            &verifying_key,
        ),
        signature_base64: STANDARD.encode(signature.to_bytes()),
        signature_algorithm: constants::lan_pairing::SIGNED_CHILD_AGENT_SIGNATURE_ALGORITHM_ED25519
            .to_string(),
    }
}

fn signed_child_agent_envelope_with_claim(
    claim: LanSignedChildAgentClaim,
) -> LanSignedChildAgentEnvelope {
    let signing_key = SigningKey::from_bytes(&[7_u8; 32]);
    let verifying_key = signing_key.verifying_key();
    let payload = require_ok(serde_json::to_vec(&claim), "signed child claim serializes");
    let signature = signing_key.sign(&payload);

    LanSignedChildAgentEnvelope {
        schema_version: claim.schema_version,
        claim,
        public_key_base64: STANDARD.encode(verifying_key.as_bytes()),
        public_key_id: ocentra_lan_core::lan_pairing::signed_child_agent_public_key_id(
            &verifying_key,
        ),
        signature_base64: STANDARD.encode(signature.to_bytes()),
        signature_algorithm: constants::lan_pairing::SIGNED_CHILD_AGENT_SIGNATURE_ALGORITHM_ED25519
            .to_string(),
    }
}

fn signed_child_agent_payload(envelope: &LanSignedChildAgentEnvelope) -> LogFields {
    let mut fields = LogFields::new();
    fields.insert(
        constants::field::LAN_SIGNED_CHILD_AGENT_ENVELOPE_JSON.to_string(),
        LogFieldValue::String(
            serde_json::to_string(envelope)
                .unwrap_or_else(|_| unreachable!("signed child envelope serializes")),
        ),
    );
    fields
}
