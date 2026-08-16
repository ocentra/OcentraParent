use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanBrowserAddDeviceDiscoveryDevice, LanCanonicalHouseholdDevice, LanDiscoveryEvidenceRecord,
};

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

pub(super) fn evidence_observed_at(evidence: &LanDiscoveryEvidenceRecord) -> Option<String> {
    if !evidence.first_seen_at.is_empty() {
        Some(evidence.first_seen_at.clone())
    } else if !evidence.last_seen_at.is_empty() {
        Some(evidence.last_seen_at.clone())
    } else {
        None
    }
}

fn device_label_timestamp_fallback(device: &LanBrowserAddDeviceDiscoveryDevice) -> String {
    let mut fallback = String::from("undated-");
    fallback.push_str(&super::compact_event_identifier(
        &device.child_device.device_id,
    ));
    fallback
}
