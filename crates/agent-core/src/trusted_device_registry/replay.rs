use std::collections::VecDeque;

use ocentra_parent_agent_protocol::constants;

use super::TrustedDeviceRegistry;

pub(super) fn remember_bounded_replay_id(history: &mut VecDeque<String>, id: String) {
    if history.iter().any(|candidate| candidate == &id) {
        return;
    }
    if history.len() >= constants::lan_pairing::LAN_PAIRING_MAX_ACCEPTED_INTENT_HISTORY {
        let _ = history.pop_front();
    }
    history.push_back(id);
}

impl TrustedDeviceRegistry {
    pub(super) fn remember_accepted_intent_id(&mut self, intent_id: String) {
        remember_bounded_replay_id(&mut self.accepted_intent_ids, intent_id);
    }
}
