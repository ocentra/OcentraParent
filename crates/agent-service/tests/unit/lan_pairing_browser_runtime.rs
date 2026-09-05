use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use ocentra_lan_core::network_inventory::passive_discovery::{
    LanPassiveDiscoverySource, LanPassiveDiscoveryTriggerReason,
};
use ocentra_lan_core::network_inventory::LanPassiveRuntimeLocalNetworkIdentity;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingNetworkMode, LanPairingProductionDiscoveryState,
    LanPairingTrustState,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDevice, LanCanonicalHouseholdDeviceClassification,
    LanCanonicalHouseholdDeviceConfidence, LanCanonicalHouseholdDeviceSource,
    LanCanonicalHouseholdNetworkIdentity, LanCanonicalHouseholdRouteState,
    LanCanonicalHouseholdSurface, LanPairingDiscoverySource,
};
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventName,
};
use serde_json::Value;
use std::path::PathBuf as TestPathBuf;
use std::primitive::str as TestStr;
use std::string::String as TestString;

use crate::{
    app::{
        fields::fields_from_pairs, lan_pairing::LanPairingRuntime,
        websocket::handle_command_text_for_test,
    },
    lan_pairing_test_commands::{
        command_for_target, intent_payload, local_network_target, pairing_command, proof_payload,
        route_select_command, serialize_command,
    },
    test_require_ok::require_ok,
    test_require_some::require_some,
    test_text::TestText,
};

const STORED_OFFLINE_AT: &TestStr = "2026-06-02T00:00:00Z";

#[path = "lan_pairing_browser_runtime/persistent_read_model.rs"]
mod persistent_read_model_tests;

#[tokio::test]
async fn unpaired_browser_discovery_scan_reports_unavailable_service_state() {
    let event = handle_command_text_for_test(
        serialize_command(browser_discovery_scan_command(LogFields::new())),
        LanPairingRuntime::empty(),
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;

    assert_eq!(
        event.event,
        AgentEventName::AgentLanPairingBrowserDiscoveryReported
    );
    assert!(event
        .payload
        .get(constants::field::LAN_DISCOVERY_SOURCE)
        .is_some());
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
        serde_json::json!(constants::value::LAN_DISCOVERY_STATE_UNAVAILABLE)
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
        read_model[constants::field::LAN_TRUSTED_DEVICE_REGISTRY],
        serde_json::json!([])
    );
    assert_eq!(
        read_model[constants::field::LAN_SELECTED_DEVICE_READINESS]
            [constants::field::LAN_READY_FOR_CONTROL],
        serde_json::json!(false)
    );
}

#[tokio::test]
async fn browser_discovery_scan_returns_before_active_refresh_completes() {
    let event = require_ok(
        tokio::time::timeout(
            Duration::from_secs(5),
            handle_command_text_for_test(
                serialize_command(browser_discovery_scan_command(LogFields::new())),
                LanPairingRuntime::empty(),
                Some(TestText::from_display(
                    constants::lan_pairing::ALLOWED_ORIGIN,
                )),
            ),
        )
        .await,
        "browser scan dispatch must not block on physical LAN IO",
    );

    assert_eq!(
        event.event,
        AgentEventName::AgentLanPairingBrowserDiscoveryReported
    );
    assert!(event
        .payload
        .get(constants::field::LAN_ADD_DEVICE_READ_MODEL)
        .is_some());
}

