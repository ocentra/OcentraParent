#[path = "merge_collections.rs"]
mod collections;
#[path = "merge_confidence.rs"]
mod confidence;
#[path = "merge_device.rs"]
mod device;
#[path = "merge_identity.rs"]
mod identity;
#[path = "merge_evidence.rs"]
mod merge_evidence;
#[path = "merge_ranking.rs"]
mod merge_ranking;

use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDevice;

pub(super) fn merge_device(
    existing: &mut LanCanonicalHouseholdDevice,
    incoming: LanCanonicalHouseholdDevice,
) {
    device::merge_device(existing, incoming)
}

pub(super) fn conflicting_source_identity(
    existing: &LanCanonicalHouseholdDevice,
    incoming: &LanCanonicalHouseholdDevice,
) -> bool {
    identity::conflicting_source_identity(existing, incoming)
}
