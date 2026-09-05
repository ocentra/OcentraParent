use std::primitive::str as TestStr;
use std::string::String as TestString;

use chrono::{Duration, SecondsFormat, Utc};
use ocentra_lan_core::network_inventory::{LanDiscoveryRefreshMode, LanNetworkInventoryDevice};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingNetworkMode, LanPairingProductionDiscoveryState,
    LanPairingText, LanPairingTrustState,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDevice, LanCanonicalHouseholdDeviceClassification,
    LanCanonicalHouseholdDeviceConfidence, LanCanonicalHouseholdDeviceSource,
    LanCanonicalHouseholdNetworkIdentity, LanCanonicalHouseholdRoleState,
    LanCanonicalHouseholdRouteState, LanCanonicalHouseholdSurface, LanChildAgentInventoryPacket,
    LanHouseholdDeviceActionKind, LanHouseholdDeviceDecision,
};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentRoute,
};

use super::{
    command_uses_physical_lan_scan, inventory_refresh_mode_after_targeted_refresh,
    network_device_scan_result_for_command, recent_previous_scan_agent_truth_devices,
    refresh_mode_for_command, scan_history_is_recent, scan_truth_context,
};
use crate::app::lan_pairing::LanPairingRuntime;
use crate::app::lan_pairing_browser_add_device_state::scan_history::LanScanHistorySnapshot;
use crate::lan_pairing_browser_add_device_state::scan_history::load_scan_history_snapshot;
use crate::lan_runtime_test_support::{
    cleanup_persistent_runtime, persistent_runtime_with_devices, write_scan_history_snapshot,
};
use crate::test_require_ok::require_ok;
use crate::test_require_some::require_some;

#[test]
fn status_and_scan_commands_keep_physical_lan_inventory_enabled() {
    assert!(command_uses_physical_lan_scan(
        &AgentCommandName::AgentLanPairingStatusGet
    ));
    assert!(command_uses_physical_lan_scan(
        &AgentCommandName::AgentLanPairingBrowserDiscoveryScan
    ));
    assert!(command_uses_physical_lan_scan(
        &AgentCommandName::AgentLanPairingAddDeviceRequest
    ));
}

#[test]
fn unrelated_lan_commands_do_not_trigger_physical_lan_inventory() {
    assert!(!command_uses_physical_lan_scan(
        &AgentCommandName::AgentLanPairingControllerLeaseRenew
    ));
}

#[test]
fn recent_previous_scan_child_truth_is_reused_for_probe_suppression_only() {
    let now = Utc::now();
    let snapshot = LanScanHistorySnapshot {
        schema_version: 1,
        updated_at: now.to_rfc3339_opts(SecondsFormat::Millis, true),
        replay_canonical_projection: None,
        metadata: None,
        devices: vec![previous_scan_device(
            constants::lan_pairing::LOCAL_AGENT_STATUS,
        )],
    };

    let devices = recent_previous_scan_agent_truth_devices(Some(&snapshot), now);

    assert_eq!(devices.len(), 1);
    assert_eq!(
        devices[0].agent_status.as_deref(),
        Some(constants::lan_pairing::LOCAL_AGENT_STATUS)
    );
    assert_eq!(
        devices[0].mac_address.as_deref(),
        Some(constants::lan_pairing::TEST_LAN_MAC)
    );
}

#[test]
fn stale_previous_scan_child_truth_does_not_suppress_probe_forever() {
    let now = Utc::now();
    let stale_updated_at = (now
        - Duration::seconds(
            constants::lan_pairing::LAN_PREVIOUS_SCAN_AGENT_TRUTH_REUSE_WINDOW_SECONDS + 1,
        ))
    .to_rfc3339_opts(SecondsFormat::Millis, true);
    let snapshot = LanScanHistorySnapshot {
        schema_version: 1,
        updated_at: stale_updated_at,
        replay_canonical_projection: None,
        metadata: None,
        devices: vec![previous_scan_device(
            constants::lan_pairing::LOCAL_AGENT_STATUS,
        )],
    };

    assert!(recent_previous_scan_agent_truth_devices(Some(&snapshot), now).is_empty());
}

#[test]
fn invalid_history_timestamp_is_not_treated_as_recent() {
    assert!(!scan_history_is_recent(
        &LanPairingText::from(constants::lan_pairing::NETWORK_NEIGHBOR_UNKNOWN_HOSTNAME),
        Utc::now(),
    ));
}

