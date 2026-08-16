use std::time::Duration;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanBrowserAddDeviceDiscoveryDevice, LanBrowserAddDeviceScanSummary,
};

use crate::network_inventory::passive_discovery::collection::current_platform_local_neighbor_collection_summaries;

pub(super) fn scan_summary(
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
            device.discovery_status
                == ocentra_parent_agent_protocol::lan_pairing::LanPairingDiscoveryRuntimeStatus::NetworkNeighbor
                && !is_infrastructure(device)
        })
        .count() as u32;
    let unsupported_device_count =
        devices.iter().filter(|device| !has_agent(device)).count() as u32;

    LanBrowserAddDeviceScanSummary {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        source_labels: super::labels::scan_source_labels(devices),
        scanned_device_count: devices.len() as u32,
        agent_device_count,
        passive_device_count,
        infrastructure_device_count,
        unsupported_device_count,
        passive_local_neighbor_collection_summaries:
            current_platform_local_neighbor_collection_summaries(Duration::from_millis(250)),
    }
}

fn has_agent(device: &LanBrowserAddDeviceDiscoveryDevice) -> bool {
    crate::network_inventory::api::is_confirmed_agent_status(
        device.child_device.agent_status.as_deref(),
    )
}

fn is_infrastructure(device: &LanBrowserAddDeviceDiscoveryDevice) -> bool {
    device.child_device.platform == constants::lan_pairing::PLATFORM_ROUTER
}
