use std::{
    sync::{Mutex, Weak},
    thread,
    time::{Duration, Instant},
};

use ocentra_lan_core::network_inventory::passive_discovery::LanPassiveDiscoveryListenerState;
use tokio::sync::watch;

use crate::lan_pairing_runtime_state::provider_heartbeat::LanAiProviderHeartbeatState;

use super::super::{
    capability_store::{LanPassiveDiscoveryCapabilityStore, LanPassiveDiscoveryRuntimeCapability},
    passive_discovery_udp_sources,
    runtime_slice::collect_runtime_slice,
    LanPassiveDiscoveryRefreshSignal, LanPassiveDiscoveryRuntimeObservedState,
    PASSIVE_DISCOVERY_INTERVAL,
};
use super::receive::is_running;
use super::{PassiveDiscoveryListenerRuntime, PassiveDiscoveryListenerSlot};

const PASSIVE_DISCOVERY_IDLE_WAIT: Duration = Duration::from_millis(25);

#[path = "engine/bind.rs"]
mod bind;

impl PassiveDiscoveryListenerRuntime {
    pub(super) fn new(
        listener_state: Weak<Mutex<LanPassiveDiscoveryListenerState>>,
        heartbeat: Weak<Mutex<Option<LanAiProviderHeartbeatState>>>,
        refresh_sender: watch::Sender<Option<LanPassiveDiscoveryRefreshSignal>>,
        capability_store: LanPassiveDiscoveryCapabilityStore,
    ) -> Self {
        let now = Instant::now();
        Self {
            listener_state,
            heartbeat,
            refresh_sender,
            observed_state: LanPassiveDiscoveryRuntimeObservedState::default(),
            listener_slots: passive_discovery_udp_sources()
                .iter()
                .copied()
                .map(|source| PassiveDiscoveryListenerSlot::pending(source, now))
                .collect(),
            capability_store,
            next_maintenance: now,
            signal_sequence: 0,
        }
    }

    pub(super) fn run(mut self) {
        self.persist_capability();
        loop {
            let Some(listener_state) = self.listener_state.upgrade() else {
                break;
            };
            if !is_running(&listener_state) {
                break;
            }
            self.run_maintenance_if_due(&listener_state);
            if !is_running(&listener_state) {
                break;
            }
            self.bind_due_listeners();
            self.receive_listener_cycle(&listener_state);
            if self.active_listener_count() == 0 {
                thread::sleep(PASSIVE_DISCOVERY_IDLE_WAIT);
            }
        }
        self.persist_stopped_capability();
    }

    fn run_maintenance_if_due(
        &mut self,
        listener_state: &std::sync::Arc<Mutex<LanPassiveDiscoveryListenerState>>,
    ) {
        if Instant::now() < self.next_maintenance {
            return;
        }
        let outcome = collect_runtime_slice(
            listener_state,
            self.heartbeat_reachability(),
            &mut self.observed_state,
        );
        self.next_maintenance = Instant::now() + PASSIVE_DISCOVERY_INTERVAL;
        if !outcome.running {
            for slot in &mut self.listener_slots {
                slot.listener = None;
            }
        }
        if outcome.network_changed {
            let now = Instant::now();
            for slot in &mut self.listener_slots {
                slot.reset_for_network_change(now);
            }
        }
        self.send_refresh_signals(outcome.refresh_signals);
        self.persist_capability();
    }

    pub(super) fn persist_capability(&self) {
        let now = Instant::now();
        let capability = LanPassiveDiscoveryRuntimeCapability::from_sources(
            self.listener_slots
                .iter()
                .map(|slot| slot.capability(now))
                .collect(),
        );
        let _persisted = self.capability_store.save(&capability);
    }

    fn persist_stopped_capability(&self) {
        let now = Instant::now();
        let capability = LanPassiveDiscoveryRuntimeCapability::stopped(
            self.listener_slots
                .iter()
                .map(|slot| slot.capability(now))
                .collect(),
        );
        let _persisted = self.capability_store.save(&capability);
    }
}
