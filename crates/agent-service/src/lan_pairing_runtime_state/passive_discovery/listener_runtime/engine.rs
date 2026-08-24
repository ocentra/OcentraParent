use std::{
    sync::{Mutex, Weak},
    thread,
    time::{Duration, Instant},
};

use ocentra_lan_core::network_inventory::passive_discovery::udp_multicast::bind_passive_udp_listener;
use ocentra_lan_core::network_inventory::passive_discovery::LanPassiveDiscoveryListenerState;
use tokio::sync::watch;

use crate::lan_pairing_runtime_state::provider_heartbeat::LanAiProviderHeartbeatState;

use super::super::{
    passive_discovery_udp_sources, runtime_slice::collect_runtime_slice,
    LanPassiveDiscoveryRefreshSignal, LanPassiveDiscoveryRuntimeObservedState,
    PASSIVE_DISCOVERY_INTERVAL, PASSIVE_DISCOVERY_READ_TIMEOUT, PASSIVE_DISCOVERY_REBIND_INTERVAL,
};
use super::receive::is_running;
use super::PassiveDiscoveryListenerRuntime;

const PASSIVE_DISCOVERY_IDLE_WAIT: Duration = Duration::from_millis(25);

impl PassiveDiscoveryListenerRuntime {
    pub(super) fn new(
        listener_state: Weak<Mutex<LanPassiveDiscoveryListenerState>>,
        heartbeat: Weak<Mutex<Option<LanAiProviderHeartbeatState>>>,
        refresh_sender: watch::Sender<Option<LanPassiveDiscoveryRefreshSignal>>,
    ) -> Self {
        let now = Instant::now();
        Self {
            listener_state,
            heartbeat,
            refresh_sender,
            observed_state: LanPassiveDiscoveryRuntimeObservedState::default(),
            listeners: Vec::new(),
            next_maintenance: now,
            next_rebind: now,
            signal_sequence: 0,
        }
    }

    pub(super) fn run(mut self) {
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
            self.bind_missing_listeners_if_due();
            self.receive_listener_cycle(&listener_state);
            if self.listeners.is_empty() {
                thread::sleep(PASSIVE_DISCOVERY_IDLE_WAIT);
            }
        }
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
        if !outcome.running || outcome.network_changed {
            self.listeners.clear();
        }
        if outcome.network_changed {
            self.next_rebind = Instant::now();
        }
        self.send_refresh_signals(outcome.refresh_signals);
    }

    fn bind_missing_listeners_if_due(&mut self) {
        if Instant::now() < self.next_rebind {
            return;
        }
        for source in passive_discovery_udp_sources() {
            if self
                .listeners
                .iter()
                .any(|listener| listener.source() == *source)
            {
                continue;
            }
            if let Ok(listener) = bind_passive_udp_listener(*source, PASSIVE_DISCOVERY_READ_TIMEOUT)
            {
                self.listeners.push(listener);
            }
        }
        self.next_rebind = Instant::now() + PASSIVE_DISCOVERY_REBIND_INTERVAL;
    }
}
