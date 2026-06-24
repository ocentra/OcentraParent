use std::fs::{read_to_string, remove_file};
use std::time::{SystemTime, UNIX_EPOCH};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingNetworkMode, LanPairingProductionDiscoveryState,
    LanPairingTrustState,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanBrowserAddDeviceReadModel, LanCanonicalHouseholdDevice,
    LanCanonicalHouseholdDeviceClassification, LanCanonicalHouseholdDeviceConfidence,
    LanCanonicalHouseholdDeviceSource, LanCanonicalHouseholdNetworkIdentity,
    LanCanonicalHouseholdRouteState, LanCanonicalHouseholdSurface, LanPairingDiscoverySource,
};
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventName,
};
use serde_json::Value;

use crate::{
    fields::fields_from_pairs,
    lan_pairing::LanPairingRuntime,
    lan_pairing_test_commands::{
        command_for_target, local_network_target, paired_runtime, serialize_command,
    },
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
        event
            .payload
            .get(constants::field::LAN_DISCOVERY_SOURCE)
            .is_some(),
        true
    );
    assert!(matches!(
        event.payload.get(constants::field::LAN_DISCOVERY_SOURCE),
        Some(LogFieldValue::String(value))
            if value.as_str()
                == serialized_discovery_source(LanPairingDiscoverySource::LocalService).as_str()
                || value.as_str()
                    == serialized_discovery_source(
                    LanPairingDiscoverySource::PhysicalHouseholdLan
                )
                .as_str()
    ));
    let read_model = read_model_payload(&event.payload);
    assert_eq!(
        read_model[constants::field::LAN_ADD_DEVICE_STATE],
        serde_json::json!(constants::value::LAN_DISCOVERY_STATE_DISCOVERED)
    );
    assert!(
        read_model[constants::field::LAN_PHYSICAL_HOUSEHOLD_LAN_STATE]
            == serde_json::json!(constants::value::LAN_DISCOVERY_STATE_MANUAL_REQUIRED)
            || read_model[constants::field::LAN_PHYSICAL_HOUSEHOLD_LAN_STATE]
                == serde_json::json!(constants::value::LAN_DISCOVERY_STATE_DISCOVERED)
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
async fn add_device_request_persists_household_decision_in_read_model() {
    let event = handle_command_text_for_test(
        &serialize_command(add_device_request_command(household_decision_payload())),
        LanPairingRuntime::empty(),
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;

    assert_eq!(
        event.event,
        AgentEventName::AgentLanPairingAddDeviceReported
    );
    let read_model = read_model_payload(&event.payload);
    assert_eq!(
        read_model[constants::lan_pairing::REGISTRY_KEY_HOUSEHOLD_DEVICE_DECISIONS][0]
            [constants::lan_pairing::HOUSEHOLD_DECISION_ACTION_KIND_FIELD],
        serde_json::json!(constants::lan_pairing::HOUSEHOLD_ACTION_RENAME)
    );
    assert_eq!(
        read_model[constants::lan_pairing::REGISTRY_KEY_HOUSEHOLD_DEVICE_DECISIONS][0]
            [constants::lan_pairing::HOUSEHOLD_ACTION_DEVICE_KIND_FIELD],
        serde_json::json!(constants::lan_pairing::HOUSEHOLD_DEVICE_KIND_DESKTOP)
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

#[tokio::test]
async fn persistent_runtime_scan_persists_known_household_device_store() {
    let registry_path = temp_registry_path();
    let _ = remove_file(&registry_path);
    let runtime = LanPairingRuntime::persistent_json(&registry_path);
    let event = handle_command_text_for_test(
        &serialize_command(browser_discovery_scan_command(LogFields::new())),
        runtime,
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;

    assert_eq!(
        event.event,
        AgentEventName::AgentLanPairingBrowserDiscoveryReported
    );
    let registry_json = read_to_string(&registry_path).unwrap_or_default();
    let registry_value: Value = serde_json::from_str(&registry_json)
        .unwrap_or_else(|error| unreachable!("persistent registry json parses: {error:?}"));
    assert!(
        registry_value[constants::lan_pairing::REGISTRY_KEY_KNOWN_HOUSEHOLD_DEVICES]
            .as_array()
            .map(|devices| !devices.is_empty())
            .unwrap_or(false)
    );

    let _ = remove_file(&registry_path);
}

#[tokio::test]
async fn persistent_runtime_restores_known_household_device_into_read_model_as_stale() {
    let registry_path = temp_registry_path();
    let _ = remove_file(&registry_path);
    let runtime = LanPairingRuntime::persistent_json(&registry_path);
    {
        let mut registry = runtime
            .registry
            .lock()
            .unwrap_or_else(|_| unreachable!("registry lock available for test"));
        let changed = registry.merge_known_household_devices(vec![stored_known_router()]);
        assert!(changed);
        assert!(runtime.persist_registry(&registry));
    }

    let event = handle_command_text_for_test(
        &serialize_command(browser_discovery_scan_command(LogFields::new())),
        LanPairingRuntime::persistent_json(&registry_path),
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;

    assert_eq!(
        event.event,
        AgentEventName::AgentLanPairingBrowserDiscoveryReported
    );
    let read_model = typed_read_model_payload(&event.payload);
    let device = read_model
        .canonical_household_devices
        .iter()
        .find(|device| {
            device.canonical_device_id
                == canonical_device_id_for_mac(constants::lan_pairing::TEST_ROUTER_MAC)
        })
        .unwrap_or_else(|| unreachable!("stored known router restored into read model"));

    assert_eq!(
        device.discovery_state,
        LanPairingProductionDiscoveryState::Stale
    );
    assert_eq!(
        device.network_identity.reachability,
        LanPairingDeviceReachability::Stale
    );
    assert!(device.network_identity.stale_at.is_some());

    let _ = remove_file(&registry_path);
}

fn read_model_payload(payload: &LogFields) -> Value {
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

fn typed_read_model_payload(payload: &LogFields) -> LanBrowserAddDeviceReadModel {
    match payload.get(constants::field::LAN_ADD_DEVICE_READ_MODEL) {
        Some(LogFieldValue::String(value)) => serde_json::from_str(value).unwrap_or_else(|error| {
            unreachable!(
                "{}: {error:?}",
                constants::value::LAN_READ_MODEL_JSON_EXPECTATION
            )
        }),
        _ => unreachable!("{}", constants::value::LAN_READ_MODEL_JSON_EXPECTATION),
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

fn household_decision_payload() -> LogFields {
    fields_from_pairs(vec![
        (
            constants::lan_pairing::HOUSEHOLD_ACTION_ID_FIELD,
            LogFieldValue::String(constants::lan_pairing::HOUSEHOLD_ACTION_ID.to_string()),
        ),
        (
            constants::lan_pairing::HOUSEHOLD_ACTION_KIND_FIELD,
            LogFieldValue::String(constants::lan_pairing::HOUSEHOLD_ACTION_RENAME.to_string()),
        ),
        (
            constants::field::LAN_CANONICAL_DEVICE_ID,
            LogFieldValue::String(local_agent_canonical_device_id()),
        ),
        (
            constants::lan_pairing::HOUSEHOLD_ACTION_DISPLAY_NAME_FIELD,
            LogFieldValue::String(
                constants::lan_pairing::HOUSEHOLD_RENAMED_DEVICE_LABEL.to_string(),
            ),
        ),
        (
            constants::lan_pairing::HOUSEHOLD_ACTION_DEVICE_KIND_FIELD,
            LogFieldValue::String(
                constants::lan_pairing::HOUSEHOLD_DEVICE_KIND_DESKTOP.to_string(),
            ),
        ),
        (
            constants::field::LAN_PARENT_ACTOR_ID,
            LogFieldValue::String(constants::lan_pairing::PARENT_ACTOR_ID.to_string()),
        ),
        (
            constants::field::STARTED_AT,
            LogFieldValue::String(constants::lan_pairing::OBSERVED_AT.to_string()),
        ),
    ])
}

fn local_agent_canonical_device_id() -> String {
    let mut id = String::from(constants::lan_pairing::CANONICAL_DEVICE_ID_PREFIX);
    id.push_str(&compact_identifier(constants::lan_pairing::CHILD_DEVICE_ID));
    id
}

fn compact_identifier(value: &str) -> String {
    value
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

fn serialized_discovery_source(source: LanPairingDiscoverySource) -> String {
    serde_json::to_value(source)
        .ok()
        .and_then(|value| value.as_str().map(ToOwned::to_owned))
        .unwrap_or_default()
}

fn temp_registry_path() -> std::path::PathBuf {
    let unique_id = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    std::env::temp_dir().join(format!(
        "ocentra-lan-known-household-registry-{unique_id}.json"
    ))
}

fn canonical_device_id_for_mac(mac_address: &str) -> String {
    let mut canonical_device_id = String::from(constants::lan_pairing::CANONICAL_DEVICE_MAC_PREFIX);
    canonical_device_id.push_str(
        &mac_address
            .chars()
            .filter(|character| character.is_ascii_alphanumeric())
            .flat_map(char::to_lowercase)
            .collect::<String>(),
    );
    canonical_device_id
}

fn stored_known_router() -> LanCanonicalHouseholdDevice {
    LanCanonicalHouseholdDevice {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        canonical_device_id: canonical_device_id_for_mac(constants::lan_pairing::TEST_ROUTER_MAC),
        display_name: "Home Router".to_string(),
        classification: LanCanonicalHouseholdDeviceClassification::NetworkInfrastructure,
        role_badges: Vec::new(),
        enrollable: false,
        discovery_state: LanPairingProductionDiscoveryState::Discovered,
        trust_state: LanPairingTrustState::Unpaired,
        route_id: None,
        route_state: LanCanonicalHouseholdRouteState::Unavailable,
        network_mode: LanPairingNetworkMode::LocalNetwork,
        source_labels: vec![LanCanonicalHouseholdDeviceSource::NetworkNeighbor],
        network_identity: LanCanonicalHouseholdNetworkIdentity {
            hostname: Some("home-router".to_string()),
            ip_addresses: vec![constants::lan_pairing::TEST_ROUTER_IP.to_string()],
            mac_address: Some(constants::lan_pairing::TEST_ROUTER_MAC.to_string()),
            mac_vendor: Some("Example Vendor".to_string()),
            network_interfaces: vec![constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()],
            reachability: LanPairingDeviceReachability::Online,
            confidence: LanCanonicalHouseholdDeviceConfidence::NetworkNeighbor,
            stale_at: None,
            offline_at: None,
            evidence_records: Vec::new(),
        },
        child_agent_inventory: None,
        policy_target_surfaces: vec![
            LanCanonicalHouseholdSurface::Devices,
            LanCanonicalHouseholdSurface::Network,
        ],
    }
}
