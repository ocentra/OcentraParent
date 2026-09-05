use std::fs::{remove_file, write};

use crate::test_text::TestText;
use crate::trusted_device_registry_support::{
    agent_event_result, household_decision, temp_registry_path,
};
use ocentra_parent_agent_core::trusted_device_registry::TrustedDeviceRegistry;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingAuthenticationState, LanPairingDeviceReachability,
    LanPairingProductionDiscoveryState, LanPairingTrustState,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDevice, LanCanonicalHouseholdDeviceClassification,
    LanCanonicalHouseholdDeviceConfidence, LanCanonicalHouseholdDeviceSource,
    LanCanonicalHouseholdNetworkIdentity, LanCanonicalHouseholdRouteState,
    LanCanonicalHouseholdSurface, LanDiscoveryEvidenceConfidence, LanDiscoveryEvidenceKind,
    LanDiscoveryEvidenceRecord,
};
use std::fmt::Display;

#[test]
fn trusted_device_registry_fails_closed_when_core_entries_are_missing(
) -> Result<(), Box<dyn std::error::Error>> {
    let path = temp_registry_path();
    let _ = remove_file(path.as_ref());
    let registry_json = serde_json::json!({
        constants::field::SCHEMA_VERSION: 1,
        constants::lan_pairing::REGISTRY_KEY_KNOWN_HOUSEHOLD_DEVICES: [
            known_household_device(
                constants::lan_pairing::OBSERVED_AT,
                constants::lan_pairing::OBSERVED_AT,
            )
        ],
    });
    write(&path, registry_json.to_string())?;

    let loaded = TrustedDeviceRegistry::load_json(path.as_ref());
    let _ = remove_file(path.as_ref());

    assert_eq!(loaded.entries().len(), 0);
    assert_eq!(loaded.known_household_devices().len(), 0);
    assert_eq!(
        loaded.authentication_state(),
        LanPairingAuthenticationState::Unpaired
    );
    Ok(())
}

#[test]
fn trusted_device_registry_persists_household_device_decisions_for_restart_recovery() {
    let path = temp_registry_path();
    let _ = remove_file(path.as_ref());
    let mut registry = TrustedDeviceRegistry::empty();
    registry.apply_household_device_decision(household_decision());
    agent_event_result(registry.save_json(path.as_ref()));

    let loaded = TrustedDeviceRegistry::load_json(path.as_ref());
    let _ = remove_file(path.as_ref());

    assert_eq!(loaded.household_device_decisions().len(), 1);
    assert_eq!(
        loaded.household_device_decisions()[0]
            .display_name
            .as_deref(),
        Some(constants::lan_pairing::HOUSEHOLD_RENAMED_DEVICE_LABEL)
    );
    assert_eq!(
        loaded.household_device_decisions()[0]
            .device_kind
            .as_deref(),
        Some(constants::lan_pairing::HOUSEHOLD_DEVICE_KIND_DESKTOP)
    );
}

#[test]
fn trusted_device_registry_persists_known_household_devices_for_restart_recovery() {
    let path = temp_registry_path();
    let _ = remove_file(path.as_ref());
    let mut registry = TrustedDeviceRegistry::empty();
    assert!(
        registry.merge_known_household_devices(vec![known_household_device(
            constants::lan_pairing::OBSERVED_AT,
            constants::lan_pairing::OBSERVED_AT,
        )])
    );
    agent_event_result(registry.save_json(path.as_ref()));

    let loaded = TrustedDeviceRegistry::load_json(path.as_ref());
    let _ = remove_file(path.as_ref());

    assert_eq!(loaded.known_household_devices().len(), 1);
    assert_eq!(
        loaded.known_household_devices()[0]
            .network_identity
            .mac_address
            .as_deref(),
        Some(constants::lan_pairing::TEST_LAN_MAC)
    );
    assert_eq!(
        loaded.known_household_devices()[0]
            .network_identity
            .evidence_records[0]
            .first_seen_at,
        constants::lan_pairing::OBSERVED_AT
    );
}

#[test]
fn known_household_device_merge_preserves_first_seen_and_updates_last_seen() {
    let mut registry = TrustedDeviceRegistry::empty();
    assert!(
        registry.merge_known_household_devices(vec![known_household_device(
            "2026-06-01T00:00:00Z",
            "2026-06-01T00:00:00Z",
        )])
    );
    assert!(
        registry.merge_known_household_devices(vec![known_household_device(
            "2026-06-02T00:00:00Z",
            "2026-06-03T00:00:00Z",
        )])
    );

    let device = &registry.known_household_devices()[0];
    let evidence = &device.network_identity.evidence_records[0];
    assert_eq!(evidence.first_seen_at, "2026-06-01T00:00:00Z");
    assert_eq!(evidence.last_seen_at, "2026-06-03T00:00:00Z");
}

