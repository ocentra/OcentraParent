#[path = "history_time_collection.rs"]
mod collection;
#[path = "history_time_compare.rs"]
mod compare;
#[path = "history_time_device.rs"]
mod device;
#[path = "history_time_reachability.rs"]
mod reachability;

use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanBrowserAddDeviceDiscoveryDevice, LanCanonicalHouseholdDevice, LanDiscoveryEvidenceRecord,
};

pub(super) fn compact_event_identifier(value: &str) -> String {
    collection::compact_event_identifier(value)
}

pub(super) fn earliest_canonical_or_discovered_observed_at(
    devices: &[LanCanonicalHouseholdDevice],
    discovered_devices: &[LanBrowserAddDeviceDiscoveryDevice],
) -> Option<String> {
    collection::earliest_canonical_or_discovered_observed_at(devices, discovered_devices)
}

pub(super) fn latest_canonical_or_discovered_observed_at(
    devices: &[LanCanonicalHouseholdDevice],
    discovered_devices: &[LanBrowserAddDeviceDiscoveryDevice],
) -> Option<String> {
    collection::latest_canonical_or_discovered_observed_at(devices, discovered_devices)
}

pub(super) fn device_discovered_at(device: &LanCanonicalHouseholdDevice) -> Option<String> {
    device::device_discovered_at(device)
}

pub(super) fn discovered_device_observed_at(device: &LanBrowserAddDeviceDiscoveryDevice) -> String {
    device::discovered_device_observed_at(device)
}

pub(super) fn discovered_device_label(device: &LanBrowserAddDeviceDiscoveryDevice) -> String {
    device::discovered_device_label(device)
}

pub(super) fn reachability_observed_at(device: &LanCanonicalHouseholdDevice) -> Option<String> {
    reachability::reachability_observed_at(device)
}

pub(super) fn evidence_observed_at(evidence: &LanDiscoveryEvidenceRecord) -> Option<String> {
    device::evidence_observed_at(evidence)
}
