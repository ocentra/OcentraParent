use std::sync::{mpsc, Arc, Mutex, Weak};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use ocentra_lan_core::network_inventory::passive_discovery::udp_multicast::LanPassiveDiscoveryUdpListener;
use ocentra_lan_core::network_inventory::passive_discovery::{
    LanPassiveDiscoveryListenerState, LanPassiveDiscoverySource, LanPassiveDiscoveryTriggerReason,
    LanPassiveDiscoveryUdpListenerIssue,
};
use ocentra_parent_agent_protocol::constants::lan_pairing as lan_pairing_constants;

use crate::{
    lan_pairing::LanPairingRuntime,
    lan_pairing_runtime_state::provider_heartbeat::LanAiProviderHeartbeatState,
};

use super::{
    capability_store::{
        LanPassiveDiscoveryCapabilityStore, LanPassiveDiscoverySourceAvailability,
        LanPassiveDiscoverySourceCapability,
    },
    pipeline_health::{
        LanPassiveDiscoveryPipelineHealth, LanPassiveDiscoveryPipelineHealthSnapshot,
    },
    PASSIVE_DISCOVERY_RETRY_BASE, PASSIVE_DISCOVERY_RETRY_MAX,
};
use super::{LanPassiveDiscoveryRefreshSignalSender, LanPassiveDiscoveryRuntimeObservedState};

const PASSIVE_DISCOVERY_LISTENER_THREAD_NAME: &str = "lan-passive-discovery-listener";
const PASSIVE_DISCOVERY_LISTENER_START_TIMEOUT: Duration = Duration::from_secs(2);
const PASSIVE_DISCOVERY_LISTENER_JOIN_TIMEOUT: Duration = Duration::from_secs(5);

enum PassiveDiscoveryListenerStartup {
    Ready,
    Unavailable,
}

#[path = "../passive_discovery_listener_bind.rs"]
mod bind;
#[path = "listener_runtime/engine.rs"]
mod engine;
#[path = "listener_runtime/receive.rs"]
mod receive;
#[path = "listener_runtime/cycle_cursor.rs"]
mod cycle_cursor;

struct PassiveDiscoveryListenerRuntime {
    listener_state: Weak<Mutex<LanPassiveDiscoveryListenerState>>,
    heartbeat: Weak<Mutex<Option<LanAiProviderHeartbeatState>>>,
    refresh_sender: LanPassiveDiscoveryRefreshSignalSender,
    observed_state: LanPassiveDiscoveryRuntimeObservedState,
    listener_slots: Vec<PassiveDiscoveryListenerSlot>,
    capability_store: LanPassiveDiscoveryCapabilityStore,
    pipeline_health: LanPassiveDiscoveryPipelineHealth,
    last_persisted_pipeline_health: Option<LanPassiveDiscoveryPipelineHealthSnapshot>,
    capability_persist_failures: u32,
    next_capability_persist_attempt: Instant,
    next_maintenance: Instant,
    signal_sequence: u64,
    next_listener_index: usize,
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
    refresh_sender: LanPassiveDiscoveryRefreshSignalSender,
    pipeline_health: LanPassiveDiscoveryPipelineHealth,
) -> std::io::Result<JoinHandle<()>> {
    let initial_refresh_signal = runtime.record_passive_rescan_trigger(
        LanPassiveDiscoveryTriggerReason::AppResumed,
        lan_pairing_constants::PASSIVE_DISCOVERY_RUNTIME_STARTED_SUMMARY,
    );
    let capability_store = LanPassiveDiscoveryCapabilityStore::for_runtime(&runtime);
    let listener_state = Arc::downgrade(&runtime.passive_discovery_listener_state);
    let heartbeat = Arc::downgrade(&runtime.lan_ai_provider_heartbeat);
    let (startup_sender, startup_receiver) = mpsc::sync_channel(1);
    let join = std::thread::Builder::new()
        .name(PASSIVE_DISCOVERY_LISTENER_THREAD_NAME.to_string())
        .spawn(move || {
            let mut listener_runtime = PassiveDiscoveryListenerRuntime::new(
                listener_state,
                heartbeat,
                refresh_sender,
                capability_store,
                pipeline_health,
            );
            if !listener_runtime.prepare_startup() {
                let _ = startup_sender.send(PassiveDiscoveryListenerStartup::Unavailable);
                return;
            }
            if startup_sender
                .send(PassiveDiscoveryListenerStartup::Ready)
                .is_err()
            {
                return;
            }
            if let Some(initial_refresh_signal) = initial_refresh_signal {
                listener_runtime.send_refresh_signals(vec![initial_refresh_signal]);
            }
            listener_runtime.run();
        })?;

    match startup_receiver.recv_timeout(PASSIVE_DISCOVERY_LISTENER_START_TIMEOUT) {
        Ok(PassiveDiscoveryListenerStartup::Ready) => Ok(join),
        Ok(PassiveDiscoveryListenerStartup::Unavailable)
        | Err(mpsc::RecvTimeoutError::Disconnected) => {
            stop_and_join_startup_worker(&runtime, join);
            Err(std::io::Error::from(std::io::ErrorKind::AddrNotAvailable))
        }
        Err(mpsc::RecvTimeoutError::Timeout) => {
            stop_and_join_startup_worker(&runtime, join);
            Err(std::io::Error::from(std::io::ErrorKind::TimedOut))
        }
    }
}

fn stop_and_join_startup_worker(runtime: &LanPairingRuntime, join: JoinHandle<()>) {
    if let Ok(mut listener_state) = runtime.passive_discovery_listener_state.lock() {
        listener_state.stop();
    }
    let deadline = Instant::now() + PASSIVE_DISCOVERY_LISTENER_JOIN_TIMEOUT;
    while !join.is_finished() && Instant::now() < deadline {
        thread::sleep(Duration::from_millis(5));
    }
    if !join.is_finished() {
        std::process::abort();
    }
    let _joined = join.join();
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
