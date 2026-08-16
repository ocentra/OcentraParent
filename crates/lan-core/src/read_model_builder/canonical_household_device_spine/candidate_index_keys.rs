#[path = "candidate_index_evidence_keys.rs"]
mod evidence_keys;

use std::collections::HashSet;

use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDevice;

pub(super) fn merge_candidate_keys(
    device: &LanCanonicalHouseholdDevice,
    source_ref: &LanPairingDeviceRef,
) -> HashSet<String> {
    let mut keys = HashSet::new();
    push_normalized_key(&mut keys, "canonical", &device.canonical_device_id);
    push_optional_keys(&mut keys, device, source_ref);
    evidence_keys::push_evidence_keys(&mut keys, &device.network_identity.evidence_records);
    keys
}

fn push_optional_keys(
    keys: &mut HashSet<String>,
    device: &LanCanonicalHouseholdDevice,
    source_ref: &LanPairingDeviceRef,
) {
    for mac_address in [
        device.network_identity.mac_address.as_deref(),
        source_ref.mac_address.as_deref(),
    ]
    .into_iter()
    .flatten()
    {
        push_normalized_key(keys, "mac", mac_address);
    }
    for ip_address in &device.network_identity.ip_addresses {
        push_normalized_key(keys, "ip", ip_address);
    }
    if let Some(ip_address) = source_ref.ip_address.as_deref() {
        push_normalized_key(keys, "ip", ip_address);
    }
    if let Some(hostname) = device.network_identity.hostname.as_deref() {
        push_normalized_key(keys, "hostname", hostname);
    }
}

pub(super) fn push_normalized_key(keys: &mut HashSet<String>, namespace: &str, value: &str) {
    let normalized = value.trim().to_ascii_lowercase();
    if !normalized.is_empty() {
        keys.insert(format!("{namespace}:{normalized}"));
    }
}
