use crate::{
    constants, LanBrowserAddDeviceDiscoveryDevice, LanBrowserAddDeviceReadModel,
    LanBrowserAddDeviceScanSummary, LanCanonicalHouseholdDevice,
    LanCanonicalHouseholdDeviceClassification, LanCanonicalHouseholdDeviceConfidence,
    LanCanonicalHouseholdDeviceRole, LanCanonicalHouseholdDeviceSource,
    LanCanonicalHouseholdNetworkIdentity, LanCanonicalHouseholdRoleState,
    LanCanonicalHouseholdRouteState, LanCanonicalHouseholdSurface, LanChildAgentInventoryPacket,
    LanPairingDeviceHardwareProfile, LanPairingDeviceReachability, LanPairingDeviceRef,
    LanPairingDiscoveryRuntimeStatus, LanPairingDiscoverySource, LanPairingNetworkMode,
    LanPairingParentAuthority, LanPairingProductionDiscoveryState, LanPairingTrustState,
    LanSelectedDeviceReadiness, LAN_PAIRING_SCHEMA_VERSION,
};

#[test]
fn browser_add_device_read_model_serializes_honest_states() {
    let model = LanBrowserAddDeviceReadModel {
        schema_version: LAN_PAIRING_SCHEMA_VERSION,
        generated_at: "2026-06-01T15:20:00.000Z".to_string(),
        discovery_source: LanPairingDiscoverySource::LocalService,
        add_device_state: LanPairingProductionDiscoveryState::Pending,
        local_service_discovery_state: LanPairingProductionDiscoveryState::Pending,
        physical_household_lan_state: LanPairingProductionDiscoveryState::ManualRequired,
        cloud_relay_state: LanPairingProductionDiscoveryState::Unavailable,
        scan_summary: scan_summary(),
        discovered_devices: Vec::new(),
        canonical_household_devices: vec![canonical_child_agent_device()],
        pairing_requests: Vec::new(),
        trusted_device_registry: Vec::new(),
        trusted_device_ids: Vec::new(),
        revoked_device_ids: Vec::new(),
        selected_device_readiness: LanSelectedDeviceReadiness {
            schema_version: LAN_PAIRING_SCHEMA_VERSION,
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
        route_requirement_labels: vec![
            constants::lan_pairing::ROUTE_REQUIREMENT_ALLOWED_ORIGIN.to_string()
        ],
        audit_check_labels: vec![
            constants::value::LAN_REASON_WRONG_ORIGIN.to_string(),
            constants::value::LAN_REASON_REPLAYED.to_string(),
        ],
        honest_non_claims: vec![
            constants::value::LAN_NON_CLAIM_CLOUD_RELAY_NOT_IMPLEMENTED.to_string()
        ],
    };

    let json = serde_json::to_string(&model).expect("read model serializes");
    let value: serde_json::Value = serde_json::from_str(&json).expect("read model parses");
    assert_eq!(
        value[constants::field::LAN_PHYSICAL_HOUSEHOLD_LAN_STATE],
        serde_json::json!(constants::value::LAN_DISCOVERY_STATE_MANUAL_REQUIRED)
    );
    assert_eq!(
        value[constants::field::LAN_CLOUD_RELAY_STATE],
        serde_json::json!(constants::value::LAN_DISCOVERY_STATE_UNAVAILABLE)
    );
    assert_eq!(
        value["selectedDeviceReadiness"]["readyForControl"],
        serde_json::json!(false)
    );
    assert_eq!(
        value[constants::field::LAN_SCAN_SUMMARY][constants::field::SOURCE_LABELS],
        serde_json::json!([constants::lan_pairing::LAN_SCAN_SOURCE_LOCAL_SERVICE])
    );
    assert_eq!(value["trustedDeviceRegistry"], serde_json::json!([]));
    assert_eq!(
        value["canonicalHouseholdDevices"][0]["policyTargetSurfaces"],
        serde_json::json!([
            "devices", "policy", "browser", "app", "screen", "network", "activity", "tracking",
            "ai"
        ])
    );
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
        network_identity: LanCanonicalHouseholdNetworkIdentity {
            hostname: Some("GAMEDEV".to_string()),
            ip_addresses: vec!["192.168.2.42".to_string()],
            mac_address: Some("54-27-1e-97-c3-31".to_string()),
            mac_vendor: None,
            network_interfaces: vec!["Ethernet 2".to_string()],
            reachability: LanPairingDeviceReachability::Online,
            confidence: LanCanonicalHouseholdDeviceConfidence::AgentConfirmed,
            stale_at: None,
            offline_at: None,
        },
        child_agent_inventory: Some(LanChildAgentInventoryPacket {
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
        }),
        policy_target_surfaces: all_child_agent_surfaces(),
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