#[test]
fn known_household_device_merge_preserves_distinct_source_backed_evidence_history() {
    let mut registry = TrustedDeviceRegistry::empty();
    let first = known_household_device("2026-06-01T00:00:00Z", "2026-06-01T00:00:00Z");
    let mut second = known_household_device("2026-06-02T00:00:00Z", "2026-06-03T00:00:00Z");
    second.network_identity.evidence_records[0].source =
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource::LocalService;
    second.network_identity.evidence_records[0].evidence_id = "evidence-2".to_string();
    second.network_identity.evidence_records[0].confidence = LanDiscoveryEvidenceConfidence::Strong;

    assert!(registry.merge_known_household_devices(vec![first]));
    assert!(registry.merge_known_household_devices(vec![second]));

    let device = &registry.known_household_devices()[0];
    assert_eq!(device.network_identity.evidence_records.len(), 2);
    assert!(device
        .network_identity
        .evidence_records
        .iter()
        .any(|record| {
            record.source
                == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource::WindowsNeighborTable
                && record.first_seen_at == "2026-06-01T00:00:00Z"
                && record.last_seen_at == "2026-06-01T00:00:00Z"
        }));
    assert!(device
        .network_identity
        .evidence_records
        .iter()
        .any(|record| {
            record.source
                == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource::LocalService
                && record.first_seen_at == "2026-06-02T00:00:00Z"
                && record.last_seen_at == "2026-06-03T00:00:00Z"
        }));
}

#[test]
fn known_household_device_merge_preserves_stronger_paired_child_truth() {
    let mut registry = TrustedDeviceRegistry::empty();
    let paired_child = paired_child_device();
    let weaker_neighbor = weaker_neighbor_device();

    assert!(registry.merge_known_household_devices(vec![paired_child]));
    assert!(registry.merge_known_household_devices(vec![weaker_neighbor]));

    let device = &registry.known_household_devices()[0];
    assert_eq!(
        device.discovery_state,
        LanPairingProductionDiscoveryState::Paired
    );
    assert_eq!(device.trust_state, LanPairingTrustState::Paired);
    assert_eq!(
        device.route_id.as_deref(),
        Some(constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK)
    );
    assert_eq!(
        device.route_state,
        LanCanonicalHouseholdRouteState::LocalNetwork
    );
    assert_eq!(
        device.network_identity.confidence,
        LanCanonicalHouseholdDeviceConfidence::AgentConfirmed
    );
    assert_eq!(device.display_name, "Family Laptop");
    assert_eq!(
        device.network_identity.hostname.as_deref(),
        Some("family-tablet")
    );
    assert_eq!(
        device
            .child_agent_inventory
            .as_ref()
            .map(|inventory| inventory.device_name.as_str()),
        Some("Family Laptop")
    );
    assert_eq!(
        device
            .child_agent_inventory
            .as_ref()
            .map(|inventory| inventory.platform.as_str()),
        Some(constants::lan_pairing::PLATFORM_WINDOWS)
    );
    assert_eq!(device.network_identity.evidence_records.len(), 2);
    assert!(device
        .source_labels
        .contains(&LanCanonicalHouseholdDeviceSource::LocalService));
    assert!(device
        .source_labels
        .contains(&LanCanonicalHouseholdDeviceSource::NetworkNeighbor));
    assert!(device
        .network_identity
        .ip_addresses
        .iter()
        .any(|ip_address| ip_address == "192.168.0.44"));
}

fn paired_child_device() -> LanCanonicalHouseholdDevice {
    let mut paired_child = known_household_device("2026-06-01T00:00:00Z", "2026-06-01T00:00:00Z");
    paired_child.canonical_device_id = constants::lan_pairing::CHILD_DEVICE_ID.to_string();
    paired_child.display_name = "Family Laptop".to_string();
    paired_child.classification = LanCanonicalHouseholdDeviceClassification::ChildAgent;
    paired_child.discovery_state = LanPairingProductionDiscoveryState::Paired;
    paired_child.trust_state = LanPairingTrustState::Paired;
    paired_child.route_id = Some(constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string());
    paired_child.route_state = LanCanonicalHouseholdRouteState::LocalNetwork;
    paired_child.source_labels = vec![LanCanonicalHouseholdDeviceSource::LocalService];
    paired_child.network_identity.confidence =
        LanCanonicalHouseholdDeviceConfidence::AgentConfirmed;
    paired_child.child_agent_inventory =
        Some(ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanChildAgentInventoryPacket {
            device_name: "Family Laptop".to_string(),
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
            role_state: ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdRoleState::Implemented,
            route_state: LanCanonicalHouseholdRouteState::LocalNetwork,
            pairing_trust_state: LanPairingTrustState::Paired,
        });
    paired_child
}

