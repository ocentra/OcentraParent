use ocentra_parent_agent_protocol::{
    constants, LanBrowserAddDeviceDiscoveryDevice, LanCanonicalHouseholdDeviceClassification,
    LanCanonicalHouseholdDeviceConfidence, LanCanonicalHouseholdDeviceRole,
    LanCanonicalHouseholdDeviceSource, LanCanonicalHouseholdSurface, LanPairingDeviceHardwareProfile,
    LanPairingDeviceReachability, LanPairingDeviceRef, LanPairingDiscoveryRuntimeStatus,
    LanPairingNetworkMode, LanPairingProductionDiscoveryState, LanPairingTrustState,
    LanTrustedDeviceRegistryEntry,
};

use super::canonical_household_devices;

#[test]
fn canonical_household_devices_merges_child_agent_sources_and_keeps_router_non_enrollable() {
    let devices = canonical_household_devices(
        &[
            trusted_child_discovery(),
            local_agent_discovery(),
            lan_neighbor_child_discovery(None),
            router_discovery(),
        ],
        &[trusted_registry_entry()],
    );

    assert_eq!(devices.len(), 2);

    let child_agent = devices
        .iter()
        .find(|device| device.canonical_device_id == "lan-physical-mac-54271e97c331")
        .expect("merged child-agent row");
    assert_eq!(child_agent.display_name, constants::lan_pairing::TEST_HOSTNAME);
    assert_eq!(
        child_agent.role_badges,
        vec![
            LanCanonicalHouseholdDeviceRole::ChildAgent,
            LanCanonicalHouseholdDeviceRole::Portal,
            LanCanonicalHouseholdDeviceRole::ParentController,
        ]
    );
    assert!(child_agent.enrollable);
    assert_eq!(
        child_agent.source_labels,
        vec![
            LanCanonicalHouseholdDeviceSource::LocalService,
            LanCanonicalHouseholdDeviceSource::NetworkNeighbor,
            LanCanonicalHouseholdDeviceSource::TrustedRegistry,
        ]
    );
    assert_eq!(
        child_agent.network_identity.confidence,
        LanCanonicalHouseholdDeviceConfidence::MacIpMatch
    );
    assert_eq!(
        child_agent.network_identity.hostname.as_deref(),
        Some(constants::lan_pairing::TEST_HOSTNAME)
    );
    assert!(child_agent.child_agent_inventory.is_some());
    assert_eq!(
        child_agent.policy_target_surfaces,
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
    );

    let router = devices
        .iter()
        .find(|device| {
            device.classification == LanCanonicalHouseholdDeviceClassification::NetworkInfrastructure
        })
        .expect("visible router row");
    assert!(!router.enrollable);
    assert_eq!(router.display_name, "LAN 192.168.2.1");
    assert_eq!(router.network_identity.hostname, None);
    assert_eq!(
        router.policy_target_surfaces,
        vec![LanCanonicalHouseholdSurface::Devices, LanCanonicalHouseholdSurface::Network]
    );
}

#[test]
fn canonical_household_devices_uses_hostname_evidence_for_neighbor_labels() {
    let devices = canonical_household_devices(&[lan_neighbor_child_discovery(Some("NAS"))], &[]);

    assert_eq!(devices.len(), 1);
    let device = &devices[0];
    assert_eq!(device.display_name, "NAS");
    assert_eq!(device.network_identity.hostname.as_deref(), Some("NAS"));
    assert_eq!(
        device.network_identity.confidence,
        LanCanonicalHouseholdDeviceConfidence::NetworkNeighbor
    );
    assert!(!device.enrollable);
}

fn trusted_child_discovery() -> LanBrowserAddDeviceDiscoveryDevice {
    discovery(child_device_ref("Mia Windows PC", None, None), LanPairingDiscoveryRuntimeStatus::WebsocketDirect)
}

fn local_agent_discovery() -> LanBrowserAddDeviceDiscoveryDevice {
    discovery(
        child_device_ref(
            constants::lan_pairing::LOCAL_AGENT_LABEL,
            Some(constants::lan_pairing::TEST_HOSTNAME),
            Some(LanPairingDeviceHardwareProfile {
                manufacturer: Some("Gigabyte Technology Co., Ltd.".to_string()),
                model: Some("X570 AORUS MASTER".to_string()),
                cpu_model: Some("AMD Ryzen 9 3900X 12-Core Processor".to_string()),
                cpu_cores: Some("12 cores / 24 logical".to_string()),
                memory_total: Some("63 GiB".to_string()),
                gpu_model: Some("GeForce RTX 2070 SUPER".to_string()),
                gpu_driver: Some("456.71".to_string()),
                gpu_memory: Some("8192 MiB".to_string()),
                nvidia_smi: Some("GeForce RTX 2070 SUPER driver 456.71 8192 MiB VRAM".to_string()),
            }),
        ),
        LanPairingDiscoveryRuntimeStatus::WebsocketDirect,
    )
}

