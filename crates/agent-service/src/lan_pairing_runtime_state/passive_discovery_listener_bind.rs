use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

use ocentra_lan_core::network_inventory::passive_discovery::{
    udp_multicast::bind_passive_udp_listener, LanPassiveDiscoveryListenerState,
};

use super::super::PASSIVE_DISCOVERY_READ_TIMEOUT;
use super::{PassiveDiscoveryListenerRuntime, PassiveDiscoveryListenerSlot};

impl PassiveDiscoveryListenerRuntime {
    pub(super) fn bind_due_listeners(&mut self) {
        let now = Instant::now();
        let mut changed = false;
        let Some(listener_state) = self.listener_state.upgrade() else {
            return;
        };
        for slot in &mut self.listener_slots {
            let Some(slot_changed) = bind_due_listener(slot, &listener_state, now) else {
                break;
            };
            changed |= slot_changed;
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

fn bind_due_listener(
    slot: &mut PassiveDiscoveryListenerSlot,
    listener_state: &Arc<Mutex<LanPassiveDiscoveryListenerState>>,
    now: Instant,
) -> Option<bool> {
    if !super::receive::is_running(listener_state) {
        return None;
    }
    if slot.listener.is_some() || slot.retry_at > now {
        return Some(false);
    }
    let bind_result = bind_passive_udp_listener(slot.source, PASSIVE_DISCOVERY_READ_TIMEOUT);
    if !super::receive::is_running(listener_state) {
        return None;
    }
    match bind_result {
        Ok(listener) => slot.record_listener(listener),
        Err(issue) => {
            // Capture the failure after bind returns. Measuring from the
            // pre-bind timestamp shortens the retry window when platform
            // interface lookup or bind blocks.
            slot.record_failure(issue, Instant::now());
        }
    }
    Some(true)
}