fn weaker_neighbor_device() -> LanCanonicalHouseholdDevice {
    let mut weaker_neighbor =
        known_household_device("2026-06-02T00:00:00Z", "2026-06-03T00:00:00Z");
    weaker_neighbor.canonical_device_id = constants::lan_pairing::CHILD_DEVICE_ID.to_string();
    weaker_neighbor.display_name = format!(
        "{} Family Laptop",
        constants::lan_pairing::NETWORK_NEIGHBOR_LABEL_PREFIX
    );
    weaker_neighbor.classification = LanCanonicalHouseholdDeviceClassification::UnknownLanDevice;
    weaker_neighbor.discovery_state = LanPairingProductionDiscoveryState::Discovered;
    weaker_neighbor.trust_state = LanPairingTrustState::Unpaired;
    weaker_neighbor.route_id = None;
    weaker_neighbor.route_state = LanCanonicalHouseholdRouteState::ManualRequired;
    weaker_neighbor.source_labels = vec![LanCanonicalHouseholdDeviceSource::NetworkNeighbor];
    weaker_neighbor.network_identity.hostname = None;
    weaker_neighbor.network_identity.confidence =
        LanCanonicalHouseholdDeviceConfidence::NetworkNeighbor;
    weaker_neighbor
        .network_identity
        .ip_addresses
        .push("192.168.0.44".to_string());
    weaker_neighbor.network_identity.evidence_records[0].source =
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource::LocalService;
    weaker_neighbor.network_identity.evidence_records[0].evidence_id = "evidence-2".to_string();
    weaker_neighbor.network_identity.evidence_records[0].merge_key =
        "agent:child-device".to_string();
    weaker_neighbor.network_identity.evidence_records[0].value =
        constants::lan_pairing::LOCAL_AGENT_STATUS.to_string();
    weaker_neighbor.network_identity.evidence_records[0].normalized_value =
        constants::lan_pairing::LOCAL_AGENT_STATUS.to_ascii_lowercase();
    weaker_neighbor.network_identity.evidence_records[0].device_id =
        constants::lan_pairing::CHILD_DEVICE_ID.to_string();
    weaker_neighbor
}

#[test]
fn known_household_devices_for_read_model_refreshes_stale_timestamp_on_restart() {
    let mut registry = TrustedDeviceRegistry::empty();
    let mut device = known_household_device("2026-06-01T00:00:00Z", "2026-06-01T00:00:00Z");
    device.network_identity.stale_at = Some("2026-06-02T00:00:00Z".to_string());
    assert!(registry.merge_known_household_devices(vec![device]));

    let observed_at = "2026-06-03T00:00:00Z";
    let restored = registry.known_household_devices_for_read_model(&[], observed_at);
    let restored_device = &restored[0];

    assert_eq!(
        restored_device.discovery_state,
        ocentra_parent_agent_protocol::lan_pairing::LanPairingProductionDiscoveryState::Stale
    );
    assert_eq!(
        restored_device.network_identity.reachability,
        LanPairingDeviceReachability::Stale
    );
    assert_eq!(
        restored_device.network_identity.stale_at.as_deref(),
        Some(observed_at)
    );
    assert_eq!(restored_device.network_identity.evidence_records.len(), 1);
    assert_eq!(
        restored_device.network_identity.evidence_records[0].first_seen_at,
        "2026-06-01T00:00:00Z"
    );
    assert_eq!(
        restored_device.network_identity.evidence_records[0].last_seen_at,
        "2026-06-01T00:00:00Z"
    );
}

#[test]
fn known_household_devices_for_read_model_preserves_offline_restart_state() {
    let mut registry = TrustedDeviceRegistry::empty();
    let mut device = known_household_device("2026-06-01T00:00:00Z", "2026-06-01T00:00:00Z");
    device.discovery_state = LanPairingProductionDiscoveryState::Offline;
    device.network_identity.reachability = LanPairingDeviceReachability::Offline;
    device.network_identity.offline_at = Some("2026-06-02T00:00:00Z".to_string());
    assert!(registry.merge_known_household_devices(vec![device]));

    let observed_at = "2026-06-03T00:00:00Z";
    let restored = registry.known_household_devices_for_read_model(&[], observed_at);
    let restored_device = &restored[0];

    assert_eq!(
        restored_device.discovery_state,
        LanPairingProductionDiscoveryState::Offline
    );
    assert_eq!(
        restored_device.network_identity.reachability,
        LanPairingDeviceReachability::Offline
    );
    assert_eq!(
        restored_device.network_identity.offline_at.as_deref(),
        Some("2026-06-02T00:00:00Z")
    );
    assert_eq!(restored_device.network_identity.stale_at, None);
}