#[test]
fn status_reads_stay_passive_while_browser_scan_and_add_device_refresh_actively() {
    assert_eq!(
        refresh_mode_for_command(&AgentCommandName::AgentLanPairingStatusGet),
        LanDiscoveryRefreshMode::Passive
    );
    assert_eq!(
        refresh_mode_for_command(&AgentCommandName::AgentLanPairingBrowserDiscoveryScan),
        LanDiscoveryRefreshMode::ActiveSubnetRefresh
    );
    assert_eq!(
        refresh_mode_for_command(&AgentCommandName::AgentLanPairingAddDeviceRequest),
        LanDiscoveryRefreshMode::ActiveSubnetRefresh
    );
}

#[test]
fn active_scan_commands_probe_once_then_read_inventory_passively() {
    assert_eq!(
        inventory_refresh_mode_after_targeted_refresh(refresh_mode_for_command(
            &AgentCommandName::AgentLanPairingBrowserDiscoveryScan
        )),
        LanDiscoveryRefreshMode::Passive
    );
    assert_eq!(
        inventory_refresh_mode_after_targeted_refresh(refresh_mode_for_command(
            &AgentCommandName::AgentLanPairingAddDeviceRequest
        )),
        LanDiscoveryRefreshMode::Passive
    );
    assert_eq!(
        inventory_refresh_mode_after_targeted_refresh(refresh_mode_for_command(
            &AgentCommandName::AgentLanPairingStatusGet
        )),
        LanDiscoveryRefreshMode::Passive
    );
}

#[test]
fn localhost_status_and_runtime_stream_preserve_recent_cached_snapshot_context() {
    let devices = vec![previous_scan_device(
        constants::lan_pairing::LOCAL_AGENT_STATUS,
    )];
    let (runtime, registry_path) = persistent_runtime_with_devices(&devices);
    let snapshot = require_some(
        load_scan_history_snapshot(&runtime),
        constants::value::LAN_READ_MODEL_JSON_EXPECTATION,
    );

    let status = network_device_scan_result_for_command(
        &runtime,
        &status_command_for_route(AgentRoute::Localhost),
    );
    let stream = network_device_scan_result_for_command(
        &runtime,
        &runtime_stream_command_for_route(AgentRoute::LocalNetwork),
    );

    cleanup_persistent_runtime(&registry_path);

    assert_eq!(status.devices.len(), 1);
    assert_eq!(
        status.devices[0].ip_address,
        constants::lan_pairing::TEST_LAN_IP
    );
    assert_eq!(stream, status);
    assert_eq!(stream.previous_scan_snapshot, Some(snapshot.clone()));
    assert_eq!(stream.current_scan_snapshot, Some(snapshot));
    assert!(stream.reused_recent_snapshot);
}

#[test]
fn localhost_status_without_cache_returns_without_physical_refresh() {
    let result = network_device_scan_result_for_command(
        &LanPairingRuntime::empty(),
        &status_command_for_route(AgentRoute::Localhost),
    );

    assert_eq!(result.devices.len(), 0);
    assert!(!result.reused_recent_snapshot);
    assert!(result.current_scan_snapshot.is_none());
}

#[test]
fn localhost_status_and_runtime_stream_preserve_stale_previous_snapshot_context() {
    let now = Utc::now();
    let stale_updated_at = (now
        - Duration::seconds(
            constants::lan_pairing::LAN_PREVIOUS_SCAN_AGENT_TRUTH_REUSE_WINDOW_SECONDS + 1,
        ))
    .to_rfc3339_opts(SecondsFormat::Millis, true);
    let devices = vec![previous_scan_device(
        constants::lan_pairing::LOCAL_AGENT_STATUS,
    )];
    let (runtime, registry_path) = persistent_runtime_with_devices(&devices);
    let mut expected_snapshot = require_some(
        load_scan_history_snapshot(&runtime),
        constants::value::LAN_READ_MODEL_JSON_EXPECTATION,
    );
    expected_snapshot.updated_at = stale_updated_at;
    write_scan_history_snapshot(&registry_path, &expected_snapshot);

    let result = network_device_scan_result_for_command(
        &runtime,
        &status_command_for_route(AgentRoute::Localhost),
    );
    let stream = network_device_scan_result_for_command(
        &runtime,
        &runtime_stream_command_for_route(AgentRoute::LocalNetwork),
    );

    cleanup_persistent_runtime(&registry_path);

    assert_eq!(stream, result);
    assert_eq!(result.devices.len(), 0);
    assert!(!result.reused_recent_snapshot);
    assert_eq!(result.previous_scan_snapshot, Some(expected_snapshot));
    assert!(result.current_scan_snapshot.is_none());
}

