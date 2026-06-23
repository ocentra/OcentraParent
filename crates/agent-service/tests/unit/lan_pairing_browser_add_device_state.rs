use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::policy_constants;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandName, AgentEventName, AgentMessageTarget, AgentRoute,
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
    assert_empty_runtime_payload(&event.payload);
    let read_model = read_model_payload(&event.payload);
    assert_empty_runtime_read_model(&read_model);
}

fn assert_empty_runtime_payload(payload: &ocentra_parent_agent_protocol::logging::LogFields) {
    assert_eq!(
        payload.get(constants::field::LAN_DISCOVERY_SOURCE),
        Some(&LogFieldValue::String(
            constants::value::LAN_DISCOVERY_SOURCE_LOCAL_SERVICE.to_string()
        ))
    );
    let physical_lan_state = payload.get(constants::field::LAN_PHYSICAL_HOUSEHOLD_LAN_STATE);
    assert!(matches!(
        physical_lan_state,
        Some(LogFieldValue::String(value))
            if value == constants::value::LAN_DISCOVERY_STATE_MANUAL_REQUIRED
                || value == constants::value::LAN_DISCOVERY_STATE_DISCOVERED
    ));
    assert_eq!(
        payload.get(constants::field::LAN_CLOUD_RELAY_STATE),
        Some(&LogFieldValue::String(
            constants::value::LAN_DISCOVERY_STATE_UNAVAILABLE.to_string()
        ))
    );
    assert_eq!(
        payload.get(constants::field::LAN_SELECTED_DEVICE_READY),
        Some(&LogFieldValue::Boolean(false))
    );
}

fn assert_empty_runtime_read_model(read_model: &Value) {
    assert_eq!(
        read_model[constants::field::LAN_ADD_DEVICE_STATE],
        serde_json::json!(constants::value::LAN_DISCOVERY_STATE_DISCOVERED)
    );
    let canonical_devices = read_model[constants::field::LAN_CANONICAL_HOUSEHOLD_DEVICES]
        .as_array()
        .unwrap_or_else(|| unreachable!("{}", constants::value::LAN_READ_MODEL_JSON_EXPECTATION));
    assert!(canonical_devices.iter().any(|device| device
        [constants::field::LAN_CANONICAL_DEVICE_ID]
        .as_str()
        .map(|value| !value.is_empty())
        .unwrap_or(false)));
    assert_eq!(
        read_model[constants::field::LAN_TRUSTED_DEVICE_REGISTRY],
        serde_json::json!([])
    );
    assert!(read_model[constants::field::LAN_HONEST_NON_CLAIMS]
        .as_array()
        .unwrap_or_else(|| {
            unreachable!(
                "{}",
                constants::value::LAN_HONEST_NON_CLAIMS_ARRAY_EXPECTATION
            )
        })
        .iter()
        .any(|claim| {
            claim.as_str() == Some(constants::value::LAN_NON_CLAIM_REMOTE_DESKTOP_NOT_IMPLEMENTED)
        }));
    assert!(
        read_model[constants::field::LAN_SCAN_SUMMARY][constants::field::SOURCE_LABELS]
            .as_array()
            .unwrap_or_else(|| {
                unreachable!(
                    "{}",
                    constants::value::LAN_HONEST_NON_CLAIMS_ARRAY_EXPECTATION
                )
            })
            .iter()
            .any(|source| {
                source.as_str() == Some(constants::lan_pairing::LAN_SCAN_SOURCE_LOCAL_SERVICE)
            })
    );
    assert_empty_runtime_production_household_proof(read_model);
    assert_empty_runtime_signed_discovery_relay_spine(read_model);
    assert_empty_runtime_lan_source_matrix(read_model);
}

