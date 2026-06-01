use ocentra_parent_agent_protocol::{
    constants, LanBrowserAddDeviceDiscoveryDevice, LanCanonicalHouseholdDeviceClassification,
    LanCanonicalHouseholdDeviceConfidence, LanCanonicalHouseholdDeviceRole,
    LanCanonicalHouseholdDeviceSource, LanCanonicalHouseholdSurface, LanPairingDeviceReachability,
    LanPairingDeviceRef, LanPairingDiscoveryRuntimeStatus, LanPairingNetworkMode,
    LanPairingProductionDiscoveryState, LanPairingTrustState, LanTrustedDeviceRegistryEntry,
};

use crate::lan_pairing_household_device_spine::canonical_household_devices;

#[test]
fn local_agent_and_neighbor_merge_into_one_canonical_physical_device() {
    let devices = canonical_household_devices(
        &[local_agent_discovery_device(), same_host_network_neighbor()],
        &[],
    );

    assert_eq!(devices.len(), 1);
    let device = &devices[0];
    assert_eq!(device.canonical_device_id, expected_test_mac_canonical_id());
    assert_eq!(
        device.classification,
        LanCanonicalHouseholdDeviceClassification::ChildAgent
    );
    assert_eq!(
        device.network_identity.confidence,
        LanCanonicalHouseholdDeviceConfidence::MacIpMatch
    );
    assert_eq!(
        device.network_identity.ip_addresses,
        vec![constants::lan_pairing::TEST_LAN_IP.to_string()]
    );
    assert!(device
        .source_labels
        .contains(&LanCanonicalHouseholdDeviceSource::LocalService));
    assert!(device
        .source_labels
        .contains(&LanCanonicalHouseholdDeviceSource::NetworkNeighbor));
    assert!(device
        .role_badges
        .contains(&LanCanonicalHouseholdDeviceRole::Portal));
    assert!(device
        .role_badges
        .contains(&LanCanonicalHouseholdDeviceRole::ParentController));
    assert!(device.child_agent_inventory.is_some());
}

#[test]
fn router_neighbor_stays_visible_but_not_enrollable() {
    let devices = canonical_household_devices(&[router_neighbor()], &[]);

    assert_eq!(devices.len(), 1);
    let device = &devices[0];
    assert_eq!(
        device.classification,
        LanCanonicalHouseholdDeviceClassification::NetworkInfrastructure
    );
    assert!(!device.enrollable);
    assert!(device.role_badges.is_empty());
    assert!(device.child_agent_inventory.is_none());
    assert_eq!(
        device.policy_target_surfaces,
        vec![
            LanCanonicalHouseholdSurface::Devices,
            LanCanonicalHouseholdSurface::Network
        ]
    );
}

#[test]
fn trusted_registry_device_remains_available_to_product_target_surfaces() {
    let devices = canonical_household_devices(&[], &[trusted_registry_entry()]);

    assert_eq!(devices.len(), 1);
    let device = &devices[0];
    assert_eq!(device.trust_state, LanPairingTrustState::Paired);
    assert_eq!(
        device.discovery_state,
        LanPairingProductionDiscoveryState::Paired
    );
    for surface in [
        LanCanonicalHouseholdSurface::Policy,
        LanCanonicalHouseholdSurface::Activity,
        LanCanonicalHouseholdSurface::Network,
        LanCanonicalHouseholdSurface::Tracking,
        LanCanonicalHouseholdSurface::Ai,
    ] {
        assert!(device.policy_target_surfaces.contains(&surface));
    }
}

fn local_agent_discovery_device() -> LanBrowserAddDeviceDiscoveryDevice {
    let mut device = LanPairingDeviceRef::new(
        constants::lan_pairing::LOCAL_AGENT_DEVICE_ID.to_string(),
        None,
        constants::lan_pairing::LOCAL_AGENT_LABEL.to_string(),
        constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
    );
    device.ip_address = Some(constants::lan_pairing::TEST_LAN_IP.to_string());
    device.mac_address = Some(constants::lan_pairing::TEST_LAN_MAC.to_string());
    device.hostname = Some(constants::lan_pairing::TEST_HOSTNAME.to_string());
    device.network_interface = Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string());
    device.agent_status = Some(constants::lan_pairing::LOCAL_AGENT_STATUS.to_string());
    discovery_device(device, LanPairingDiscoveryRuntimeStatus::WebsocketDirect)
}

