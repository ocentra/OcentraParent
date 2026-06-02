use crate::{
    constants, LanBrowserAddDeviceDiscoveryDevice, LanBrowserAddDeviceScanSummary,
    LanCanonicalHouseholdDevice, LanCanonicalHouseholdDeviceClassification,
    LanCanonicalHouseholdDeviceConfidence, LanCanonicalHouseholdDeviceRole,
    LanCanonicalHouseholdDeviceSource, LanCanonicalHouseholdNetworkIdentity,
    LanCanonicalHouseholdRoleState, LanCanonicalHouseholdRouteState, LanCanonicalHouseholdSurface,
    LanChildAgentInventoryPacket, LanDiscoveryEvidenceConfidence, LanDiscoveryEvidenceKind,
    LanDiscoveryEvidenceRecord, LanDiscoveryEvidenceSource, LanPairingDeviceHardwareProfile,
    LanPairingDeviceReachability, LanPairingDeviceRef, LanPairingDiscoveryRuntimeStatus,
    LanPairingNetworkMode, LanPairingProductionDiscoveryState, LanPairingTrustState,
    LAN_PAIRING_SCHEMA_VERSION,
};

mod production_household_proof_test_support;
mod signed_discovery_relay_spine_test_support;
mod source_matrix_test_support;

#[test]
fn browser_add_device_read_model_serializes_honest_states() {
    let model = production_household_proof_test_support::browser_add_device_read_model_fixture();

    let json = serde_json::to_string(&model).expect("read model serializes");
    let value: serde_json::Value = serde_json::from_str(&json).expect("read model parses");
    production_household_proof_test_support::assert_browser_add_device_read_model_json(&value);
}

#[test]
fn signed_discovery_relay_spine_serializes_adapter_rejection_and_relay_boundaries() {
    let spine = signed_discovery_relay_spine_test_support::signed_discovery_relay_spine_fixture();

    let json = serde_json::to_value(&spine).expect("signed discovery relay spine serializes");
    signed_discovery_relay_spine_test_support::assert_signed_discovery_relay_spine_json(&json);
}

#[test]
fn lan_discovery_source_matrix_serializes_workpack_and_source_boundaries() {
    let matrix = source_matrix_test_support::source_matrix_fixture();

    let json = serde_json::to_value(&matrix).expect("LAN source matrix serializes");
    source_matrix_test_support::assert_source_matrix_json(&json);
}

#[test]
fn discovered_device_serializes_network_and_hardware_details() {
    let mut child_device = LanPairingDeviceRef::new(
        constants::lan_pairing::LOCAL_AGENT_DEVICE_ID.to_string(),
        None,
        constants::lan_pairing::LOCAL_AGENT_LABEL.to_string(),
        constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
    );
    child_device.ip_address = Some("192.168.2.42".to_string());
    child_device.mac_address = Some("54-27-1e-97-c3-31".to_string());
    child_device.hostname = Some("GAMEDEV".to_string());
    child_device.network_interface = Some("Ethernet 2".to_string());
    child_device.agent_status = Some(constants::lan_pairing::LOCAL_AGENT_STATUS.to_string());
    child_device.hardware_profile = Some(LanPairingDeviceHardwareProfile {
        manufacturer: Some("Gigabyte Technology Co., Ltd.".to_string()),
        model: Some("X570 AORUS MASTER".to_string()),
        cpu_model: Some("AMD Ryzen 9 3900X 12-Core Processor".to_string()),
        cpu_cores: Some("12 cores / 24 logical".to_string()),
        memory_total: Some("63 GiB".to_string()),
        gpu_model: Some("GeForce RTX 2070 SUPER".to_string()),
        gpu_driver: Some("456.71".to_string()),
        gpu_memory: Some("8192 MiB".to_string()),
        nvidia_smi: Some("GeForce RTX 2070 SUPER driver 456.71 8192 MiB VRAM".to_string()),
    });

    let device = LanBrowserAddDeviceDiscoveryDevice {
        schema_version: LAN_PAIRING_SCHEMA_VERSION,
        discovered_at: "2026-06-01T15:20:00.000Z".to_string(),
        child_device,
        agent_peer_id: constants::lan_pairing::PARENT_PEER_ID.to_string(),
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        network_mode: LanPairingNetworkMode::LocalNetwork,
        reachability: LanPairingDeviceReachability::Online,
        address_ref: constants::lan_pairing::ADDRESS_REF_DIRECT_WEBSOCKET.to_string(),
        discovery_status: LanPairingDiscoveryRuntimeStatus::WebsocketDirect,
        discovery_state: LanPairingProductionDiscoveryState::Discovered,
    };

    let json = serde_json::to_value(&device).expect("device serializes");
    assert_eq!(
        json["childDevice"]["ipAddress"],
        serde_json::json!("192.168.2.42")
    );
    assert_eq!(
        json["childDevice"]["hardwareProfile"]["gpuModel"],
        serde_json::json!("GeForce RTX 2070 SUPER")
    );
}

