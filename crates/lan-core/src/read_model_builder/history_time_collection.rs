use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanBrowserAddDeviceDiscoveryDevice, LanCanonicalHouseholdDevice,
};

pub(super) fn compact_event_identifier(value: &str) -> String {
    value
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

pub(super) fn earliest_canonical_or_discovered_observed_at(
    devices: &[LanCanonicalHouseholdDevice],
    discovered_devices: &[LanBrowserAddDeviceDiscoveryDevice],
) -> Option<String> {
    let canonical = earliest_device_observed_at(devices);
    let discovered = earliest_discovered_device_observed_at(discovered_devices);
    super::compare::earliest_timestamp(canonical.as_deref(), discovered.as_deref())
}

pub(super) fn latest_canonical_or_discovered_observed_at(
    devices: &[LanCanonicalHouseholdDevice],
    discovered_devices: &[LanBrowserAddDeviceDiscoveryDevice],
) -> Option<String> {
    let canonical = latest_device_observed_at(devices);
    let discovered = latest_discovered_device_observed_at(discovered_devices);
    super::compare::latest_timestamp(canonical.as_deref(), discovered.as_deref())
}

fn earliest_device_observed_at(devices: &[LanCanonicalHouseholdDevice]) -> Option<String> {
    let mut earliest: Option<String> = None;
    for device in devices {
        earliest = super::compare::earliest_timestamp(
            earliest.as_deref(),
            super::device::device_discovered_at(device).as_deref(),
        );
        for evidence in &device.network_identity.evidence_records {
            earliest = super::compare::earliest_timestamp(
                earliest.as_deref(),
                super::device::evidence_observed_at(evidence).as_deref(),
            );
        }
    }
    earliest
}

fn latest_device_observed_at(devices: &[LanCanonicalHouseholdDevice]) -> Option<String> {
    let mut latest: Option<String> = None;
    for device in devices {
        latest = super::compare::latest_timestamp(
            latest.as_deref(),
            super::reachability::reachability_observed_at(device)
                .or_else(|| super::device::device_discovered_at(device))
                .as_deref(),
        );
        for evidence in &device.network_identity.evidence_records {
            latest = super::compare::latest_timestamp(
                latest.as_deref(),
                super::reachability::latest_evidence_observed_at(evidence).as_deref(),
            );
        }
    }
    latest
}

fn earliest_discovered_device_observed_at(
    discovered_devices: &[LanBrowserAddDeviceDiscoveryDevice],
) -> Option<String> {
    let mut earliest: Option<String> = None;
    for device in discovered_devices {
        let observed_at = super::device::discovered_device_observed_at(device);
        earliest =
            super::compare::earliest_timestamp(earliest.as_deref(), Some(observed_at.as_str()));
    }
    earliest
}

fn latest_discovered_device_observed_at(
    discovered_devices: &[LanBrowserAddDeviceDiscoveryDevice],
) -> Option<String> {
    let mut latest: Option<String> = None;
    for device in discovered_devices {
        let observed_at = super::device::discovered_device_observed_at(device);
        latest = super::compare::latest_timestamp(latest.as_deref(), Some(observed_at.as_str()));
    }
    latest
}
