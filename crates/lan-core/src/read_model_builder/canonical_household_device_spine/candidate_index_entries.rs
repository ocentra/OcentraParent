use std::collections::{HashMap, HashSet};

use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDevice;

use super::keys::merge_candidate_keys;

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