fn scan_summary() -> LanBrowserAddDeviceScanSummary {
    LanBrowserAddDeviceScanSummary {
        schema_version: LAN_PAIRING_SCHEMA_VERSION,
        source_labels: vec![constants::lan_pairing::LAN_SCAN_SOURCE_LOCAL_SERVICE.to_string()],
        scanned_device_count: 0,
        agent_device_count: 0,
        passive_device_count: 0,
        infrastructure_device_count: 0,
        unsupported_device_count: 0,
    }
}

fn canonical_child_agent_device() -> LanCanonicalHouseholdDevice {
    LanCanonicalHouseholdDevice {
        schema_version: LAN_PAIRING_SCHEMA_VERSION,
        canonical_device_id: "lan-physical-mac-54271e97c331".to_string(),
        display_name: "GAMEDEV".to_string(),
        classification: LanCanonicalHouseholdDeviceClassification::ChildAgent,
        role_badges: vec![
            LanCanonicalHouseholdDeviceRole::ChildAgent,
            LanCanonicalHouseholdDeviceRole::Portal,
            LanCanonicalHouseholdDeviceRole::ParentController,
        ],
        enrollable: true,
        discovery_state: LanPairingProductionDiscoveryState::Paired,
        trust_state: LanPairingTrustState::Paired,
        route_id: Some(constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string()),
        route_state: LanCanonicalHouseholdRouteState::LocalNetwork,
        network_mode: LanPairingNetworkMode::LocalNetwork,
        source_labels: vec![
            LanCanonicalHouseholdDeviceSource::LocalService,
            LanCanonicalHouseholdDeviceSource::NetworkNeighbor,
            LanCanonicalHouseholdDeviceSource::TrustedRegistry,
        ],
        network_identity: canonical_network_identity(),
        child_agent_inventory: Some(canonical_inventory_packet()),
        policy_target_surfaces: all_child_agent_surfaces(),
    }
}

fn canonical_network_identity() -> LanCanonicalHouseholdNetworkIdentity {
    LanCanonicalHouseholdNetworkIdentity {
        hostname: Some("GAMEDEV".to_string()),
        ip_addresses: vec!["192.168.2.42".to_string()],
        mac_address: Some("54-27-1e-97-c3-31".to_string()),
        mac_vendor: None,
        network_interfaces: vec!["Ethernet 2".to_string()],
        reachability: LanPairingDeviceReachability::Online,
        confidence: LanCanonicalHouseholdDeviceConfidence::AgentConfirmed,
        stale_at: None,
        offline_at: None,
        evidence_records: canonical_evidence_records(),
    }
}

