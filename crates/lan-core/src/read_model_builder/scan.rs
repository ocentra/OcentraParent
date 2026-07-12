#[path = "scan_builder.rs"]
mod builder;
#[path = "scan_label_mapping.rs"]
mod label_mapping;
#[path = "scan_labels.rs"]
mod labels;

use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanBrowserAddDeviceDiscoveryDevice, LanBrowserAddDeviceScanSummary,
};

pub(super) fn scan_summary(
    devices: &[LanBrowserAddDeviceDiscoveryDevice],
) -> LanBrowserAddDeviceScanSummary {
    builder::scan_summary(devices)
}
