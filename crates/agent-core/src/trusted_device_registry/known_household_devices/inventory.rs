use ocentra_parent_agent_protocol::lan_pairing::LanPairingTrustState;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDevice;

pub(super) fn merge_child_agent_inventory(
    existing: &mut LanCanonicalHouseholdDevice,
    incoming: Option<
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanChildAgentInventoryPacket,
    >,
    merged_trust_state: LanPairingTrustState,
) {
    if incoming.is_some()
        && (merged_trust_state == LanPairingTrustState::Paired
            || existing.child_agent_inventory.is_none())
    {
        existing.child_agent_inventory = incoming;
    }
}