fn assert_empty_runtime_production_household_proof(read_model: &Value) {
    let production_household_proof =
        &read_model[constants::lan_pairing::PRODUCTION_PROOF_FIELD_SUMMARY];
    assert_eq!(
        production_household_proof[constants::lan_pairing::PRODUCTION_PROOF_FIELD_STATUS_ROWS][0]
            [constants::field::CAPABILITY],
        serde_json::json!(constants::lan_pairing::PRODUCTION_PROOF_CAPABILITY_SIGNED_HELLO)
    );
    assert_eq!(
        production_household_proof[constants::lan_pairing::PRODUCTION_PROOF_FIELD_STATUS_ROWS][0]
            [constants::lan_pairing::PRODUCTION_PROOF_FIELD_PROOF_STATE],
        serde_json::json!(constants::lan_pairing::PRODUCTION_PROOF_STATE_MANUAL_REQUIRED)
    );
    assert_eq!(
        production_household_proof[constants::lan_pairing::PRODUCTION_PROOF_FIELD_NOT_IMPLEMENTED],
        serde_json::json!([
            constants::lan_pairing::PRODUCTION_PROOF_CAPABILITY_RELAY_ROUTE,
            constants::lan_pairing::PRODUCTION_PROOF_CAPABILITY_CACHE_ROUTE
        ])
    );
    assert!(production_household_proof
        [constants::lan_pairing::PRODUCTION_PROOF_FIELD_CLAIMS_NOT_PROVED]
        .as_array()
        .unwrap_or_else(|| {
            unreachable!(
                "{}",
                constants::value::LAN_HONEST_NON_CLAIMS_ARRAY_EXPECTATION
            )
        })
        .iter()
        .any(|claim| claim.as_str()
            == Some(constants::lan_pairing::PRODUCTION_PROOF_NON_CLAIM_SIGNED)));
}

fn assert_empty_runtime_signed_discovery_relay_spine(read_model: &Value) {
    let signed_discovery_relay_spine =
        &read_model[constants::lan_pairing::SIGNED_DISCOVERY_RELAY_FIELD_SUMMARY];
    assert_eq!(
        signed_discovery_relay_spine
            [constants::lan_pairing::SIGNED_DISCOVERY_RELAY_FIELD_ADAPTER_ROWS][6]
            [constants::lan_pairing::SIGNED_DISCOVERY_RELAY_FIELD_ADAPTER],
        serde_json::json!(
            constants::lan_pairing::SIGNED_DISCOVERY_RELAY_ADAPTER_SIGNED_CHILD_AGENT_HELLO
        )
    );
    assert_eq!(
        signed_discovery_relay_spine
            [constants::lan_pairing::SIGNED_DISCOVERY_RELAY_FIELD_SIGNED_PROOF_ROWS][3]
            [constants::lan_pairing::SIGNED_DISCOVERY_RELAY_FIELD_REJECTION_REASON],
        serde_json::json!(constants::value::LAN_REASON_ANONYMOUS)
    );
    assert_eq!(
        signed_discovery_relay_spine
            [constants::lan_pairing::SIGNED_DISCOVERY_RELAY_FIELD_RELAY_CACHE_ROWS][4]
            [constants::lan_pairing::SIGNED_DISCOVERY_RELAY_FIELD_CUSTODY_LABEL],
        serde_json::json!(constants::lan_pairing::SIGNED_DISCOVERY_RELAY_CUSTODY_NO_CHILD_DATA)
    );
}

