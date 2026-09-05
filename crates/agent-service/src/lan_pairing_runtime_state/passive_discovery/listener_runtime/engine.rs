use std::{
    thread,
    time::{Duration, Instant},
};

use crate::lan_pairing::LanPairingRuntime;

use super::super::{
    capability_store::{LanPassiveDiscoveryCapabilityStore, LanPassiveDiscoveryRuntimeCapability},
    passive_discovery_udp_sources,
    runtime_slice::collect_runtime_slice,
    LanPassiveDiscoveryRefreshSignalSender, LanPassiveDiscoveryRuntimeObservedState,
    PASSIVE_DISCOVERY_INTERVAL,
};
use super::receive::is_running;
use super::{retry_delay, PassiveDiscoveryListenerRuntime, PassiveDiscoveryListenerSlot};

const PASSIVE_DISCOVERY_IDLE_WAIT: Duration = Duration::from_millis(25);

#[path = "engine/startup.rs"]
mod startup;

impl PassiveDiscoveryListenerRuntime {
    pub(super) fn new(
        runtime: LanPairingRuntime,
        refresh_sender: LanPassiveDiscoveryRefreshSignalSender,
        capability_store: LanPassiveDiscoveryCapabilityStore,
        pipeline_health: super::super::pipeline_health::LanPassiveDiscoveryPipelineHealth,
    ) -> Self {
        let now = Instant::now();
        Self {
            runtime,
            refresh_sender,
            observed_state: LanPassiveDiscoveryRuntimeObservedState::default(),
            listener_slots: passive_discovery_udp_sources()
                .iter()
                .copied()
                .map(|source| PassiveDiscoveryListenerSlot::pending(source, now))
                .collect(),
            capability_store,
            pipeline_health,
            last_persisted_pipeline_health: None,
            capability_persist_failures: 0,
            next_capability_persist_attempt: now,
            next_maintenance: now,
            signal_sequence: 0,
            next_listener_index: 0,
        }
    }

    pub(super) fn run(mut self) {
        self.persist_capability();
        loop {
            let listener_state =
                std::sync::Arc::clone(&self.runtime.passive_discovery_listener_state);
            if !is_running(&listener_state) {
                break;
            }
            self.run_maintenance_if_due();
            if !is_running(&listener_state) {
                break;
            }
            self.bind_due_listeners();
            self.receive_listener_cycle(&listener_state);
            self.persist_capability_if_health_changed();
            if self.active_listener_count() == 0 {
                thread::sleep(PASSIVE_DISCOVERY_IDLE_WAIT);
            }
        }
        self.pipeline_health.record_stopped();
        self.persist_stopped_capability();
    }

    fn run_maintenance_if_due(&mut self) {
        if Instant::now() < self.next_maintenance {
            return;
        }
        let outcome = collect_runtime_slice(&self.runtime, &mut self.observed_state);
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

    pub(super) fn persist_capability(&mut self) {
        let now = Instant::now();
        if now < self.next_capability_persist_attempt {
            return;
        }
        let pipeline_health = self.pipeline_health.snapshot();
        let sources = self
            .listener_slots
            .iter()
            .map(|slot| slot.capability(now))
            .collect::<Vec<_>>();
        if self
            .capability_store
            .save_sources(&sources, &pipeline_health)
        {
            self.last_persisted_pipeline_health = Some(pipeline_health);
            self.capability_persist_failures = 0;
            self.next_capability_persist_attempt = now;
            return;
        }
        self.capability_persist_failures = self.capability_persist_failures.saturating_add(1);
        self.next_capability_persist_attempt = now + retry_delay(self.capability_persist_failures);
    }

    fn persist_capability_if_health_changed(&mut self) {
        let current_health = self.pipeline_health.snapshot();
        if self.capability_persist_failures > 0
            || self.last_persisted_pipeline_health.as_ref() != Some(&current_health)
        {
            self.persist_capability();
        }
    }

    fn persist_stopped_capability(&self) {
        let now = Instant::now();
        let pipeline_health = self.pipeline_health.snapshot();
        let capability = LanPassiveDiscoveryRuntimeCapability::stopped(
            self.listener_slots
                .iter()
                .map(|slot| slot.capability(now))
                .collect(),
            pipeline_health,
        );
        let _persisted = self.capability_store.save(&capability);
    }
}