#[test]
fn local_network_status_cancellation_preserves_prior_context_without_reusing_it() {
    let devices = vec![previous_scan_device(
        constants::lan_pairing::LOCAL_AGENT_STATUS,
    )];
    let (runtime, registry_path) = persistent_runtime_with_devices(&devices);
    let expected_snapshot = require_some(
        load_scan_history_snapshot(&runtime),
        constants::value::LAN_READ_MODEL_JSON_EXPECTATION,
    );
    let cancellation = std::sync::atomic::AtomicBool::new(true);
    let result = crate::lan_pairing_browser_add_device_state::physical_lan_scan::cancellation::refresh_network_device_scan_history_with_cancellation(
        &runtime,
        &status_command_for_route(AgentRoute::LocalNetwork),
        &cancellation,
    );

    cleanup_persistent_runtime(&registry_path);

    assert!(result.devices.is_empty());
    assert!(!result.reused_recent_snapshot);
    assert_eq!(result.previous_scan_snapshot, Some(expected_snapshot));
    assert!(result.current_scan_snapshot.is_none());
}

#[test]
fn previous_router_truth_becomes_scan_suppression_without_upgrading_identity_truth() {
    let now = Utc::now();
    let snapshot = LanScanHistorySnapshot {
        schema_version: 2,
        updated_at: now.to_rfc3339_opts(SecondsFormat::Millis, true),
        replay_canonical_projection: None,
        metadata: None,
        devices: vec![router_scan_device()],
    };

    let runtime = LanPairingRuntime::empty();
    let devices = scan_truth_context(&runtime, Some(&snapshot), now).scan_suppression_devices;

    assert_eq!(devices.len(), 1);
    assert_eq!(
        devices[0].mac_address.as_deref(),
        Some(constants::lan_pairing::TEST_ROUTER_MAC)
    );
    assert_eq!(
        devices[0].ip_address.as_deref(),
        Some(constants::lan_pairing::TEST_ROUTER_IP)
    );
}

#[test]
fn ignored_previous_household_device_becomes_scan_suppression_truth() {
    let now = Utc::now();
    let snapshot = LanScanHistorySnapshot {
        schema_version: 2,
        updated_at: now.to_rfc3339_opts(SecondsFormat::Millis, true),
        replay_canonical_projection: None,
        metadata: None,
        devices: vec![previous_scan_device(
            constants::lan_pairing::LOCAL_AGENT_STATUS,
        )],
    };
    let mut decisions = vec![LanHouseholdDeviceDecision {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        action_id: constants::lan_pairing::HOUSEHOLD_ACTION_ID.to_string(),
        action_kind: LanHouseholdDeviceActionKind::Ignore,
        canonical_device_id: canonical_device_id_for_mac(constants::lan_pairing::TEST_LAN_MAC),
        child_profile_id: None,
        display_name: None,
        device_kind: None,
        parent_actor_id: constants::lan_pairing::PARENT_ACTOR_ID.to_string(),
        decided_at: now.to_rfc3339_opts(SecondsFormat::Millis, true),
        revoked_at: None,
    }];

    let runtime = LanPairingRuntime::empty();
    {
        let mut registry = require_ok(
            runtime.registry.lock(),
            constants::error::AGENT_EVENT_SERIALIZES,
        );
        registry.apply_household_device_decision(decisions.remove(0));
    }
    let devices = scan_truth_context(&runtime, Some(&snapshot), now).scan_suppression_devices;

    assert_eq!(devices.len(), 1);
    assert_eq!(
        devices[0].mac_address.as_deref(),
        Some(constants::lan_pairing::TEST_LAN_MAC)
    );
}

#[test]
fn stored_known_router_truth_suppresses_scan_work_without_scan_history() {
    let runtime = LanPairingRuntime::empty();
    {
        let mut registry = require_ok(
            runtime.registry.lock(),
            constants::error::AGENT_EVENT_SERIALIZES,
        );
        assert!(registry.merge_known_household_devices(vec![stored_known_router()]));
    }
    let devices = scan_truth_context(&runtime, None, Utc::now()).scan_suppression_devices;

    assert_eq!(devices.len(), 1);
    assert_eq!(
        devices[0].mac_address.as_deref(),
        Some(constants::lan_pairing::TEST_ROUTER_MAC)
    );
}

