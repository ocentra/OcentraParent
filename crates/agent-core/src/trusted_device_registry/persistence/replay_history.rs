use ocentra_parent_agent_protocol::constants;

use super::super::TrustedDeviceRegistry;

pub(super) fn merge_challenge_ids(
    current: &TrustedDeviceRegistry,
    persisted: &mut TrustedDeviceRegistry,
) {
    persisted
        .accepted_challenge_ids
        .extend(current.accepted_challenge_ids.iter().cloned());
    while persisted.accepted_challenge_ids.len()
        > constants::lan_pairing::LAN_PAIRING_MAX_ACCEPTED_INTENT_HISTORY
    {
        if let Some(oldest) = persisted.accepted_challenge_ids.iter().next().cloned() {
            persisted.accepted_challenge_ids.remove(&oldest);
        }
    }
}
