use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingDeviceRef,
};

use super::LanNetworkInventoryDevice;

pub(super) fn trusted_device_matches_network_identity(
    trusted_device: &LanPairingDeviceRef,
    network_mac_address: &str,
    network_ip_address: &str,
) -> bool {
    trusted_mac_address(trusted_device)
        .filter(|_| !network_mac_address.trim().is_empty())
        .map(|trusted_mac_address| {
            trusted_mac_address.eq_ignore_ascii_case(network_mac_address.trim())
        })
        .unwrap_or_else(|| trusted_ip_address_matches(trusted_device, network_ip_address))
}

pub(super) fn should_probe_service_identity(
    device: &LanNetworkInventoryDevice,
    probe_suppression_devices: &[LanPairingDeviceRef],
    selected_interface: &str,
) -> bool {
    device_is_on_selected_interface(device, selected_interface)
        && device.platform != constants::lan_pairing::PLATFORM_ROUTER
        && device.agent_status.is_none()
        && !probe_suppression_devices.iter().any(|trusted_device| {
            trusted_device_matches_network_identity(
                trusted_device,
                &device.mac_address,
                &device.ip_address,
            )
        })
        && device_has_probeable_reachability(device)
}

pub(super) fn device_is_on_selected_interface(
    device: &LanNetworkInventoryDevice,
    selected_interface: &str,
) -> bool {
    device
        .network_interface
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|device_interface| device_interface.eq_ignore_ascii_case(selected_interface))
        .unwrap_or(false)
}

fn trusted_mac_address(device: &LanPairingDeviceRef) -> Option<&str> {
    device
        .mac_address
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
}

fn trusted_ip_address_matches(device: &LanPairingDeviceRef, network_ip_address: &str) -> bool {
    device
        .ip_address
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|trusted_ip_address| {
            trusted_ip_address.eq_ignore_ascii_case(network_ip_address.trim())
        })
        .unwrap_or(false)
}

fn device_has_probeable_reachability(device: &LanNetworkInventoryDevice) -> bool {
    matches!(
        device.reachability,
        LanPairingDeviceReachability::Online | LanPairingDeviceReachability::Stale
    )
}
