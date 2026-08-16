use super::*;

#[test]
fn trusted_device_suppresses_service_identity_probe() {
    let device = LanNetworkInventoryDevice {
        device_id: "lan-device-1".to_string(),
        label: constants::lan_pairing::TEST_HOSTNAME.to_string(),
        platform: constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
        ip_address: constants::lan_pairing::TEST_LAN_IP.to_string(),
        mac_address: constants::lan_pairing::TEST_LAN_MAC.to_string(),
        hostname: Some(constants::lan_pairing::TEST_HOSTNAME.to_string()),
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
        reachability: LanPairingDeviceReachability::Online,
        agent_status: None,
        scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
        observed_at: String::new(),
        used_previous_scan_hint: false,
        service_identity_probe_evidence: Vec::new(),
    };
    let mut trusted_device = LanPairingDeviceRef::new(
        "trusted-child".to_string(),
        None,
        constants::lan_pairing::TEST_HOSTNAME.to_string(),
        constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
    );
    trusted_device.mac_address = Some(constants::lan_pairing::TEST_LAN_MAC.to_string());

    assert!(!should_probe_service_identity(
        &device,
        &[trusted_device],
        constants::lan_pairing::TEST_NETWORK_INTERFACE,
    ));
}

#[test]
fn trusted_device_without_mac_can_still_match_by_ip() {
    let device = LanNetworkInventoryDevice {
        device_id: "lan-device-1".to_string(),
        label: constants::lan_pairing::TEST_HOSTNAME.to_string(),
        platform: constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
        ip_address: constants::lan_pairing::TEST_LAN_IP.to_string(),
        mac_address: constants::lan_pairing::TEST_LAN_MAC.to_string(),
        hostname: Some(constants::lan_pairing::TEST_HOSTNAME.to_string()),
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
        reachability: LanPairingDeviceReachability::Online,
        agent_status: None,
        scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
        observed_at: String::new(),
        used_previous_scan_hint: false,
        service_identity_probe_evidence: Vec::new(),
    };
    let trusted_device = trusted_device(
        "",
        Some(constants::lan_pairing::TEST_LAN_IP),
        Some(constants::lan_pairing::TEST_HOSTNAME),
        constants::lan_pairing::TEST_HOSTNAME,
        constants::lan_pairing::PLATFORM_WINDOWS,
    );

    assert!(!should_probe_service_identity(
        &device,
        &[trusted_device],
        constants::lan_pairing::TEST_NETWORK_INTERFACE,
    ));
}

#[test]
fn trusted_device_with_mac_can_match_by_ip_when_protocol_source_has_no_mac() {
    let trusted_device = trusted_device(
        constants::lan_pairing::TEST_LAN_MAC,
        Some(constants::lan_pairing::TEST_LAN_IP),
        Some(constants::lan_pairing::TEST_HOSTNAME),
        constants::lan_pairing::TEST_HOSTNAME,
        constants::lan_pairing::PLATFORM_WINDOWS,
    );

    assert!(trusted_device_matches_network_identity(
        &trusted_device,
        "",
        constants::lan_pairing::TEST_LAN_IP,
    ));
}

#[test]
fn router_device_never_uses_service_identity_probe() {
    let device = LanNetworkInventoryDevice {
        device_id: "lan-router-1".to_string(),
        label: "Home Router".to_string(),
        platform: constants::lan_pairing::PLATFORM_ROUTER.to_string(),
        ip_address: constants::lan_pairing::TEST_ROUTER_IP.to_string(),
        mac_address: constants::lan_pairing::TEST_ROUTER_MAC.to_string(),
        hostname: Some("home-router".to_string()),
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
        reachability: LanPairingDeviceReachability::Online,
        agent_status: None,
        scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
        observed_at: String::new(),
        used_previous_scan_hint: false,
        service_identity_probe_evidence: Vec::new(),
    };

    assert!(!should_probe_service_identity(
        &device,
        &[],
        constants::lan_pairing::TEST_NETWORK_INTERFACE,
    ));
}

#[test]
fn trusted_device_mac_mismatch_does_not_suppress_probe_on_reused_ip() {
    let device = LanNetworkInventoryDevice {
        device_id: "lan-device-1".to_string(),
        label: constants::lan_pairing::TEST_HOSTNAME.to_string(),
        platform: constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
        ip_address: constants::lan_pairing::TEST_LAN_IP.to_string(),
        mac_address: constants::lan_pairing::TEST_LAN_MAC.to_string(),
        hostname: Some(constants::lan_pairing::TEST_HOSTNAME.to_string()),
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
        reachability: LanPairingDeviceReachability::Online,
        agent_status: None,
        scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
        observed_at: String::new(),
        used_previous_scan_hint: false,
        service_identity_probe_evidence: Vec::new(),
    };
    let trusted_device = trusted_device(
        "AA-BB-CC-DD-EE-FF",
        Some(constants::lan_pairing::TEST_LAN_IP),
        Some(constants::lan_pairing::TEST_HOSTNAME),
        constants::lan_pairing::TEST_HOSTNAME,
        constants::lan_pairing::PLATFORM_WINDOWS,
    );

    assert!(should_probe_service_identity(
        &device,
        &[trusted_device],
        constants::lan_pairing::TEST_NETWORK_INTERFACE,
    ));
}

#[test]
fn service_identity_probe_requires_selected_interface_match() {
    let device = LanNetworkInventoryDevice {
        device_id: "lan-device-1".to_string(),
        label: constants::lan_pairing::TEST_HOSTNAME.to_string(),
        platform: constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
        ip_address: constants::lan_pairing::TEST_LAN_IP.to_string(),
        mac_address: constants::lan_pairing::TEST_LAN_MAC.to_string(),
        hostname: Some(constants::lan_pairing::TEST_HOSTNAME.to_string()),
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
        reachability: LanPairingDeviceReachability::Online,
        agent_status: None,
        scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
        observed_at: String::new(),
        used_previous_scan_hint: false,
        service_identity_probe_evidence: Vec::new(),
    };

    assert!(should_probe_service_identity(
        &device,
        &[],
        constants::lan_pairing::TEST_NETWORK_INTERFACE,
    ));
    assert!(!should_probe_service_identity(&device, &[], "Ethernet 7",));
}
