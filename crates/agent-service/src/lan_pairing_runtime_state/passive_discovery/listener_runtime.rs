use std::sync::{Arc, Mutex, Weak};
use std::time::{Duration, Instant};

use ocentra_lan_core::network_inventory::passive_discovery::udp_multicast::LanPassiveDiscoveryUdpListener;
use ocentra_lan_core::network_inventory::passive_discovery::{
    LanPassiveDiscoveryListenerState, LanPassiveDiscoverySource, LanPassiveDiscoveryTriggerReason,
    LanPassiveDiscoveryUdpListenerIssue,
};
use ocentra_parent_agent_protocol::constants::lan_pairing as lan_pairing_constants;
use tokio::sync::watch;

use crate::{
    lan_pairing::LanPairingRuntime,
    lan_pairing_runtime_state::provider_heartbeat::LanAiProviderHeartbeatState,
};

use super::{
    capability_store::{
        LanPassiveDiscoveryCapabilityStore, LanPassiveDiscoverySourceAvailability,
        LanPassiveDiscoverySourceCapability,
    },
    PASSIVE_DISCOVERY_RETRY_BASE, PASSIVE_DISCOVERY_RETRY_MAX,
};
use super::{LanPassiveDiscoveryRefreshSignal, LanPassiveDiscoveryRuntimeObservedState};

#[path = "listener_runtime/engine.rs"]
mod engine;
#[path = "listener_runtime/receive.rs"]
mod receive;

struct PassiveDiscoveryListenerRuntime {
    listener_state: Weak<Mutex<LanPassiveDiscoveryListenerState>>,
    heartbeat: Weak<Mutex<Option<LanAiProviderHeartbeatState>>>,
    refresh_sender: watch::Sender<Option<LanPassiveDiscoveryRefreshSignal>>,
    observed_state: LanPassiveDiscoveryRuntimeObservedState,
    listener_slots: Vec<PassiveDiscoveryListenerSlot>,
    capability_store: LanPassiveDiscoveryCapabilityStore,
    next_maintenance: Instant,
    signal_sequence: u64,
}

struct PassiveDiscoveryListenerSlot {
    source: LanPassiveDiscoverySource,
    listener: Option<LanPassiveDiscoveryUdpListener>,
    consecutive_failures: u32,
    retry_at: Instant,
    issue: Option<LanPassiveDiscoveryUdpListenerIssue>,
}

pub(super) fn spawn(
    runtime: LanPairingRuntime,
    refresh_sender: watch::Sender<Option<LanPassiveDiscoveryRefreshSignal>>,
) {
    let initial_refresh_signal = runtime.record_passive_rescan_trigger(
        LanPassiveDiscoveryTriggerReason::AppResumed,
        lan_pairing_constants::PASSIVE_DISCOVERY_RUNTIME_STARTED_SUMMARY,
    );
    let capability_store = LanPassiveDiscoveryCapabilityStore::for_runtime(&runtime);
    let listener_state = Arc::downgrade(&runtime.passive_discovery_listener_state);
    let heartbeat = Arc::downgrade(&runtime.lan_ai_provider_heartbeat);
    tokio::task::spawn_blocking(move || {
        let mut listener_runtime = PassiveDiscoveryListenerRuntime::new(
            listener_state,
            heartbeat,
            refresh_sender,
            capability_store,
        );
        if let Some(initial_refresh_signal) = initial_refresh_signal {
            listener_runtime.send_refresh_signals(vec![initial_refresh_signal]);
        }
        listener_runtime.run();
    });
}

impl PassiveDiscoveryListenerSlot {
    fn pending(source: LanPassiveDiscoverySource, now: Instant) -> Self {
        Self {
            source,
            listener: None,
            consecutive_failures: 0,
            retry_at: now,
            issue: None,
        }
    }

    fn record_listener(&mut self, listener: LanPassiveDiscoveryUdpListener) {
        self.listener = Some(listener);
        self.consecutive_failures = 0;
        self.issue = None;
    }

    fn record_failure(&mut self, issue: LanPassiveDiscoveryUdpListenerIssue, now: Instant) {
        self.listener = None;
        self.consecutive_failures = self.consecutive_failures.saturating_add(1);
        self.retry_at = now + retry_delay(self.consecutive_failures);
        self.issue = Some(issue);
    }

    fn reset_for_network_change(&mut self, now: Instant) {
        self.listener = None;
        self.consecutive_failures = 0;
        self.retry_at = now;
        self.issue = None;
    }

    fn capability(&self, now: Instant) -> LanPassiveDiscoverySourceCapability {
        let availability = if self.listener.is_some() {
            LanPassiveDiscoverySourceAvailability::Listening
        } else if self.consecutive_failures == 0 {
            LanPassiveDiscoverySourceAvailability::PendingBind
        } else {
            LanPassiveDiscoverySourceAvailability::RetryScheduled
        };
        let retry_delay_millis = (availability
            == LanPassiveDiscoverySourceAvailability::RetryScheduled)
            .then(|| duration_millis(self.retry_at.saturating_duration_since(now)));
        LanPassiveDiscoverySourceCapability {
            source: self.source,
            availability,
            consecutive_failures: self.consecutive_failures,
            retry_delay_millis,
            issue: self.issue.clone(),
        }
    }
}

fn retry_delay(consecutive_failures: u32) -> Duration {
    let exponent = consecutive_failures.saturating_sub(1).min(6);
    let multiplier = 1_u32 << exponent;
    PASSIVE_DISCOVERY_RETRY_BASE
        .saturating_mul(multiplier)
        .min(PASSIVE_DISCOVERY_RETRY_MAX)
}

fn duration_millis(duration: Duration) -> u64 {
    u64::try_from(duration.as_millis()).unwrap_or(u64::MAX)
}
