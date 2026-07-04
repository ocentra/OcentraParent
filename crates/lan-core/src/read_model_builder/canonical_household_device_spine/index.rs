use std::collections::{HashMap, HashSet};

use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDevice, LanDiscoveryEvidenceKind, LanDiscoveryEvidenceRecord,
};

pub(super) fn candidate_indices(
    merge_index: &HashMap<String, Vec<usize>>,
    device: &LanCanonicalHouseholdDevice,
    source_ref: &LanPairingDeviceRef,
) -> Vec<usize> {
    let mut seen = HashSet::new();
    let mut indices = Vec::new();
    for key in merge_candidate_keys(device, source_ref) {
        let Some(indexed) = merge_index.get(&key) else {
            continue;
        };
        for index in indexed {
            if seen.insert(*index) {
                indices.push(*index);
            }
        }
    }
    indices
}

pub(super) fn index_device(
    merge_index: &mut HashMap<String, Vec<usize>>,
    device: &LanCanonicalHouseholdDevice,
    source_ref: &LanPairingDeviceRef,
    index: usize,
) {
    for key in merge_candidate_keys(device, source_ref) {
        let indices = merge_index.entry(key).or_default();
        if !indices.contains(&index) {
            indices.push(index);
        }
    }
}

fn merge_candidate_keys(
    device: &LanCanonicalHouseholdDevice,
    source_ref: &LanPairingDeviceRef,
) -> HashSet<String> {
    let mut keys = HashSet::new();
    push_normalized_key(&mut keys, "canonical", &device.canonical_device_id);
    push_optional_keys(&mut keys, device, source_ref);
    push_evidence_keys(&mut keys, &device.network_identity.evidence_records);
    keys
}

fn push_optional_keys(
    keys: &mut HashSet<String>,
    device: &LanCanonicalHouseholdDevice,
    source_ref: &LanPairingDeviceRef,
) {
    if let Some(mac_address) = device.network_identity.mac_address.as_deref() {
        push_normalized_key(keys, "mac", mac_address);
    }
    if let Some(mac_address) = source_ref.mac_address.as_deref() {
        push_normalized_key(keys, "mac", mac_address);
    }
    if let Some(ip_address) = source_ref.ip_address.as_deref() {
        push_normalized_key(keys, "ip", ip_address);
    }
    for ip_address in &device.network_identity.ip_addresses {
        push_normalized_key(keys, "ip", ip_address);
    }
    if let Some(hostname) = device.network_identity.hostname.as_deref() {
        push_normalized_key(keys, "hostname", hostname);
    }
}

fn push_evidence_keys(keys: &mut HashSet<String>, records: &[LanDiscoveryEvidenceRecord]) {
    for record in records {
        match record.evidence_kind {
            LanDiscoveryEvidenceKind::InstallId => {
                push_normalized_key(keys, "install", &record.normalized_value);
            }
            LanDiscoveryEvidenceKind::PairingId => {
                push_normalized_key(keys, "pairing", &record.normalized_value);
            }
            LanDiscoveryEvidenceKind::TrustedRegistry => {
                push_normalized_key(keys, "trusted", &record.normalized_value);
            }
            LanDiscoveryEvidenceKind::Vendor => {
                push_normalized_key(keys, "vendor", &record.normalized_value);
            }
            LanDiscoveryEvidenceKind::ServiceProbeHint => push_service_hint_key(keys, record),
            LanDiscoveryEvidenceKind::MacAddress => {
                push_normalized_key(keys, "mac", &record.normalized_value);
            }
            LanDiscoveryEvidenceKind::IpAddress => {
                push_normalized_key(keys, "ip", &record.normalized_value);
            }
            LanDiscoveryEvidenceKind::Hostname => {
                push_normalized_key(keys, "hostname", &record.normalized_value);
            }
            LanDiscoveryEvidenceKind::ChildAgentPresence
            | LanDiscoveryEvidenceKind::HistoricalIdentityHint
            | LanDiscoveryEvidenceKind::Interface
            | LanDiscoveryEvidenceKind::ParentDecision
            | LanDiscoveryEvidenceKind::Route
            | LanDiscoveryEvidenceKind::RouterClassification => {}
        }
    }
}

fn push_service_hint_key(keys: &mut HashSet<String>, record: &LanDiscoveryEvidenceRecord) {
    for prefix in [
        "mdns-instance-name:",
        "ssdp-udn:",
        "mdns-service-type:",
        "ssdp-device-type:",
    ] {
        if record
            .value
            .get(..prefix.len())
            .is_some_and(|value| value.eq_ignore_ascii_case(prefix))
        {
            push_normalized_key(keys, prefix, &record.normalized_value);
        }
    }
}

fn push_normalized_key(keys: &mut HashSet<String>, namespace: &str, value: &str) {
    let normalized = value.trim().to_ascii_lowercase();
    if !normalized.is_empty() {
        keys.insert(format!("{namespace}:{normalized}"));
    }
}
