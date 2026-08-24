use std::time::Instant;

use ocentra_lan_core::network_inventory::passive_discovery::udp_multicast::bind_passive_udp_listener;

use super::super::super::PASSIVE_DISCOVERY_READ_TIMEOUT;
use super::super::PassiveDiscoveryListenerRuntime;

impl PassiveDiscoveryListenerRuntime {
    pub(super) fn bind_due_listeners(&mut self) {
        let now = Instant::now();
        let mut changed = false;
        let Some(listener_state) = self.listener_state.upgrade() else {
            return;
        };
        for slot in &mut self.listener_slots {
            if !super::super::receive::is_running(&listener_state) {
                break;
            }
            if slot.listener.is_some() || slot.retry_at > now {
                continue;
            }
            match bind_passive_udp_listener(slot.source, PASSIVE_DISCOVERY_READ_TIMEOUT) {
                Ok(listener) if super::super::receive::is_running(&listener_state) => {
                    slot.record_listener(listener)
                }
                Ok(_listener) => break,
                Err(issue) if super::super::receive::is_running(&listener_state) => {
                    // Capture the failure after bind returns.  Measuring from the
                    // pre-bind timestamp shortens the retry window when platform
                    // interface lookup or bind blocks.
                    slot.record_failure(issue, Instant::now())
                }
                Err(_issue) => break,
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