#[test]
fn stored_known_child_agent_truth_feeds_identity_and_scan_context_without_scan_history() {
    let runtime = LanPairingRuntime::empty();
    {
        let mut registry = require_ok(
            runtime.registry.lock(),
            constants::error::AGENT_EVENT_SERIALIZES,
        );
        let changed = registry.merge_known_household_devices(vec![stored_known_child_agent()]);
        assert!(changed);
    }

    let context = scan_truth_context(&runtime, None, Utc::now());

    assert_eq!(context.identity_hint_devices.len(), 1);
    assert_eq!(context.durable_household_truth_count, 1);
    assert_eq!(context.scan_suppression_devices.len(), 1);
    assert_eq!(
        context.identity_hint_devices[0].mac_address.as_deref(),
        Some(constants::lan_pairing::TEST_LAN_MAC)
    );
    assert_eq!(
        context.scan_suppression_devices[0].mac_address.as_deref(),
        Some(constants::lan_pairing::TEST_LAN_MAC)
    );
    assert_eq!(
        context.scan_suppression_devices[0].ip_address.as_deref(),
        Some(constants::lan_pairing::TEST_LAN_IP)
    );
}

#[tokio::test]
async fn scan_truth_context_reuses_registry_and_history_truth_without_agentless_devices() {
    let runtime = LanPairingRuntime::empty();
    {
        let mut registry = require_ok(
            runtime.registry.lock(),
            constants::error::AGENT_EVENT_SERIALIZES,
        );
        let changed = registry.merge_known_household_devices(vec![stored_known_router()]);
        assert!(changed);
    }

    let now = Utc::now();
    let snapshot = LanScanHistorySnapshot {
        schema_version: 2,
        updated_at: now.to_rfc3339_opts(SecondsFormat::Millis, true),
        replay_canonical_projection: None,
        metadata: None,
        devices: vec![
            previous_scan_device(constants::lan_pairing::LOCAL_AGENT_STATUS),
            agentless_scan_device(),
        ],
    };

    let context = scan_truth_context(&runtime, Some(&snapshot), now);

    assert_eq!(context.paired_registry_truth_count, 0);
    assert_eq!(context.recent_previous_agent_truth_count, 1);
    assert_eq!(context.durable_household_truth_count, 2);
    assert_eq!(context.identity_hint_devices.len(), 2);
    assert!(context
        .identity_hint_devices
        .iter()
        .any(|device| device.mac_address.as_deref() == Some(constants::lan_pairing::TEST_LAN_MAC)));
    assert!(context.identity_hint_devices.iter().any(
        |device| device.mac_address.as_deref() == Some(constants::lan_pairing::TEST_ROUTER_MAC)
    ));
    assert!(context
        .identity_hint_devices
        .iter()
        .all(|device| device.mac_address.as_deref() != Some("00-66-77-88-99-AA")));
}

fn previous_scan_device(agent_status: &TestStr) -> LanNetworkInventoryDevice {
    LanNetworkInventoryDevice {
        device_id: constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
        label: constants::lan_pairing::TEST_HOSTNAME.to_string(),
        platform: constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
        ip_address: constants::lan_pairing::TEST_LAN_IP.to_string(),
        mac_address: constants::lan_pairing::TEST_LAN_MAC.to_string(),
        hostname: Some(constants::lan_pairing::TEST_HOSTNAME.to_string()),
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
        observed_at: constants::lan_pairing::OBSERVED_AT.to_string(),
        reachability: LanPairingDeviceReachability::Online,
        agent_status: Some(agent_status.to_string()),
        scan_sources: vec![
            constants::lan_pairing::LAN_SCAN_SOURCE_SERVICE_IDENTITY_PROBE.to_string(),
        ],
        used_previous_scan_hint: false,
        service_identity_probe_evidence: Vec::new(),
    }
}

fn status_command_for_route(route: AgentRoute) -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: 1,
        message_id: constants::lan_pairing::COMMAND_STATUS_GET.to_string(),
        sent_at: String::new(),
        source: ocentra_parent_agent_protocol::transport::AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: ocentra_parent_agent_protocol::transport::AgentPeerRole::Portal,
        },
        target: ocentra_parent_agent_protocol::transport::AgentMessageTarget {
            device_id: constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
            platform: constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
            route,
        },
        command: AgentCommandName::AgentLanPairingStatusGet,
        payload: ocentra_parent_agent_protocol::logging::LogFields::new(),
    }
}

fn runtime_stream_command_for_route(route: AgentRoute) -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        command: AgentCommandName::AgentLanRuntimeEventChainStreamGet,
        ..status_command_for_route(route)
    }
}

