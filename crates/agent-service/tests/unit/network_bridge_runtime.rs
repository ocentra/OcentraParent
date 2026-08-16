extern crate ocentra_parent_agent_service as agent_service_lib;
extern crate self as ocentra_parent_agent_service;

use chrono::{DateTime, SecondsFormat};

#[path = "../support/test_invariants.rs"]
mod test_invariants;
#[path = "../support/test_text.rs"]
mod test_text;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::policy_constants;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentMessageTarget, AgentRoute,
};
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;
use test_invariants::require_ok;

#[path = "../support/network_bridge_test_support.rs"]
pub mod test_support;

#[path = "../../src/activity_network_flow_payload.rs"]
mod activity_network_flow_payload;
#[path = "../support/activity_report_env_lock.rs"]
mod activity_report_env_lock;
#[path = "../../src/event_builder.rs"]
mod event_builder;
#[path = "../../src/fields.rs"]
mod fields;
#[path = "../../src/network_android_vpn_service_gate_status_bridge.rs"]
mod network_android_vpn_service_gate_status_bridge;
#[path = "../../src/network_apple_network_extension_gate_status_bridge.rs"]
mod network_apple_network_extension_gate_status_bridge;
#[path = "../../src/network_flow_digest.rs"]
mod network_flow_digest;
#[path = "../../src/network_flow_digest_indicators.rs"]
mod network_flow_digest_indicators;
#[path = "../../src/network_flow_digest_rollups.rs"]
mod network_flow_digest_rollups;
#[path = "../../src/network_linux_nftables_lab_status_bridge.rs"]
mod network_linux_nftables_lab_status_bridge;
#[path = "../../src/network_live_capture_execution_bridge.rs"]
mod network_live_capture_execution_bridge;
#[path = "../../src/network_live_capture_readiness_bridge.rs"]
mod network_live_capture_readiness_bridge;
#[path = "../../src/network_remote_delivery_status_cross_process.rs"]
mod network_remote_delivery_status_cross_process;
#[path = "../../src/network_remote_delivery_status_payload.rs"]
mod network_remote_delivery_status_payload;
#[path = "../../src/network_runtime_delivery.rs"]
mod network_runtime_delivery;
#[path = "../../src/network_runtime_stream_event_payloads.rs"]
mod network_runtime_stream_event_payloads;
#[path = "../../src/network_runtime_stream_event_values.rs"]
mod network_runtime_stream_event_values;
#[path = "../../src/network_runtime_stream_events.rs"]
mod network_runtime_stream_events;
#[path = "../../src/network_runtime_stream_payload.rs"]
mod network_runtime_stream_payload;
#[path = "../../src/network_windows_firewall_lab_status_bridge.rs"]
mod network_windows_firewall_lab_status_bridge;
#[path = "../../src/network_windows_wfp_gate_status_bridge.rs"]
mod network_windows_wfp_gate_status_bridge;
#[path = "../../src/time.rs"]
mod time;

#[path = "network_android_vpn_service_gate_status_bridge_tests.rs"]
mod network_android_vpn_service_gate_status_bridge_tests;
#[path = "network_apple_network_extension_gate_status_bridge_tests.rs"]
mod network_apple_network_extension_gate_status_bridge_tests;
#[path = "network_flow_digest_tests.rs"]
mod network_flow_digest_tests;
#[path = "network_flow_payload_tests.rs"]
mod network_flow_payload_tests;
#[path = "network_linux_nftables_lab_status_bridge_tests.rs"]
mod network_linux_nftables_lab_status_bridge_tests;
#[path = "network_live_capture_readiness_bridge_tests.rs"]
mod network_live_capture_readiness_bridge_tests;
#[path = "network_remote_delivery_status_service_tests.rs"]
mod network_remote_delivery_status_service_tests;
#[path = "network_runtime_delivery_tests.rs"]
mod network_runtime_delivery_tests;
#[path = "network_runtime_stream_tests.rs"]
mod network_runtime_stream_tests;
#[path = "network_windows_firewall_lab_status_bridge_tests.rs"]
mod network_windows_firewall_lab_status_bridge_tests;
#[path = "network_windows_wfp_gate_status_bridge_tests.rs"]
mod network_windows_wfp_gate_status_bridge_tests;