fn same_host_network_neighbor() -> LanBrowserAddDeviceDiscoveryDevice {
    let mut device = network_neighbor_device_ref(
        constants::lan_pairing::TEST_LAN_IP,
        constants::lan_pairing::TEST_LAN_MAC,
        constants::lan_pairing::PLATFORM_UNKNOWN,
    );
    device.hostname = Some(constants::lan_pairing::TEST_HOSTNAME.to_string());
    discovery_device(device, LanPairingDiscoveryRuntimeStatus::NetworkNeighbor)
}

fn router_neighbor() -> LanBrowserAddDeviceDiscoveryDevice {
    discovery_device(
        network_neighbor_device_ref(
            constants::lan_pairing::TEST_ROUTER_IP,
            constants::lan_pairing::TEST_ROUTER_MAC,
            constants::lan_pairing::PLATFORM_ROUTER,
        ),
        LanPairingDiscoveryRuntimeStatus::NetworkNeighbor,
    )
}

fn network_neighbor_device_ref(ip: &str, mac: &str, platform: &str) -> LanPairingDeviceRef {
    let mut device = LanPairingDeviceRef::new(
        expected_device_id_from_mac(mac),
        None,
        label_for_ip(ip),
        platform.to_string(),
    );
    device.ip_address = Some(ip.to_string());
    device.mac_address = Some(mac.to_string());
    device.hostname = Some(constants::lan_pairing::NETWORK_NEIGHBOR_UNKNOWN_HOSTNAME.to_string());
    device.network_interface = Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string());
    device
}

fn discovery_device(
    child_device: LanPairingDeviceRef,
    discovery_status: LanPairingDiscoveryRuntimeStatus,
) -> LanBrowserAddDeviceDiscoveryDevice {
    LanBrowserAddDeviceDiscoveryDevice {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        discovered_at: constants::lan_pairing::OBSERVED_AT.to_string(),
        child_device,
        agent_peer_id: constants::lan_pairing::PARENT_PEER_ID.to_string(),
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        network_mode: LanPairingNetworkMode::LocalNetwork,
        reachability: LanPairingDeviceReachability::Online,
        address_ref: constants::lan_pairing::ADDRESS_REF_DIRECT_WEBSOCKET.to_string(),
        discovery_status,
        discovery_state: LanPairingProductionDiscoveryState::Discovered,
    }
}

fn trusted_registry_entry() -> LanTrustedDeviceRegistryEntry {
    LanTrustedDeviceRegistryEntry {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        pairing_id: constants::lan_pairing::PAIRING_ID.to_string(),
        child_device: trusted_child_device(),
        parent_device: parent_device(),
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        origin: constants::lan_pairing::ALLOWED_ORIGIN.to_string(),
        proof_digest: constants::lan_pairing::PROOF_DIGEST.to_string(),
        trust_state: LanPairingTrustState::Paired,
        trusted_at: constants::lan_pairing::OBSERVED_AT.to_string(),
        expires_at: constants::lan_pairing::EXPIRES_AT.to_string(),
        revoked_at: None,
    }
}

fn trusted_child_device() -> LanPairingDeviceRef {
    LanPairingDeviceRef::new(
        constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
        None,
        constants::lan_pairing::LOCAL_AGENT_LABEL.to_string(),
        constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
    )
}

fn parent_device() -> LanPairingDeviceRef {
    LanPairingDeviceRef::new(
        constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
        None,
        constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
        constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
    )
}

fn expected_test_mac_canonical_id() -> String {
    let mut id = String::from(constants::lan_pairing::CANONICAL_DEVICE_MAC_PREFIX);
    id.push_str(&compact(constants::lan_pairing::TEST_LAN_MAC));
    id
}

fn expected_device_id_from_mac(mac: &str) -> String {
    let mut id = String::from(constants::lan_pairing::NETWORK_NEIGHBOR_DEVICE_PREFIX);
    id.push_str(&compact(mac));
    id
}

fn label_for_ip(ip: &str) -> String {
    let mut label = String::from(constants::lan_pairing::NETWORK_NEIGHBOR_LABEL_PREFIX);
    label.push_str(ip);
    label
}

fn compact(value: &str) -> String {
    value
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}