fn canonical_evidence_records() -> Vec<LanDiscoveryEvidenceRecord> {
    vec![
        evidence_record(
            LanDiscoveryEvidenceSource::LocalService,
            LanDiscoveryEvidenceKind::IpAddress,
            "192.168.2.42",
            "ip:192.168.2.42",
            LanDiscoveryEvidenceConfidence::Confirmed,
        ),
        evidence_record(
            LanDiscoveryEvidenceSource::LocalService,
            LanDiscoveryEvidenceKind::MacAddress,
            "54-27-1e-97-c3-31",
            "mac:54271e97c331",
            LanDiscoveryEvidenceConfidence::Confirmed,
        ),
        evidence_record(
            LanDiscoveryEvidenceSource::LocalService,
            LanDiscoveryEvidenceKind::Hostname,
            "GAMEDEV",
            "hostname:gamedev",
            LanDiscoveryEvidenceConfidence::Confirmed,
        ),
        evidence_record(
            LanDiscoveryEvidenceSource::LocalService,
            LanDiscoveryEvidenceKind::Interface,
            "Ethernet 2",
            "interface:ethernet2",
            LanDiscoveryEvidenceConfidence::Confirmed,
        ),
        evidence_record(
            LanDiscoveryEvidenceSource::LocalService,
            LanDiscoveryEvidenceKind::ChildAgentPresence,
            constants::lan_pairing::LOCAL_AGENT_STATUS,
            "agent:lan-physical-mac-54271e97c331",
            LanDiscoveryEvidenceConfidence::Confirmed,
        ),
    ]
}

fn canonical_inventory_packet() -> LanChildAgentInventoryPacket {
    LanChildAgentInventoryPacket {
        device_name: "GAMEDEV".to_string(),
        platform: constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
        os: constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
        cpu_model: Some("AMD Ryzen 9 3900X 12-Core Processor".to_string()),
        cpu_cores: Some("12 cores / 24 logical".to_string()),
        memory_total: Some("63 GiB".to_string()),
        gpu_model: Some("GeForce RTX 2070 SUPER".to_string()),
        gpu_driver: Some("456.71".to_string()),
        gpu_memory: Some("8192 MiB".to_string()),
        nvidia_smi: Some("GeForce RTX 2070 SUPER driver 456.71 8192 MiB VRAM".to_string()),
        network_interfaces: vec!["Ethernet 2".to_string()],
        capabilities: vec![
            constants::lan_pairing::CHILD_AGENT_CAPABILITY_DIRECT_WEBSOCKET.to_string(),
            constants::lan_pairing::CHILD_AGENT_CAPABILITY_DEVICE_INVENTORY.to_string(),
            constants::lan_pairing::CHILD_AGENT_CAPABILITY_PAIRING_ROUTE.to_string(),
        ],
        role_state: LanCanonicalHouseholdRoleState::Implemented,
        route_state: LanCanonicalHouseholdRouteState::LocalNetwork,
        pairing_trust_state: LanPairingTrustState::Paired,
    }
}

fn evidence_record(
    source: LanDiscoveryEvidenceSource,
    evidence_kind: LanDiscoveryEvidenceKind,
    value: &str,
    merge_key: &str,
    confidence: LanDiscoveryEvidenceConfidence,
) -> LanDiscoveryEvidenceRecord {
    LanDiscoveryEvidenceRecord {
        schema_version: LAN_PAIRING_SCHEMA_VERSION,
        evidence_id: merge_key.replace(':', "-"),
        source,
        evidence_kind,
        device_id: "lan-physical-mac-54271e97c331".to_string(),
        value: value.to_string(),
        normalized_value: value.to_ascii_lowercase(),
        first_seen_at: "2026-06-01T15:20:00.000Z".to_string(),
        last_seen_at: "2026-06-01T15:20:00.000Z".to_string(),
        expires_at: None,
        confidence,
        merge_key: merge_key.to_string(),
        note: None,
    }
}

fn all_child_agent_surfaces() -> Vec<LanCanonicalHouseholdSurface> {
    vec![
        LanCanonicalHouseholdSurface::Devices,
        LanCanonicalHouseholdSurface::Policy,
        LanCanonicalHouseholdSurface::Browser,
        LanCanonicalHouseholdSurface::App,
        LanCanonicalHouseholdSurface::Screen,
        LanCanonicalHouseholdSurface::Network,
        LanCanonicalHouseholdSurface::Activity,
        LanCanonicalHouseholdSurface::Tracking,
        LanCanonicalHouseholdSurface::Ai,
    ]
}