fn lan_neighbor_child_discovery(hostname: Option<&str>) -> LanBrowserAddDeviceDiscoveryDevice {
    discovery(
        child_device_ref("LAN 192.168.2.42", hostname, None),
        LanPairingDiscoveryRuntimeStatus::NetworkNeighbor,
    )
}

fn router_discovery() -> LanBrowserAddDeviceDiscoveryDevice {
    discovery(router_device_ref(), LanPairingDiscoveryRuntimeStatus::NetworkNeighbor)
}

fn discovery(
    child_device: LanPairingDeviceRef,
    discovery_status: LanPairingDiscoveryRuntimeStatus,
) -> LanBrowserAddDeviceDiscoveryDevice {
    LanBrowserAddDeviceDiscoveryDevice {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        discovered_at: constants::lan_pairing::ISSUED_AT.to_string(),
        child_device,
        agent_peer_id: constants::peer::PORTAL_DEV.to_string(),
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        network_mode: LanPairingNetworkMode::LocalNetwork,
        reachability: LanPairingDeviceReachability::Online,
        address_ref: address_ref_for(discovery_status.clone()),
        discovery_status,
        discovery_state: LanPairingProductionDiscoveryState::Discovered,
    }
}

fn address_ref_for(discovery_status: LanPairingDiscoveryRuntimeStatus) -> String {
    match discovery_status {
        LanPairingDiscoveryRuntimeStatus::WebsocketDirect => {
            constants::lan_pairing::ADDRESS_REF_DIRECT_WEBSOCKET.to_string()
        }
        LanPairingDiscoveryRuntimeStatus::NetworkNeighbor => {
            constants::lan_pairing::ADDRESS_REF_NETWORK_NEIGHBOR.to_string()
        }
        LanPairingDiscoveryRuntimeStatus::PlannedUnsupported => {
            constants::lan_pairing::ADDRESS_REF_UNPROVEN.to_string()
        }
    }
}

fn trusted_registry_entry() -> LanTrustedDeviceRegistryEntry {
    LanTrustedDeviceRegistryEntry {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        pairing_id: constants::lan_pairing::PAIRING_ID.to_string(),
        child_device: child_device_ref("Mia Windows PC", None, None),
        parent_device: LanPairingDeviceRef::new(
            constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
            None,
            "Parent Windows PC".to_string(),
            constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
        ),
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        origin: constants::lan_pairing::ALLOWED_ORIGIN.to_string(),
        proof_digest: constants::lan_pairing::PROOF_DIGEST.to_string(),
        trust_state: LanPairingTrustState::Paired,
        trusted_at: constants::lan_pairing::ISSUED_AT.to_string(),
        expires_at: constants::lan_pairing::EXPIRES_AT.to_string(),
        revoked_at: None,
    }
}

fn child_device_ref(
    label: &str,
    hostname: Option<&str>,
    hardware_profile: Option<LanPairingDeviceHardwareProfile>,
) -> LanPairingDeviceRef {
    let mut device = LanPairingDeviceRef::new(
        constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
        None,
        label.to_string(),
        constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
    );
    device.ip_address = Some(constants::lan_pairing::TEST_LAN_IP.to_string());
    device.mac_address = Some(constants::lan_pairing::TEST_LAN_MAC.to_string());
    device.hostname = hostname.map(str::to_string);
    device.network_interface = Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string());
    device.agent_status = hardware_profile
        .as_ref()
        .map(|_| constants::lan_pairing::LOCAL_AGENT_STATUS.to_string());
    device.hardware_profile = hardware_profile;
    device
}

fn router_device_ref() -> LanPairingDeviceRef {
    let mut device = LanPairingDeviceRef::new(
        "lan-device-001122334455".to_string(),
        None,
        "LAN 192.168.2.1".to_string(),
        constants::lan_pairing::PLATFORM_ROUTER.to_string(),
    );
    device.ip_address = Some(constants::lan_pairing::TEST_ROUTER_IP.to_string());
    device.mac_address = Some(constants::lan_pairing::TEST_ROUTER_MAC.to_string());
    device.network_interface = Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string());
    device
}
