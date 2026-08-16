use std::collections::HashMap;

use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDevice;

#[path = "candidate_index_entries.rs"]
mod entries;
#[path = "candidate_index_keys.rs"]
mod keys;

pub(super) fn candidate_indices(
    merge_index: &HashMap<String, Vec<usize>>,
    device: &LanCanonicalHouseholdDevice,
    source_ref: &LanPairingDeviceRef,
) -> Vec<usize> {
    entries::candidate_indices(merge_index, device, source_ref)
}

pub(super) fn index_device(
    merge_index: &mut HashMap<String, Vec<usize>>,
    device: &LanCanonicalHouseholdDevice,
    source_ref: &LanPairingDeviceRef,
    index: usize,
) {
    entries::index_device(merge_index, device, source_ref, index);
}
