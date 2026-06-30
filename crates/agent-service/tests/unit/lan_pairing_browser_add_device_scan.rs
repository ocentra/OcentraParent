use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingDeviceRef,
};

use super::*;

#[test]
fn same_physical_network_device_ignores_empty_mac_values() {
    let mut child_device = LanPairingDeviceRef::new(
        "local-child".to_string(),
        None,
        "Local Child".to_string(),
        constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
    );
    child_device.mac_address = Some(String::new());
    child_device.ip_address = Some("192.168.2.10".to_string());

    let network_device = LanNetworkInventoryDevice {
        device_id: "ssdp-device".to_string(),
        label: "Living Room TV".to_string(),
        platform: constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
        ip_address: "192.168.2.11".to_string(),
        mac_address: String::new(),
        hostname: None,
        network_interface: None,
        observed_at: constants::lan_pairing::OBSERVED_AT.to_string(),
        reachability: LanPairingDeviceReachability::Online,
        agent_status: None,
        scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_SSDP_UPNP.to_string()],
        used_previous_scan_hint: false,
        service_identity_probe_evidence: Vec::new(),
    };

    assert!(!same_physical_network_device(
        &child_device,
        &network_device
    ));
}