fn assert_empty_runtime_lan_source_matrix(read_model: &Value) {
    let matrix = &read_model[constants::lan_pairing::LAN_SOURCE_MATRIX_FIELD_SUMMARY];
    assert_eq!(
        matrix[constants::lan_pairing::LAN_SOURCE_MATRIX_FIELD_WORKPACK_ROWS]
            .as_array()
            .unwrap_or_else(|| unreachable!(
                "{}",
                constants::value::LAN_READ_MODEL_JSON_EXPECTATION
            ))
            .len(),
        20
    );
    assert_eq!(
        matrix[constants::lan_pairing::LAN_SOURCE_MATRIX_FIELD_WORKPACK_ROWS][17]
            [constants::lan_pairing::LAN_SOURCE_MATRIX_FIELD_WORKPACK_ID],
        serde_json::json!(constants::lan_pairing::LAN_SOURCE_MATRIX_WORKPACK_ID_SIGNED_CHILD_HELLO)
    );
    assert_eq!(
        matrix[constants::lan_pairing::LAN_SOURCE_MATRIX_FIELD_SOURCE_ROWS][0]
            [constants::lan_pairing::LAN_SOURCE_MATRIX_FIELD_CAN_CONFIRM],
        serde_json::json!(false)
    );
    assert!(
        matrix[constants::lan_pairing::PRODUCTION_PROOF_FIELD_CLAIMS_NOT_PROVED]
            .as_array()
            .unwrap_or_else(|| {
                unreachable!(
                    "{}",
                    constants::value::LAN_HONEST_NON_CLAIMS_ARRAY_EXPECTATION
                )
            })
            .iter()
            .any(|claim| claim.as_str()
                == Some(constants::lan_pairing::LAN_SOURCE_MATRIX_NON_CLAIM_PACKET_MODE))
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
    assert_paired_production_route_custody(&read_model);
    assert_paired_signed_route_custody(&read_model);
    assert_eq!(
        read_model[constants::field::LAN_CANONICAL_HOUSEHOLD_DEVICES][0]
            [constants::field::LAN_POLICY_TARGET_SURFACES],
        serde_json::json!([
            constants::lan_pairing::SURFACE_DEVICES,
            constants::lan_pairing::SURFACE_POLICY,
            constants::lan_pairing::SURFACE_BROWSER,
            constants::lan_pairing::SURFACE_APP,
            constants::lan_pairing::SURFACE_SCREEN,
            constants::lan_pairing::SURFACE_NETWORK,
            constants::lan_pairing::SURFACE_ACTIVITY,
            constants::lan_pairing::SURFACE_TRACKING,
            constants::lan_pairing::SURFACE_AI
        ])
    );
}

fn assert_paired_production_route_custody(read_model: &Value) {
    assert_eq!(
        read_model[constants::lan_pairing::PRODUCTION_PROOF_FIELD_SUMMARY]
            [constants::lan_pairing::PRODUCTION_PROOF_FIELD_STATUS_ROWS]
            .as_array()
            .unwrap_or_else(|| {
                unreachable!(
                    "{}",
                    constants::value::LAN_HONEST_NON_CLAIMS_ARRAY_EXPECTATION
                )
            })
            .iter()
            .find(|row| row[constants::field::CAPABILITY]
                == serde_json::json!(
                    constants::lan_pairing::PRODUCTION_PROOF_CAPABILITY_ROUTE_CUSTODY
                ))
            .unwrap_or_else(|| unreachable!(
                "{}",
                constants::value::LAN_READ_MODEL_JSON_EXPECTATION
            ))[constants::field::LAN_DISCOVERY_STATE],
        serde_json::json!(constants::value::LAN_DISCOVERY_STATE_PAIRED)
    );
}

fn assert_paired_signed_route_custody(read_model: &Value) {
    assert_eq!(
        read_model[constants::lan_pairing::SIGNED_DISCOVERY_RELAY_FIELD_SUMMARY]
            [constants::lan_pairing::SIGNED_DISCOVERY_RELAY_FIELD_ROUTE_SAFETY_ROWS]
            .as_array()
            .unwrap_or_else(|| {
                unreachable!(
                    "{}",
                    constants::value::LAN_HONEST_NON_CLAIMS_ARRAY_EXPECTATION
                )
            })
            .iter()
            .find(
                |row| row[constants::lan_pairing::SIGNED_DISCOVERY_RELAY_FIELD_CHECK]
                    == serde_json::json!(
                        constants::lan_pairing::SIGNED_DISCOVERY_RELAY_ROUTE_CHECK_SELECTED_CUSTODY
                    )
            )
            .unwrap_or_else(|| unreachable!(
                "{}",
                constants::value::LAN_READ_MODEL_JSON_EXPECTATION
            ))[constants::field::LAN_DISCOVERY_STATE],
        serde_json::json!(constants::value::LAN_DISCOVERY_STATE_PAIRED)
    );
}

fn read_model_payload(payload: &ocentra_parent_agent_protocol::logging::LogFields) -> Value {
    match payload.get(constants::field::LAN_ADD_DEVICE_READ_MODEL) {
        Some(LogFieldValue::String(value)) => serde_json::from_str(value).unwrap_or_else(|error| {
            unreachable!(
                "{}: {error:?}",
                constants::value::LAN_READ_MODEL_JSON_EXPECTATION
            )
        }),
        _ => serde_json::json!({}),
    }
}

fn loopback_status_command() -> ocentra_parent_agent_protocol::transport::AgentCommandEnvelope {
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
