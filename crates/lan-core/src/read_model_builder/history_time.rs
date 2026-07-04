use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanBrowserAddDeviceDiscoveryDevice, LanCanonicalHouseholdDevice, LanDiscoveryEvidenceRecord,
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
    earliest_timestamp(canonical.as_deref(), discovered.as_deref())
}

pub(super) fn latest_canonical_or_discovered_observed_at(
    devices: &[LanCanonicalHouseholdDevice],
    discovered_devices: &[LanBrowserAddDeviceDiscoveryDevice],
) -> Option<String> {
    let canonical = latest_device_observed_at(devices);
    let discovered = latest_discovered_device_observed_at(discovered_devices);
    latest_timestamp(canonical.as_deref(), discovered.as_deref())
}

pub(super) fn device_discovered_at(device: &LanCanonicalHouseholdDevice) -> Option<String> {
    let mut earliest: Option<String> = None;
    for evidence in &device.network_identity.evidence_records {
        if let Some(observed_at) = evidence_observed_at(evidence) {
            if earliest
                .as_ref()
                .is_none_or(|current| observed_at.as_str() < current.as_str())
            {
                earliest = Some(observed_at);
            }
        }
    }
    earliest
}

pub(super) fn discovered_device_observed_at(device: &LanBrowserAddDeviceDiscoveryDevice) -> String {
    if !device.discovered_at.is_empty() {
        device.discovered_at.clone()
    } else {
        device_label_timestamp_fallback(device)
    }
}

pub(super) fn discovered_device_label(device: &LanBrowserAddDeviceDiscoveryDevice) -> String {
    device
        .child_device
        .hostname
        .as_ref()
        .filter(|hostname| !hostname.is_empty())
        .cloned()
        .unwrap_or_else(|| device.child_device.label.clone())
}

pub(super) fn reachability_observed_at(device: &LanCanonicalHouseholdDevice) -> Option<String> {
    match device.network_identity.reachability {
        LanPairingDeviceReachability::Online => {
            latest_evidence_last_seen(&device.network_identity.evidence_records)
                .or_else(|| device_discovered_at(device))
        }
        LanPairingDeviceReachability::Offline => device
            .network_identity
            .offline_at
            .clone()
            .or_else(|| latest_evidence_last_seen(&device.network_identity.evidence_records))
            .or_else(|| device_discovered_at(device)),
        LanPairingDeviceReachability::Stale => device
            .network_identity
            .stale_at
            .clone()
            .or_else(|| latest_evidence_last_seen(&device.network_identity.evidence_records))
            .or_else(|| device_discovered_at(device)),
    }
}

pub(super) fn evidence_observed_at(evidence: &LanDiscoveryEvidenceRecord) -> Option<String> {
    if !evidence.first_seen_at.is_empty() {
        Some(evidence.first_seen_at.clone())
    } else if !evidence.last_seen_at.is_empty() {
        Some(evidence.last_seen_at.clone())
    } else {
        None
    }
}

fn earliest_device_observed_at(devices: &[LanCanonicalHouseholdDevice]) -> Option<String> {
    let mut earliest: Option<String> = None;
    for device in devices {
        earliest = earliest_timestamp(earliest.as_deref(), device_discovered_at(device).as_deref());
        for evidence in &device.network_identity.evidence_records {
            earliest = earliest_timestamp(
                earliest.as_deref(),
                evidence_observed_at(evidence).as_deref(),
            );
        }
    }
    earliest
}

fn latest_device_observed_at(devices: &[LanCanonicalHouseholdDevice]) -> Option<String> {
    let mut latest: Option<String> = None;
    for device in devices {
        latest = latest_timestamp(
            latest.as_deref(),
            reachability_observed_at(device)
                .or_else(|| device_discovered_at(device))
                .as_deref(),
        );
        for evidence in &device.network_identity.evidence_records {
            latest = latest_timestamp(
                latest.as_deref(),
                latest_evidence_observed_at(evidence).as_deref(),
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
        let observed_at = discovered_device_observed_at(device);
        earliest = earliest_timestamp(earliest.as_deref(), Some(observed_at.as_str()));
    }
    earliest
}

fn latest_discovered_device_observed_at(
    discovered_devices: &[LanBrowserAddDeviceDiscoveryDevice],
) -> Option<String> {
    let mut latest: Option<String> = None;
    for device in discovered_devices {
        let observed_at = discovered_device_observed_at(device);
        latest = latest_timestamp(latest.as_deref(), Some(observed_at.as_str()));
    }
    latest
}

fn earliest_timestamp(first: Option<&str>, second: Option<&str>) -> Option<String> {
    match (first, second) {
        (Some(first), Some(second)) => {
            Some(if first <= second { first } else { second }.to_string())
        }
        (Some(first), None) => Some(first.to_string()),
        (None, Some(second)) => Some(second.to_string()),
        (None, None) => None,
    }
}

fn latest_timestamp(first: Option<&str>, second: Option<&str>) -> Option<String> {
    match (first, second) {
        (Some(first), Some(second)) => {
            Some(if first >= second { first } else { second }.to_string())
        }
        (Some(first), None) => Some(first.to_string()),
        (None, Some(second)) => Some(second.to_string()),
        (None, None) => None,
    }
}

fn device_label_timestamp_fallback(device: &LanBrowserAddDeviceDiscoveryDevice) -> String {
    let mut fallback = String::from("undated-");
    fallback.push_str(&compact_event_identifier(&device.child_device.device_id));
    fallback
}

fn latest_evidence_last_seen(evidence_records: &[LanDiscoveryEvidenceRecord]) -> Option<String> {
    let mut latest: Option<String> = None;
    for evidence in evidence_records {
        latest = latest_timestamp(
            latest.as_deref(),
            latest_evidence_observed_at(evidence).as_deref(),
        );
    }
    latest
}

fn latest_evidence_observed_at(evidence: &LanDiscoveryEvidenceRecord) -> Option<String> {
    if !evidence.last_seen_at.is_empty() {
        Some(evidence.last_seen_at.clone())
    } else {
        evidence_observed_at(evidence)
    }
}
