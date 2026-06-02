use ocentra_parent_agent_protocol::{
    constants, policy_constants, AgentCommandName, AgentEventName, AgentMessageTarget, AgentRoute,
    LogFieldValue, LogFields,
};
use serde_json::Value;

use crate::{
    lan_pairing::LanPairingRuntime,
    lan_pairing_test_commands::{command_for_target, paired_runtime, serialize_command},
    websocket::handle_command_text_for_test,
};

#[tokio::test]
async fn lan_status_reports_browser_first_add_device_read_model_from_service_state() {
    let event = handle_command_text_for_test(
        &serialize_command(loopback_status_command()),
        LanPairingRuntime::empty(),
        None,
    )
    .await;

    assert_eq!(event.event, AgentEventName::AgentLanPairingStatusReported);
    assert_eq!(
        event.payload.get(constants::field::LAN_DISCOVERY_SOURCE),
        Some(&LogFieldValue::String(
            constants::value::LAN_DISCOVERY_SOURCE_LOCAL_SERVICE.to_string()
        ))
    );
    let physical_lan_state = event
        .payload
        .get(constants::field::LAN_PHYSICAL_HOUSEHOLD_LAN_STATE);
    assert!(matches!(
        physical_lan_state,
        Some(LogFieldValue::String(value))
            if value == constants::value::LAN_DISCOVERY_STATE_MANUAL_REQUIRED
                || value == constants::value::LAN_DISCOVERY_STATE_DISCOVERED
    ));
    assert_eq!(
        event.payload.get(constants::field::LAN_CLOUD_RELAY_STATE),
        Some(&LogFieldValue::String(
            constants::value::LAN_DISCOVERY_STATE_UNAVAILABLE.to_string()
        ))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_SELECTED_DEVICE_READY),
        Some(&LogFieldValue::Boolean(false))
    );
    let read_model = read_model_payload(&event.payload);
    assert_eq!(
        read_model[constants::field::LAN_ADD_DEVICE_STATE],
        serde_json::json!(constants::value::LAN_DISCOVERY_STATE_DISCOVERED)
    );
    assert_eq!(
        read_model[constants::field::LAN_TRUSTED_DEVICE_REGISTRY],
        serde_json::json!([])
    );
    assert!(read_model[constants::field::LAN_HONEST_NON_CLAIMS]
        .as_array()
        .expect(constants::value::LAN_HONEST_NON_CLAIMS_ARRAY_EXPECTATION)
        .iter()
        .any(|claim| {
            claim.as_str() == Some(constants::value::LAN_NON_CLAIM_REMOTE_DESKTOP_NOT_IMPLEMENTED)
        }));
    assert!(
        read_model[constants::field::LAN_SCAN_SUMMARY][constants::field::SOURCE_LABELS]
            .as_array()
            .expect(constants::value::LAN_HONEST_NON_CLAIMS_ARRAY_EXPECTATION)
            .iter()
            .any(|source| {
                source.as_str() == Some(constants::lan_pairing::LAN_SCAN_SOURCE_LOCAL_SERVICE)
            })
    );
}

#[tokio::test]
async fn lan_status_marks_selected_trusted_device_ready_for_control() {
    let event = handle_command_text_for_test(
        &serialize_command(loopback_status_command()),
        paired_runtime().await,
        None,
    )
    .await;

    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_SELECTED_DEVICE_READY),
        Some(&LogFieldValue::Boolean(true))
    );
    assert_eq!(
        event.payload.get(constants::field::LAN_ADD_DEVICE_STATE),
        Some(&LogFieldValue::String(
            constants::value::LAN_DISCOVERY_STATE_PAIRED.to_string()
        ))
    );
    let read_model = read_model_payload(&event.payload);
    assert_eq!(
        read_model[constants::field::LAN_TRUSTED_DEVICE_REGISTRY][0]
            [constants::field::LAN_PAIRING_ID],
        serde_json::json!(constants::lan_pairing::PAIRING_ID)
    );
    assert_eq!(
        read_model[constants::field::LAN_TRUSTED_DEVICE_REGISTRY][0]
            [constants::field::LAN_CHILD_DEVICE][constants::field::DEVICE_ID],
        serde_json::json!(constants::lan_pairing::CHILD_DEVICE_ID)
    );
    assert_eq!(
        read_model[constants::field::LAN_SELECTED_DEVICE_READINESS]
            [constants::field::LAN_READY_FOR_CONTROL],
        serde_json::json!(true)
    );
}

fn read_model_payload(payload: &ocentra_parent_agent_protocol::LogFields) -> Value {
    match payload.get(constants::field::LAN_ADD_DEVICE_READ_MODEL) {
        Some(LogFieldValue::String(value)) => {
            serde_json::from_str(value).expect(constants::value::LAN_READ_MODEL_JSON_EXPECTATION)
        }
        _ => serde_json::json!({}),
    }
}

fn loopback_status_command() -> ocentra_parent_agent_protocol::AgentCommandEnvelope {
    command_for_target(
        AgentCommandName::AgentLanPairingStatusGet,
        AgentMessageTarget {
            device_id: constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
            platform: policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS.to_string(),
            route: AgentRoute::Localhost,
        },
        LogFields::new(),
    )
}
