use ocentra_lan_core::network_inventory::LanNetworkInventoryDevice;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceDiscoveryDevice;

pub(crate) fn push_if_absent(
    devices: &mut Vec<LanBrowserAddDeviceDiscoveryDevice>,
    device: LanBrowserAddDeviceDiscoveryDevice,
) {
    if devices
        .iter()
        .any(|existing| existing.child_device.device_id == device.child_device.device_id)
    {
        return;
    }
    devices.push(device);
}

pub(crate) fn same_physical_network_device(
    child_device: &LanPairingDeviceRef,
    network_device: &LanNetworkInventoryDevice,
) -> bool {
    let same_mac_address = match (
        child_device.mac_address.as_deref(),
        Some(network_device.mac_address.as_str()),
    ) {
        (Some(left), Some(right)) => {
            let left = left.trim();
            let right = right.trim();
            !left.is_empty() && !right.is_empty() && left.eq_ignore_ascii_case(right)
        }
        _ => false,
    };

    if same_mac_address {
        return true;
    }

    match (
        child_device.ip_address.as_deref(),
        Some(network_device.ip_address.as_str()),
    ) {
        (Some(left), Some(right)) => {
            let left = left.trim();
            let right = right.trim();
            !left.is_empty() && !right.is_empty() && left.eq_ignore_ascii_case(right)
        }
        _ => false,
    }
}