#[test]
fn network_bridge_runtime_links_report_builders_and_time_helpers() {
    let command = command_envelope();

    let android = network_android_vpn_service_gate_status_bridge::build_network_android_vpn_service_gate_status_report(
        command.clone(),
    );
    assert_eq!(
        android.event,
        ocentra_parent_agent_protocol::transport::AgentEventName::AgentNetworkAndroidVpnServiceGateStatusReported
    );

    let apple = network_apple_network_extension_gate_status_bridge::build_network_apple_network_extension_gate_status_report(
        command.clone(),
    );
    assert_eq!(
        apple.event,
        ocentra_parent_agent_protocol::transport::AgentEventName::AgentNetworkAppleNetworkExtensionGateStatusReported
    );

    let linux =
        network_linux_nftables_lab_status_bridge::build_network_linux_nftables_lab_status_report(
            command.clone(),
        );
    assert_eq!(
        linux.event,
        ocentra_parent_agent_protocol::transport::AgentEventName::AgentNetworkLinuxNftablesLabStatusReported
    );

    let live_capture =
        network_live_capture_readiness_bridge::build_network_live_capture_status_report(
            command.clone(),
        );
    assert_eq!(
        live_capture.event,
        ocentra_parent_agent_protocol::transport::AgentEventName::AgentNetworkLiveCaptureStatusReported
    );

    let firewall = network_windows_firewall_lab_status_bridge::build_network_windows_firewall_lab_status_report(
        command.clone(),
    );
    assert_eq!(
        firewall.event,
        ocentra_parent_agent_protocol::transport::AgentEventName::AgentNetworkWindowsFirewallLabStatusReported
    );

    let wfp = network_windows_wfp_gate_status_bridge::build_network_windows_wfp_gate_status_report(
        command,
    );
    assert_eq!(
        wfp.event,
        ocentra_parent_agent_protocol::transport::AgentEventName::AgentNetworkWindowsWfpGateStatusReported
    );

    let runtime_report =
        network_runtime_stream_payload::NetworkRuntimeServiceStreamReport::default();
    let runtime_payload =
        network_runtime_stream_payload::network_runtime_event_chain_stream_payload(&runtime_report);
    assert_eq!(
        runtime_payload.get(constants::field::NETWORK_RUNTIME_EVENT_CHAIN_STREAM),
        Some(&ocentra_parent_agent_protocol::logging::LogFieldValue::String("[]".to_string()))
    );

    let timestamp_now: String = time::timestamp_now();
    let timestamp_from_epoch: String = time::timestamp_from_epoch_seconds(0);
    let timestamp_after_epoch: String = time::timestamp_after_epoch_seconds(0, 1);
    let parsed_timestamp_now = require_ok(
        DateTime::parse_from_rfc3339(&timestamp_now),
        "timestamp_now must use RFC3339 formatting",
    );
    assert_eq!(
        parsed_timestamp_now.to_rfc3339_opts(SecondsFormat::Millis, true),
        timestamp_now
    );
    assert_eq!(timestamp_from_epoch, "1970-01-01T00:00:00.000Z");
    assert_eq!(timestamp_after_epoch, "1970-01-01T00:00:01.000Z");
    assert_eq!(
        event_builder::portal_peer().peer_id,
        constants::peer::PORTAL_DEV
    );
}

#[tokio::test]
async fn network_bridge_runtime_links_remote_delivery_report_builder() {
    let event =
        network_remote_delivery_status_payload::build_network_remote_delivery_status_report(
            command_envelope(),
        )
        .await;

    assert_eq!(
        event.event,
        ocentra_parent_agent_protocol::transport::AgentEventName::AgentNetworkRemoteDeliveryStatusReported
    );
}

#[test]
fn network_bridge_runtime_links_test_invariant_helpers() {
    let decoded: serde_json::Value =
        test_invariants::require_json_decode("{\"timestamp\":\"ok\"}", "json decodes");
    assert_eq!(decoded["timestamp"], "ok");

    assert_eq!(
        test_invariants::require_some(Some("linked"), "option links"),
        "linked"
    );

    let mut payload = LogFields::new();
    payload.insert(
        constants::field::NETWORK_RUNTIME_EVENT_CHAIN_STREAM.to_string(),
        ocentra_parent_agent_protocol::logging::LogFieldValue::String("[]".to_string()),
    );
    assert_eq!(
        test_invariants::require_log_string_field(
            Some(&test_invariants::log_field(
                &payload,
                constants::field::NETWORK_RUNTIME_EVENT_CHAIN_STREAM,
                "log field links",
            )),
            "string log field links",
        ),
        "[]"
    );
}

fn command_envelope() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::event_id::NETWORK_RUNTIME_EVENT_CHAIN_STREAM_REPORTED.to_string(),
        sent_at: time::timestamp_from_epoch_seconds(0),
        source: event_builder::portal_peer(),
        target: AgentMessageTarget {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform: policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS.to_string(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentNetworkRuntimeEventChainStreamGet,
        payload: LogFields::new(),
    }
}
