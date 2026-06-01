use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentCommandName, AgentEventName,
    LanPairingProductionDiscoveryState, LogFieldValue, LogFields,
};
use serde_json::Value;

use crate::{
    fields::fields_from_pairs,
    lan_pairing::LanPairingRuntime,
    lan_pairing_test_commands::{command_for_target, local_network_target},
    lan_pairing_test_support::{paired_runtime, serialize_command},
    websocket::handle_command_text_for_test,
};

#[tokio::test]
async fn browser_discovery_scan_reports_real_local_service_state() {
    let event = handle_command_text_for_test(
        &serialize_command(browser_discovery_scan_command(LogFields::new())),
        LanPairingRuntime::empty(),
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;

    assert_eq!(
        event.event,
        AgentEventName::AgentLanPairingBrowserDiscoveryReported
    );
    assert_eq!(
        event.payload.get(constants::field::LAN_DISCOVERY_SOURCE),
        Some(&LogFieldValue::String(
            constants::value::LAN_DISCOVERY_SOURCE_LOCAL_SERVICE.to_string()
        ))
    );
    let read_model = read_model_payload(&event.payload);
    assert_eq!(
        read_model[constants::field::LAN_ADD_DEVICE_STATE],
        serde_json::json!(constants::value::LAN_DISCOVERY_STATE_DISCOVERED)
    );
    assert_eq!(
        read_model[constants::field::LAN_PHYSICAL_HOUSEHOLD_LAN_STATE],
        serde_json::json!(constants::value::LAN_DISCOVERY_STATE_MANUAL_REQUIRED)
    );
    assert_eq!(
        read_model[constants::field::LAN_CLOUD_RELAY_STATE],
        serde_json::json!(constants::value::LAN_DISCOVERY_STATE_UNAVAILABLE)
    );
    assert_eq!(
        read_model[constants::field::LAN_DISCOVERED_DEVICES][0][constants::field::LAN_CHILD_DEVICE]
            [constants::field::DEVICE_ID],
        serde_json::json!(constants::lan_pairing::CHILD_DEVICE_ID)
    );
}

#[tokio::test]
async fn add_device_request_issues_pending_challenge_event() {
    let event = handle_command_text_for_test(
        &serialize_command(add_device_request_command(challenge_request_payload())),
        LanPairingRuntime::empty(),
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;

    assert_eq!(
        event.event,
        AgentEventName::AgentLanPairingAddDeviceReported
    );
    assert_eq!(
        event.payload.get(constants::field::LAN_ADD_DEVICE_STATE),
        Some(&LogFieldValue::String(
            constants::value::LAN_DISCOVERY_STATE_PENDING.to_string()
        ))
    );
    let read_model = read_model_payload(&event.payload);
    assert_eq!(
        read_model[constants::field::LAN_ADD_DEVICE_STATE],
        serde_json::json!(constants::value::LAN_DISCOVERY_STATE_PENDING)
    );
    assert_eq!(
        read_model[constants::field::LAN_PAIRING_REQUESTS][0][constants::field::LAN_PAIRING_STATE],
        serde_json::json!(LanPairingProductionDiscoveryState::Pending)
    );
    assert_eq!(
        read_model[constants::field::LAN_PAIRING_REQUESTS][0]
            [constants::field::LAN_CHILD_DEVICE_ID],
        serde_json::json!(constants::lan_pairing::CHILD_DEVICE_ID)
    );
}

#[tokio::test]
async fn add_device_request_rejects_wrong_origin_without_trusting_device() {
    let event = handle_command_text_for_test(
        &serialize_command(add_device_request_command(challenge_request_payload())),
        LanPairingRuntime::empty(),
        Some(constants::lan_pairing::WRONG_ORIGIN.to_string()),
    )
    .await;

    assert_eq!(event.event, AgentEventName::AgentCommandRejected);
    assert_eq!(
        event.payload.get(constants::field::LAN_REJECTION_REASON),
        Some(&LogFieldValue::String(
            constants::value::LAN_REASON_WRONG_ORIGIN.to_string()
        ))
    );
}

#[tokio::test]
async fn paired_runtime_scan_exposes_registry_and_selected_readiness() {
    let event = handle_command_text_for_test(
        &serialize_command(browser_discovery_scan_command(LogFields::new())),
        paired_runtime().await,
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;

    assert_eq!(
        event.event,
        AgentEventName::AgentLanPairingBrowserDiscoveryReported
    );
    let read_model = read_model_payload(&event.payload);
    assert_eq!(
        read_model[constants::field::LAN_TRUSTED_DEVICE_REGISTRY][0]
            [constants::field::LAN_PAIRING_ID],
        serde_json::json!(constants::lan_pairing::PAIRING_ID)
    );
    assert_eq!(
        read_model[constants::field::LAN_SELECTED_DEVICE_READINESS]
            [constants::field::LAN_READY_FOR_CONTROL],
        serde_json::json!(true)
    );
}

fn read_model_payload(payload: &LogFields) -> Value {
    match payload.get(constants::field::LAN_ADD_DEVICE_READ_MODEL) {
        Some(LogFieldValue::String(value)) => {
            serde_json::from_str(value).expect(constants::value::LAN_READ_MODEL_JSON_EXPECTATION)
        }
        _ => serde_json::json!({}),
    }
}

fn browser_discovery_scan_command(payload: LogFields) -> AgentCommandEnvelope {
    command_for_target(
        AgentCommandName::AgentLanPairingBrowserDiscoveryScan,
        local_network_target(constants::lan_pairing::CHILD_DEVICE_ID),
        payload,
    )
}

fn add_device_request_command(payload: LogFields) -> AgentCommandEnvelope {
    command_for_target(
        AgentCommandName::AgentLanPairingAddDeviceRequest,
        local_network_target(constants::lan_pairing::CHILD_DEVICE_ID),
        payload,
    )
}

fn challenge_request_payload() -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::LAN_CHILD_DEVICE_ID,
            LogFieldValue::String(constants::lan_pairing::CHILD_DEVICE_ID.to_string()),
        ),
        (
            constants::field::LAN_PARENT_DEVICE_ID,
            LogFieldValue::String(constants::lan_pairing::PARENT_DEVICE_ID.to_string()),
        ),
        (
            constants::field::LAN_ROUTE_ID,
            LogFieldValue::String(constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string()),
        ),
        (
            constants::field::ORIGIN,
            LogFieldValue::String(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
        ),
        (
            constants::field::STARTED_AT,
            LogFieldValue::String(constants::lan_pairing::ISSUED_AT.to_string()),
        ),
        (
            constants::field::STALE_AT,
            LogFieldValue::String(constants::lan_pairing::EXPIRES_AT.to_string()),
        ),
    ])
}
