use ocentra_parent_agent_protocol::{
    constants, LanBrowserAddDeviceDiscoveryDevice, LanHouseholdDeviceActionKind,
    LanHouseholdDeviceDecision, LanPairingDeviceReachability, LanPairingDeviceRef,
    LanPairingDiscoveryRuntimeStatus, LanPairingNetworkMode, LanPairingProductionDiscoveryState,
    LanPairingTrustState, LanTrustedDeviceRegistryEntry,
};

pub(crate) fn local_agent_discovery_device() -> LanBrowserAddDeviceDiscoveryDevice {
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

pub(crate) fn same_host_network_neighbor() -> LanBrowserAddDeviceDiscoveryDevice {
    let mut device = network_neighbor_device_ref(
        constants::lan_pairing::TEST_LAN_IP,
        constants::lan_pairing::TEST_LAN_MAC,
        constants::lan_pairing::PLATFORM_UNKNOWN,
    );
    device.hostname = Some(constants::lan_pairing::TEST_HOSTNAME.to_string());
    discovery_device(device, LanPairingDiscoveryRuntimeStatus::NetworkNeighbor)
}

pub(crate) fn router_neighbor() -> LanBrowserAddDeviceDiscoveryDevice {
    discovery_device(
        network_neighbor_device_ref(
            constants::lan_pairing::TEST_ROUTER_IP,
            constants::lan_pairing::TEST_ROUTER_MAC,
            constants::lan_pairing::PLATFORM_ROUTER,
        ),
        LanPairingDiscoveryRuntimeStatus::NetworkNeighbor,
    )
}

pub(crate) fn ip_only_neighbor(
    ip: &str,
    hostname: &str,
    platform: &str,
    device_id_seed: &str,
) -> LanBrowserAddDeviceDiscoveryDevice {
    let mut device = LanPairingDeviceRef::new(
        device_id_seed.to_string(),
        None,
        label_for_ip(ip),
        platform.to_string(),
    );
    device.ip_address = Some(ip.to_string());
    device.hostname = Some(hostname.to_string());
    discovery_device(device, LanPairingDiscoveryRuntimeStatus::NetworkNeighbor)
}

pub(crate) fn trusted_registry_entry() -> LanTrustedDeviceRegistryEntry {
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

pub(crate) fn household_decision(
    action_kind: LanHouseholdDeviceActionKind,
    canonical_device_id: &str,
    display_name: Option<&str>,
) -> LanHouseholdDeviceDecision {
    LanHouseholdDeviceDecision {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        action_id: constants::lan_pairing::HOUSEHOLD_ACTION_ID.to_string(),
        action_kind,
        canonical_device_id: canonical_device_id.to_string(),
        child_profile_id: None,
        display_name: display_name.map(ToOwned::to_owned),
        parent_actor_id: constants::lan_pairing::PARENT_ACTOR_ID.to_string(),
        decided_at: constants::lan_pairing::OBSERVED_AT.to_string(),
        revoked_at: None,
    }
}

pub(crate) fn household_restore_decision(canonical_device_id: &str) -> LanHouseholdDeviceDecision {
    LanHouseholdDeviceDecision {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        action_id: constants::lan_pairing::HOUSEHOLD_RESTORE_ACTION_ID.to_string(),
        action_kind: LanHouseholdDeviceActionKind::Restore,
        canonical_device_id: canonical_device_id.to_string(),
        child_profile_id: None,
        display_name: None,
        parent_actor_id: constants::lan_pairing::PARENT_ACTOR_ID.to_string(),
        decided_at: constants::lan_pairing::OBSERVED_AT.to_string(),
        revoked_at: None,
    }
}

pub(crate) fn expected_test_mac_canonical_id() -> String {
    let mut id = String::from(constants::lan_pairing::CANONICAL_DEVICE_MAC_PREFIX);
    id.push_str(&compact(constants::lan_pairing::TEST_LAN_MAC));
    id
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
