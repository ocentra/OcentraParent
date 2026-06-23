use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDiscoveryRuntimeStatus;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceDiscoveryDevice;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceScanSummary;

use crate::lan_network_inventory;

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
    network_device: &lan_network_inventory::LanNetworkInventoryDevice,
) -> bool {
    same_device_text(
        child_device.mac_address.as_deref(),
        Some(&network_device.mac_address),
    ) || same_device_text(
        child_device.ip_address.as_deref(),
        Some(&network_device.ip_address),
    )
}

pub(crate) fn scan_summary(
    devices: &[LanBrowserAddDeviceDiscoveryDevice],
) -> LanBrowserAddDeviceScanSummary {
    let agent_device_count = devices.iter().filter(|device| has_agent(device)).count() as u32;
    let infrastructure_device_count = devices
        .iter()
        .filter(|device| is_infrastructure(device))
        .count() as u32;
    let passive_device_count = devices
        .iter()
        .filter(|device| {
            device.discovery_status == LanPairingDiscoveryRuntimeStatus::NetworkNeighbor
                && !is_infrastructure(device)
        })
        .count() as u32;
    let unsupported_device_count =
        devices.iter().filter(|device| !has_agent(device)).count() as u32;

    LanBrowserAddDeviceScanSummary {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        source_labels: scan_source_labels(devices),
        scanned_device_count: devices.len() as u32,
        agent_device_count,
        passive_device_count,
        infrastructure_device_count,
        unsupported_device_count,
    }
}

fn same_device_text(left: Option<&str>, right: Option<&str>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.trim().eq_ignore_ascii_case(right.trim()),
        _ => false,
    }
}

fn scan_source_labels(devices: &[LanBrowserAddDeviceDiscoveryDevice]) -> Vec<String> {
    let mut labels = vec![constants::lan_pairing::LAN_SCAN_SOURCE_LOCAL_SERVICE.to_string()];
    if devices
        .iter()
        .any(|device| device.discovery_status == LanPairingDiscoveryRuntimeStatus::NetworkNeighbor)
    {
        labels.push(constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string());
    }
    labels
}

fn has_agent(device: &LanBrowserAddDeviceDiscoveryDevice) -> bool {
    device.child_device.agent_status.is_some()
}

fn is_infrastructure(device: &LanBrowserAddDeviceDiscoveryDevice) -> bool {
    device.child_device.platform == constants::lan_pairing::PLATFORM_ROUTER
}
