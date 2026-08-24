use std::time::Instant;

use ocentra_lan_core::network_inventory::passive_discovery::udp_multicast::bind_passive_udp_listener;

use super::super::super::PASSIVE_DISCOVERY_READ_TIMEOUT;
use super::super::PassiveDiscoveryListenerRuntime;

impl PassiveDiscoveryListenerRuntime {
    pub(super) fn bind_due_listeners(&mut self) {
        let now = Instant::now();
        let mut changed = false;
        for slot in &mut self.listener_slots {
            if slot.listener.is_some() || slot.retry_at > now {
                continue;
            }
            match bind_passive_udp_listener(slot.source, PASSIVE_DISCOVERY_READ_TIMEOUT) {
                Ok(listener) => slot.record_listener(listener),
                Err(issue) => slot.record_failure(issue, now),
            }
            changed = true;
        }
        if changed {
            self.persist_capability();
        }
    }

    pub(super) fn active_listener_count(&self) -> usize {
        self.listener_slots
            .iter()
            .filter(|slot| slot.listener.is_some())
            .count()
    }
}
