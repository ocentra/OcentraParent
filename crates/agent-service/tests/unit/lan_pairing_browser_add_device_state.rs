use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingProductionDiscoveryState, LanPairingTrustState,
};
use ocentra_parent_agent_protocol::lan_pairing_authority::LanPairingParentAuthority;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanPairingDiscoverySource;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanSelectedDeviceReadiness;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::policy_constants;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandName, AgentEventName, AgentMessageTarget, AgentRoute,
};
use serde_json::Value;
use std::string::String as TestString;

use ocentra_lan_core::read_model_builder::{
    build_lan_add_device_read_model, LanAddDeviceReadModelInput,
};

use crate::{
    app::{lan_pairing::LanPairingRuntime, websocket::handle_command_text_for_test},
    lan_pairing_test_commands::{command_for_target, serialize_command},
    test_require_json_decode::require_json_decode,
    test_require_ok::require_ok,
    test_require_some::require_some,
};

#[tokio::test]
async fn lan_status_reports_browser_first_add_device_read_model_from_service_state() {
    let event = handle_command_text_for_test(
        serialize_command(loopback_status_command()),
        LanPairingRuntime::empty(),
        None,
    )
    .await;

    assert_eq!(event.event, AgentEventName::AgentLanPairingStatusReported);
    assert_empty_runtime_payload(&event.payload);
    let read_model = read_model_payload(&event.payload);
    assert_empty_runtime_read_model(&read_model);
}

#[test]
fn unavailable_platform_data_marks_the_read_model_unavailable() {
    let model = build_lan_add_device_read_model(LanAddDeviceReadModelInput {
        generated_at: "2026-06-23T00:00:00Z".to_string(),
        discovery_source: LanPairingDiscoverySource::LocalService,
        service_data_available: false,
        platform_data_available: false,
        add_device_state: LanPairingProductionDiscoveryState::ManualRequired,
        local_service_discovery_state: LanPairingProductionDiscoveryState::ManualRequired,
        physical_household_lan_state: LanPairingProductionDiscoveryState::ManualRequired,
        cloud_relay_state: LanPairingProductionDiscoveryState::Unavailable,
        discovered_devices: Vec::new(),
        pairing_requests: Vec::new(),
        trusted_device_registry: Vec::new(),
        household_device_decisions: Vec::new(),
        trusted_device_ids: Vec::new(),
        revoked_device_ids: Vec::new(),
        selected_device_readiness: LanSelectedDeviceReadiness {
            schema_version: constants::lan_pairing::SCHEMA_VERSION,
            selected_child_device_id: None,
            route_id: None,
            pairing_id: None,
            trust_state: LanPairingTrustState::Unpaired,
            reachability: LanPairingDeviceReachability::Offline,
            ready_for_control: false,
            stale_at: None,
            offline_at: None,
        },
        controller_authority: LanPairingParentAuthority::ActiveController,
        observer_authority: LanPairingParentAuthority::Observer,
    });

    assert_eq!(
        model.add_device_state,
        LanPairingProductionDiscoveryState::Unavailable
    );
    assert_eq!(
        model.local_service_discovery_state,
        LanPairingProductionDiscoveryState::Unavailable
    );
    assert_eq!(
        model.physical_household_lan_state,
        LanPairingProductionDiscoveryState::Unavailable
    );
    assert_eq!(
        model.discovery_event_history.state,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEventHistoryState::Unavailable
    );
}

#[test]
fn service_data_available_keeps_pairing_state_when_platform_scan_is_unavailable() {
    let model = build_lan_add_device_read_model(LanAddDeviceReadModelInput {
        generated_at: "2026-06-23T00:00:00Z".to_string(),
        discovery_source: LanPairingDiscoverySource::LocalService,
        service_data_available: true,
        platform_data_available: false,
        add_device_state: LanPairingProductionDiscoveryState::Paired,
        local_service_discovery_state: LanPairingProductionDiscoveryState::Paired,
        physical_household_lan_state: LanPairingProductionDiscoveryState::ManualRequired,
        cloud_relay_state: LanPairingProductionDiscoveryState::Unavailable,
        discovered_devices: Vec::new(),
        pairing_requests: Vec::new(),
        trusted_device_registry: Vec::new(),
        household_device_decisions: Vec::new(),
        trusted_device_ids: Vec::new(),
        revoked_device_ids: Vec::new(),
        selected_device_readiness: LanSelectedDeviceReadiness {
            schema_version: constants::lan_pairing::SCHEMA_VERSION,
            selected_child_device_id: None,
            route_id: None,
            pairing_id: None,
            trust_state: LanPairingTrustState::Paired,
            reachability: LanPairingDeviceReachability::Online,
            ready_for_control: true,
            stale_at: None,
            offline_at: None,
        },
        controller_authority: LanPairingParentAuthority::ActiveController,
        observer_authority: LanPairingParentAuthority::Observer,
    });

    assert_eq!(
        model.add_device_state,
        LanPairingProductionDiscoveryState::Paired
    );
    assert_eq!(
        model.local_service_discovery_state,
        LanPairingProductionDiscoveryState::Paired
    );
    assert_eq!(
        model.physical_household_lan_state,
        LanPairingProductionDiscoveryState::Unavailable
    );
}

