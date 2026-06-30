use std::fs::{read_to_string, remove_file};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

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
    app::{
        fields::fields_from_pairs, lan_pairing::LanPairingRuntime,
        websocket::handle_command_text_for_test,
    },
    lan_pairing_test_commands::{
        command_for_target, intent_payload, local_network_target, paired_runtime, pairing_command,
        proof_payload, route_select_command, serialize_command,
    },
};

const STORED_OFFLINE_AT: &str = "2026-06-02T00:00:00Z";

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
    assert!(event
        .payload
        .contains_key(constants::field::LAN_DISCOVERY_SOURCE));
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
            || read_model[constants::field::LAN_PHYSICAL_HOUSEHOLD_LAN_STATE]
                == serde_json::json!(constants::value::LAN_DISCOVERY_STATE_UNAVAILABLE)
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
async fn browser_discovery_scan_returns_before_active_refresh_completes() {
    let event = tokio::time::timeout(
        Duration::from_secs(5),
        handle_command_text_for_test(
            &serialize_command(browser_discovery_scan_command(LogFields::new())),
            LanPairingRuntime::empty(),
            Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
        ),
    )
    .await
    .unwrap_or_else(|_| unreachable!("browser scan dispatch must not block on physical LAN IO"));

    assert_eq!(
        event.event,
        AgentEventName::AgentLanPairingBrowserDiscoveryReported
    );
    assert!(event
        .payload
        .contains_key(constants::field::LAN_ADD_DEVICE_READ_MODEL));
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
async fn add_device_request_persists_household_revocation_in_read_model() {
    let event = handle_command_text_for_test(
        &serialize_command(add_device_request_command(
            household_decision_payload_with_action(
                constants::lan_pairing::HOUSEHOLD_ACTION_REVOKE,
                Some(constants::lan_pairing::OBSERVED_AT),
            ),
        )),
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
        serde_json::json!(constants::lan_pairing::HOUSEHOLD_ACTION_REVOKE)
    );
    assert_eq!(
        read_model[constants::lan_pairing::REGISTRY_KEY_HOUSEHOLD_DEVICE_DECISIONS][0]
            [constants::lan_pairing::HOUSEHOLD_ACTION_REVOKED_AT_FIELD],
        serde_json::json!(constants::lan_pairing::OBSERVED_AT)
    );
}

#[tokio::test]
async fn persistent_runtime_restores_selected_route_and_household_rename_into_read_model() {
    let registry_path = temp_registry_path();
    let _ = remove_file(&registry_path);
    let runtime = LanPairingRuntime::persistent_json(&registry_path);
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
    let initial_scan = handle_command_text_for_test(
        &serialize_command(browser_discovery_scan_command(LogFields::new())),
        runtime.clone(),
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;
    assert_eq!(
        initial_scan.event,
        AgentEventName::AgentLanPairingBrowserDiscoveryReported
    );
    let initial_read_model = typed_read_model_payload(&initial_scan.payload);
    let selected_canonical_device_id = initial_read_model
        .canonical_household_devices
        .iter()
        .find(|device| {
            device.classification == LanCanonicalHouseholdDeviceClassification::ChildAgent
                && device.trust_state == LanPairingTrustState::Paired
                && device.route_id.as_deref()
                    == Some(constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK)
        })
        .map(|device| device.canonical_device_id.clone())
        .unwrap_or_else(|| {
            let available_ids = initial_read_model
                .canonical_household_devices
                .iter()
                .map(|device| device.canonical_device_id.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            unreachable!(
                "selected paired child device present before restart: available [{}]",
                available_ids
            )
        });
    let _ = handle_command_text_for_test(
        &serialize_command(add_device_request_command(
            household_decision_payload_for_device(&selected_canonical_device_id),
        )),
        runtime,
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;

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
    assert_eq!(
        read_model
            .selected_device_readiness
            .selected_child_device_id
            .as_deref(),
        Some(constants::lan_pairing::CHILD_DEVICE_ID)
    );
    assert_eq!(
        read_model.selected_device_readiness.route_id.as_deref(),
        Some(constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK)
    );
    assert!(read_model.selected_device_readiness.ready_for_control);
    assert_eq!(
        read_model.household_device_decisions.len(),
        1,
        "rename decision should survive persistent restart"
    );
    let device = read_model
        .canonical_household_devices
        .iter()
        .find(|device| device.canonical_device_id == selected_canonical_device_id)
        .unwrap_or_else(|| {
            let available_ids = read_model
                .canonical_household_devices
                .iter()
                .map(|device| device.canonical_device_id.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            unreachable!(
                "selected child canonical device restored into read model: expected {}, available [{}]",
                selected_canonical_device_id,
                available_ids
            )
        });
    assert_eq!(
        device.display_name,
        constants::lan_pairing::HOUSEHOLD_RENAMED_DEVICE_LABEL
    );
    assert_eq!(device.trust_state, LanPairingTrustState::Paired);

    let _ = remove_file(&registry_path);
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
    assert_eq!(
        device.network_identity.stale_at.as_deref(),
        Some(read_model.generated_at.as_str())
    );

    let _ = remove_file(&registry_path);
}

#[tokio::test]
async fn persistent_runtime_restores_offline_known_household_device_as_offline() {
    let registry_path = temp_registry_path();
    let _ = remove_file(&registry_path);
    let runtime = LanPairingRuntime::persistent_json(&registry_path);
    {
        let mut registry = runtime
            .registry
            .lock()
            .unwrap_or_else(|_| unreachable!("registry lock available for test"));
        let changed = registry.merge_known_household_devices(vec![stored_offline_known_router()]);
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
        .unwrap_or_else(|| unreachable!("stored offline known router restored into read model"));

    assert_eq!(
        device.discovery_state,
        LanPairingProductionDiscoveryState::Offline
    );
    assert_eq!(
        device.network_identity.reachability,
        LanPairingDeviceReachability::Offline
    );
    assert_eq!(
        device.network_identity.offline_at.as_deref(),
        Some(STORED_OFFLINE_AT)
    );
    assert_eq!(device.network_identity.stale_at, None);

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
    household_decision_payload_for_device_with_action(
        &local_agent_canonical_device_id(),
        constants::lan_pairing::HOUSEHOLD_ACTION_RENAME,
        None,
    )
}

fn household_decision_payload_for_device(canonical_device_id: &str) -> LogFields {
    household_decision_payload_for_device_with_action(
        canonical_device_id,
        constants::lan_pairing::HOUSEHOLD_ACTION_RENAME,
        None,
    )
}

fn household_decision_payload_with_action(
    action_kind: &str,
    revoked_at: Option<&str>,
) -> LogFields {
    household_decision_payload_for_device_with_action(
        &local_agent_canonical_device_id(),
        action_kind,
        revoked_at,
    )
}

fn household_decision_payload_for_device_with_action(
    canonical_device_id: &str,
    action_kind: &str,
    revoked_at: Option<&str>,
) -> LogFields {
    let mut pairs = vec![
        (
            constants::lan_pairing::HOUSEHOLD_ACTION_ID_FIELD,
            LogFieldValue::String(constants::lan_pairing::HOUSEHOLD_ACTION_ID.to_string()),
        ),
        (
            constants::lan_pairing::HOUSEHOLD_ACTION_KIND_FIELD,
            LogFieldValue::String(action_kind.to_string()),
        ),
        (
            constants::field::LAN_CANONICAL_DEVICE_ID,
            LogFieldValue::String(canonical_device_id.to_string()),
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
    ];
    if let Some(revoked_at) = revoked_at {
        pairs.push((
            constants::lan_pairing::HOUSEHOLD_ACTION_REVOKED_AT_FIELD,
            LogFieldValue::String(revoked_at.to_string()),
        ));
    }
    fields_from_pairs(pairs)
}

fn local_agent_canonical_device_id() -> String {
    let mut canonical_device_id = canonical_device_id_for_mac(constants::lan_pairing::TEST_LAN_MAC);
    canonical_device_id.push('-');
    canonical_device_id.push_str(
        &constants::lan_pairing::LOCAL_AGENT_DEVICE_ID
            .chars()
            .filter(|character| character.is_ascii_alphanumeric())
            .flat_map(char::to_lowercase)
            .collect::<String>(),
    );
    canonical_device_id
}

fn serialized_discovery_source(source: LanPairingDiscoverySource) -> String {
    serde_json::to_value(source)
        .ok()
        .and_then(|value| value.as_str().map(ToOwned::to_owned))
        .unwrap_or_default()
}

fn temp_registry_path() -> std::path::PathBuf {
    static REGISTRY_COUNTER: AtomicUsize = AtomicUsize::new(0);
    std::env::temp_dir().join(format!(
        "ocentra-lan-known-household-registry-{}-{}.json",
        std::process::id(),
        REGISTRY_COUNTER.fetch_add(1, Ordering::Relaxed)
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

fn stored_offline_known_router() -> LanCanonicalHouseholdDevice {
    let mut device = stored_known_router();
    device.discovery_state = LanPairingProductionDiscoveryState::Offline;
    device.network_identity.reachability = LanPairingDeviceReachability::Offline;
    device.network_identity.offline_at = Some(STORED_OFFLINE_AT.to_string());
    device
}