fn router_scan_device() -> LanNetworkInventoryDevice {
    LanNetworkInventoryDevice {
        device_id: constants::lan_pairing::SECOND_CHILD_DEVICE_ID.to_string(),
        label: constants::lan_pairing::PLATFORM_ROUTER.to_string(),
        platform: constants::lan_pairing::PLATFORM_ROUTER.to_string(),
        ip_address: constants::lan_pairing::TEST_ROUTER_IP.to_string(),
        mac_address: constants::lan_pairing::TEST_ROUTER_MAC.to_string(),
        hostname: Some(constants::lan_pairing::PLATFORM_ROUTER.to_string()),
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
        observed_at: constants::lan_pairing::OBSERVED_AT.to_string(),
        reachability: LanPairingDeviceReachability::Online,
        agent_status: None,
        scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
        used_previous_scan_hint: false,
        service_identity_probe_evidence: Vec::new(),
    }
}

fn canonical_device_id_for_mac(mac_address: &TestStr) -> TestString {
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
        display_name: constants::lan_pairing::PLATFORM_ROUTER.to_string(),
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
            hostname: Some(constants::lan_pairing::PLATFORM_ROUTER.to_string()),
            ip_addresses: vec![constants::lan_pairing::TEST_ROUTER_IP.to_string()],
            mac_address: Some(constants::lan_pairing::TEST_ROUTER_MAC.to_string()),
            mac_vendor: None,
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

fn stored_known_child_agent() -> LanCanonicalHouseholdDevice {
    LanCanonicalHouseholdDevice {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        canonical_device_id: canonical_device_id_for_mac(constants::lan_pairing::TEST_LAN_MAC),
        display_name: constants::lan_pairing::TEST_HOSTNAME.to_string(),
        classification: LanCanonicalHouseholdDeviceClassification::ChildAgent,
        role_badges: Vec::new(),
        enrollable: true,
        discovery_state: LanPairingProductionDiscoveryState::Discovered,
        trust_state: LanPairingTrustState::Unpaired,
        route_id: Some(constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string()),
        route_state: LanCanonicalHouseholdRouteState::LocalNetwork,
        network_mode: LanPairingNetworkMode::LocalNetwork,
        source_labels: vec![LanCanonicalHouseholdDeviceSource::LocalService],
        network_identity: LanCanonicalHouseholdNetworkIdentity {
            hostname: Some(constants::lan_pairing::TEST_HOSTNAME.to_string()),
            ip_addresses: vec![constants::lan_pairing::TEST_LAN_IP.to_string()],
            mac_address: Some(constants::lan_pairing::TEST_LAN_MAC.to_string()),
            mac_vendor: None,
            network_interfaces: vec![constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()],
            reachability: LanPairingDeviceReachability::Online,
            confidence: LanCanonicalHouseholdDeviceConfidence::AgentConfirmed,
            stale_at: None,
            offline_at: None,
            evidence_records: Vec::new(),
        },
        child_agent_inventory: Some(LanChildAgentInventoryPacket {
            device_name: constants::lan_pairing::TEST_HOSTNAME.to_string(),
            platform: constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
            os: constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
            cpu_model: None,
            cpu_cores: None,
            memory_total: None,
            gpu_model: None,
            gpu_driver: None,
            gpu_memory: None,
            nvidia_smi: None,
            network_interfaces: vec![constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()],
            capabilities: vec![constants::lan_pairing::SURFACE_SCREEN.to_string()],
            role_state: LanCanonicalHouseholdRoleState::Implemented,
            route_state: LanCanonicalHouseholdRouteState::LocalNetwork,
            pairing_trust_state: LanPairingTrustState::Unpaired,
        }),
        policy_target_surfaces: vec![
            LanCanonicalHouseholdSurface::Devices,
            LanCanonicalHouseholdSurface::Screen,
        ],
    }
}

fn agentless_scan_device() -> LanNetworkInventoryDevice {
    LanNetworkInventoryDevice {
        device_id: "lan-agentless-device".to_string(),
        label: "Media Player".to_string(),
        platform: "unknown".to_string(),
        ip_address: "192.168.0.77".to_string(),
        mac_address: "00-66-77-88-99-AA".to_string(),
        hostname: Some("media-player".to_string()),
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
        observed_at: constants::lan_pairing::OBSERVED_AT.to_string(),
        reachability: LanPairingDeviceReachability::Online,
        agent_status: None,
        scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
        used_previous_scan_hint: false,
        service_identity_probe_evidence: Vec::new(),
    }
}