fn assert_empty_runtime_payload(payload: &ocentra_parent_agent_protocol::logging::LogFields) {
    let discovery_source = payload.get(constants::field::LAN_DISCOVERY_SOURCE);
    assert!(
        matches!(discovery_source, Some(LogFieldValue::String(value))
        if value.as_str()
            == serialized_discovery_source(LanPairingDiscoverySource::LocalService).as_str()
            || value.as_str()
                == serialized_discovery_source(LanPairingDiscoverySource::PhysicalHouseholdLan)
                    .as_str())
    );
    let physical_lan_state = payload.get(constants::field::LAN_PHYSICAL_HOUSEHOLD_LAN_STATE);
    assert!(matches!(
        physical_lan_state,
        Some(LogFieldValue::String(value))
            if value == constants::value::LAN_DISCOVERY_STATE_MANUAL_REQUIRED
                || value == constants::value::LAN_DISCOVERY_STATE_DISCOVERED
                || value == constants::value::LAN_DISCOVERY_STATE_UNAVAILABLE
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
        serde_json::json!(constants::value::LAN_DISCOVERY_STATE_UNAVAILABLE)
    );
    assert_eq!(
        read_model[constants::field::LAN_TRUSTED_DEVICE_REGISTRY],
        serde_json::json!([])
    );
    assert_eq!(
        read_model[constants::field::LAN_SELECTED_DEVICE_READINESS]
            [constants::field::LAN_READY_FOR_CONTROL],
        serde_json::json!(false)
    );
    assert!(require_some(
        read_model[constants::field::LAN_HONEST_NON_CLAIMS]
            .as_array()
            .map(|claims| claims.as_slice()),
        constants::value::LAN_READ_MODEL_JSON_EXPECTATION,
    )
    .iter()
    .any(|claim| {
        claim.as_str() == Some(constants::value::LAN_NON_CLAIM_REMOTE_DESKTOP_NOT_IMPLEMENTED)
    }));
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
    assert!(
        require_some(
            production_household_proof
                [constants::lan_pairing::PRODUCTION_PROOF_FIELD_CLAIMS_NOT_PROVED]
                .as_array()
                .map(|claims| claims.as_slice()),
            constants::value::LAN_READ_MODEL_JSON_EXPECTATION,
        )
        .iter()
        .any(|claim| claim.as_str()
            == Some(constants::lan_pairing::PRODUCTION_PROOF_NON_CLAIM_SIGNED))
    );
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
    let workpack_rows = require_some(
        matrix[constants::lan_pairing::LAN_SOURCE_MATRIX_FIELD_WORKPACK_ROWS]
            .as_array()
            .map(|rows| rows.as_slice()),
        constants::value::LAN_READ_MODEL_JSON_EXPECTATION,
    );
    assert!(workpack_rows.len() >= 20);
    assert!(workpack_rows.iter().any(|row| {
        row[constants::lan_pairing::LAN_SOURCE_MATRIX_FIELD_WORKPACK_ID]
            == serde_json::json!(
                constants::lan_pairing::LAN_SOURCE_MATRIX_WORKPACK_ID_SIGNED_CHILD_HELLO
            )
    }));
    assert_eq!(
        matrix[constants::lan_pairing::LAN_SOURCE_MATRIX_FIELD_SOURCE_ROWS][0]
            [constants::lan_pairing::LAN_SOURCE_MATRIX_FIELD_CAN_CONFIRM],
        serde_json::json!(false)
    );
    assert!(require_some(
        matrix[constants::lan_pairing::PRODUCTION_PROOF_FIELD_CLAIMS_NOT_PROVED]
            .as_array()
            .map(|claims| claims.as_slice()),
        constants::value::LAN_READ_MODEL_JSON_EXPECTATION,
    )
    .iter()
    .any(|claim| claim.as_str()
        == Some(constants::lan_pairing::LAN_SOURCE_MATRIX_NON_CLAIM_PACKET_MODE)));
}

fn read_model_payload(payload: &ocentra_parent_agent_protocol::logging::LogFields) -> Value {
    match payload.get(constants::field::LAN_ADD_DEVICE_READ_MODEL) {
        Some(LogFieldValue::String(value)) => {
            require_json_decode(value, constants::value::LAN_READ_MODEL_JSON_EXPECTATION)
        }
        _ => serde_json::json!({}),
    }
}

fn serialized_discovery_source(source: LanPairingDiscoverySource) -> TestString {
    let value = require_ok(
        serde_json::to_value(source),
        constants::value::LAN_READ_MODEL_JSON_EXPECTATION,
    );
    require_some(
        value.as_str().map(ToOwned::to_owned),
        constants::value::LAN_READ_MODEL_JSON_EXPECTATION,
    )
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