#[tokio::test]
async fn stale_add_device_challenge_is_rejected_without_runtime_mutation() {
    let runtime = LanPairingRuntime::empty();
    let event = handle_command_text_for_test(
        serialize_command(add_device_request_command(challenge_request_payload())),
        runtime.clone(),
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;

    assert_rejected_without_read_model(&event, constants::value::LAN_REASON_STALE);
    assert_eq!(runtime.trusted_device_count(), 0);
}

#[tokio::test]
async fn add_device_request_rejects_wrong_origin_without_trusting_device() {
    let event = handle_command_text_for_test(
        serialize_command(add_device_request_command(challenge_request_payload())),
        LanPairingRuntime::empty(),
        Some(TestText::from_display(constants::lan_pairing::WRONG_ORIGIN)),
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
async fn add_device_request_rejects_malformed_payload_without_trusting_device() {
    let event = handle_command_text_for_test(
        serialize_command(add_device_request_command(LogFields::new())),
        LanPairingRuntime::empty(),
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;

    assert_eq!(event.event, AgentEventName::AgentCommandRejected);
    assert_eq!(
        event.payload.get(constants::field::LAN_REJECTION_REASON),
        Some(&LogFieldValue::String(
            constants::value::LAN_REASON_MALFORMED.to_string()
        ))
    );
    assert!(event
        .payload
        .get(constants::field::LAN_ADD_DEVICE_READ_MODEL)
        .is_none());
}

#[tokio::test]
async fn unpaired_household_decision_is_rejected_without_registry_mutation() {
    let runtime = LanPairingRuntime::empty();
    let event = handle_command_text_for_test(
        serialize_command(add_device_request_command(household_decision_payload())),
        runtime.clone(),
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;

    assert_rejected_without_read_model(&event, constants::value::LAN_REASON_ANONYMOUS);
    assert!(crate::lan_pairing_browser_add_device_state::registry_projection::household_device_decisions(
        &runtime
    )
    .is_empty());
}

#[tokio::test]
async fn unpaired_household_revocation_is_rejected_without_registry_mutation() {
    let runtime = LanPairingRuntime::empty();
    let event = handle_command_text_for_test(
        serialize_command(add_device_request_command(
            household_decision_payload_with_action(
                constants::lan_pairing::HOUSEHOLD_ACTION_REVOKE,
                Some(constants::lan_pairing::OBSERVED_AT.to_string()),
            ),
        )),
        runtime.clone(),
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;

    assert_rejected_without_read_model(&event, constants::value::LAN_REASON_ANONYMOUS);
    assert!(crate::lan_pairing_browser_add_device_state::registry_projection::household_device_decisions(
        &runtime
    )
    .is_empty());
}

#[tokio::test]
async fn unpaired_runtime_scan_exposes_empty_registry_and_blocked_readiness() {
    let event = handle_command_text_for_test(
        serialize_command(browser_discovery_scan_command(LogFields::new())),
        LanPairingRuntime::empty(),
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;

    assert_eq!(
        event.event,
        AgentEventName::AgentLanPairingBrowserDiscoveryReported
    );
    let read_model = read_model_payload(&event.payload);
    assert_eq!(
        read_model[constants::field::LAN_TRUSTED_DEVICE_REGISTRY],
        serde_json::json!([])
    );
    assert_eq!(
        read_model[constants::field::LAN_SELECTED_DEVICE_READINESS]
            [constants::field::LAN_READY_FOR_CONTROL],
        serde_json::json!(false)
    );
}

#[test]
fn passive_discovery_helpers_report_network_identity_changes_and_sources() {
    let previous = LanPassiveRuntimeLocalNetworkIdentity {
        ip_address: Some("192.168.1.10".to_string()),
        network_interface: Some("wlan0".to_string()),
        wifi_ssid: Some("old-wifi".to_string()),
        default_gateway: Some("192.168.1.1".to_string()),
    };
    let current = LanPassiveRuntimeLocalNetworkIdentity {
        ip_address: Some("192.168.1.11".to_string()),
        network_interface: Some("eth0".to_string()),
        wifi_ssid: Some("new-wifi".to_string()),
        default_gateway: Some("192.168.1.254".to_string()),
    };

    let triggers =
        crate::lan_pairing_runtime_state::passive_discovery::local_network_change_triggers(
            Some(&previous),
            &current,
        );
    let reasons = triggers
        .iter()
        .map(|trigger| trigger.reason.clone())
        .collect::<Vec<_>>();
    assert_eq!(
        reasons,
        vec![
            LanPassiveDiscoveryTriggerReason::InterfaceDown,
            LanPassiveDiscoveryTriggerReason::InterfaceUp,
            LanPassiveDiscoveryTriggerReason::IpAddressChanged,
            LanPassiveDiscoveryTriggerReason::DefaultGatewayChanged,
        ]
    );
    assert_eq!(
        crate::lan_pairing_runtime_state::passive_discovery::passive_discovery_udp_sources(),
        &[
            LanPassiveDiscoverySource::Dhcp,
            LanPassiveDiscoverySource::Mdns,
            LanPassiveDiscoverySource::Ssdp,
            LanPassiveDiscoverySource::WsDiscovery,
            LanPassiveDiscoverySource::Llmnr,
            LanPassiveDiscoverySource::Netbios,
        ]
    );
}

fn read_model_payload(payload: &LogFields) -> Value {
    match payload.get(constants::field::LAN_ADD_DEVICE_READ_MODEL) {
        Some(LogFieldValue::String(value)) => require_ok(
            serde_json::from_str(value),
            constants::value::LAN_READ_MODEL_JSON_EXPECTATION,
        ),
        _ => serde_json::json!({}),
    }
}

fn assert_rejected_without_read_model(
    event: &ocentra_parent_agent_protocol::transport::AgentEventEnvelope,
    reason: &str,
) {
    assert_eq!(event.event, AgentEventName::AgentCommandRejected);
    assert_eq!(
        event.payload.get(constants::field::LAN_REJECTION_REASON),
        Some(&LogFieldValue::String(reason.to_string()))
    );
    assert!(event
        .payload
        .get(constants::field::LAN_ADD_DEVICE_READ_MODEL)
        .is_none());
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
        local_agent_canonical_device_id(),
        constants::lan_pairing::HOUSEHOLD_ACTION_RENAME,
        None,
    )
}

fn household_decision_payload_with_action(
    action_kind: impl Into<TestString>,
    revoked_at: Option<TestString>,
) -> LogFields {
    household_decision_payload_for_device_with_action(
        local_agent_canonical_device_id(),
        action_kind,
        revoked_at,
    )
}

fn household_decision_payload_for_device_with_action(
    canonical_device_id: impl Into<TestString>,
    action_kind: impl Into<TestString>,
    revoked_at: Option<TestString>,
) -> LogFields {
    let canonical_device_id = canonical_device_id.into();
    let action_kind = action_kind.into();
    let mut pairs = vec![
        (
            constants::lan_pairing::HOUSEHOLD_ACTION_ID_FIELD,
            LogFieldValue::String(constants::lan_pairing::HOUSEHOLD_ACTION_ID.to_string()),
        ),
        (
            constants::lan_pairing::HOUSEHOLD_ACTION_KIND_FIELD,
            LogFieldValue::String(action_kind),
        ),
        (
            constants::field::LAN_CANONICAL_DEVICE_ID,
            LogFieldValue::String(canonical_device_id),
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
            LogFieldValue::String(revoked_at),
        ));
    }
    fields_from_pairs(pairs)
}

fn local_agent_canonical_device_id() -> TestString {
    let mut canonical_device_id = canonical_device_id_for_mac(constants::lan_pairing::TEST_LAN_MAC);
    canonical_device_id.push('-');
    canonical_device_id.push_str(
        &constants::lan_pairing::LOCAL_AGENT_DEVICE_ID
            .chars()
            .filter(|character| character.is_ascii_alphanumeric())
            .flat_map(char::to_lowercase)
            .collect::<TestString>(),
    );
    canonical_device_id
}

fn serialized_discovery_source(source: LanPairingDiscoverySource) -> TestString {
    require_some(
        require_ok(
            serde_json::to_value(source),
            constants::value::LAN_READ_MODEL_JSON_EXPECTATION,
        )
        .as_str(),
        constants::value::LAN_READ_MODEL_JSON_EXPECTATION,
    )
    .to_owned()
}

fn temp_registry_path() -> TestPathBuf {
    static REGISTRY_COUNTER: AtomicUsize = AtomicUsize::new(0);
    std::env::temp_dir().join(format!(
        "ocentra-lan-known-household-registry-{}-{}.json",
        std::process::id(),
        REGISTRY_COUNTER.fetch_add(1, Ordering::Relaxed)
    ))
}

fn canonical_device_id_for_mac(mac_address: impl Into<TestString>) -> TestString {
    let mac_address = mac_address.into();
    let mut canonical_device_id =
        TestString::from(constants::lan_pairing::CANONICAL_DEVICE_MAC_PREFIX);
    canonical_device_id.push_str(
        &mac_address
            .chars()
            .filter(|character| character.is_ascii_alphanumeric())
            .flat_map(char::to_lowercase)
            .collect::<TestString>(),
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