#[test]
fn merge_known_household_devices_keeps_conflicting_canonical_ids_separate() {
    let mut registry = TrustedDeviceRegistry::empty();
    let mut router = known_household_device("2026-06-01T00:00:00Z", "2026-06-01T00:00:00Z");
    router.canonical_device_id = "lan-physical-router-001122334455".to_string();
    router.display_name = "Home Router".to_string();
    router.network_identity.mac_address = Some("00-11-22-33-44-55".to_string());
    let mut child = known_household_device("2026-06-02T00:00:00Z", "2026-06-03T00:00:00Z");
    child.canonical_device_id = "lan-physical-child-aabbccddeeff".to_string();
    child.display_name = "Family Tablet".to_string();
    child.network_identity.mac_address = Some("00-11-22-33-44-55".to_string());

    assert!(registry.merge_known_household_devices(vec![router, child]));

    assert_eq!(registry.known_household_devices().len(), 2);
    let mut device_ids = registry
        .known_household_devices()
        .iter()
        .map(|device| device.canonical_device_id.as_str())
        .collect::<Vec<_>>();
    device_ids.sort_unstable();
    assert_eq!(
        device_ids,
        vec![
            "lan-physical-child-aabbccddeeff",
            "lan-physical-router-001122334455",
        ]
    );
}

fn known_household_device(
    first_seen_at: impl Display,
    last_seen_at: impl Display,
) -> LanCanonicalHouseholdDevice {
    let first_seen_at = TestText::from_display(first_seen_at);
    let last_seen_at = TestText::from_display(last_seen_at);
    LanCanonicalHouseholdDevice {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        canonical_device_id: "lan-physical-mac-001122334455".to_string(),
        display_name: "Family Tablet".to_string(),
        classification: LanCanonicalHouseholdDeviceClassification::UnknownLanDevice,
        role_badges: Vec::new(),
        enrollable: false,
        discovery_state: ocentra_parent_agent_protocol::lan_pairing::LanPairingProductionDiscoveryState::Discovered,
        trust_state: LanPairingTrustState::Unpaired,
        route_id: None,
        route_state: LanCanonicalHouseholdRouteState::ManualRequired,
        network_mode: ocentra_parent_agent_protocol::lan_pairing::LanPairingNetworkMode::LocalNetwork,
        source_labels: vec![LanCanonicalHouseholdDeviceSource::NetworkNeighbor],
        network_identity: LanCanonicalHouseholdNetworkIdentity {
            hostname: Some("family-tablet".to_string()),
            ip_addresses: vec![constants::lan_pairing::TEST_LAN_IP.to_string()],
            mac_address: Some(constants::lan_pairing::TEST_LAN_MAC.to_string()),
            mac_vendor: Some("Example Vendor".to_string()),
            network_interfaces: vec![constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()],
            reachability: LanPairingDeviceReachability::Online,
            confidence: LanCanonicalHouseholdDeviceConfidence::NetworkNeighbor,
            stale_at: None,
            offline_at: None,
            evidence_records: vec![LanDiscoveryEvidenceRecord {
                schema_version: constants::lan_pairing::SCHEMA_VERSION,
                evidence_id: "evidence-1".to_string(),
                source: ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource::WindowsNeighborTable,
                evidence_kind: LanDiscoveryEvidenceKind::MacAddress,
                device_id: "lan-physical-mac-001122334455".to_string(),
                value: constants::lan_pairing::TEST_LAN_MAC.to_string(),
                normalized_value: constants::lan_pairing::TEST_LAN_MAC.to_string(),
                first_seen_at: first_seen_at.to_string(),
                last_seen_at: last_seen_at.to_string(),
                expires_at: None,
                confidence: LanDiscoveryEvidenceConfidence::Confirmed,
                merge_key: "mac:001122334455".to_string(),
                note: None,
            }],
        },
        child_agent_inventory: None,
        policy_target_surfaces: vec![
            LanCanonicalHouseholdSurface::Devices,
            LanCanonicalHouseholdSurface::Network,
        ],
    }
}
