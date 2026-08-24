#[path = "passive_discovery/capability_store.rs"]
pub(crate) mod capability_store;
#[path = "passive_discovery/change_triggers.rs"]
mod change_triggers;
#[path = "passive_discovery/listener_runtime.rs"]
mod listener_runtime;
#[path = "passive_discovery/pipeline_health.rs"]
mod pipeline_health;
#[path = "passive_discovery/reconciliation.rs"]
mod reconciliation;
#[path = "passive_discovery/runtime_slice.rs"]
mod runtime_slice;

use std::{
    sync::{Arc, Mutex, Weak},
    time::Duration,
};

use ocentra_lan_core::network_inventory::passive_discovery::{
    LanPassiveDiscoveryListenerState, LanPassiveDiscoverySource, LanPassiveDiscoveryTriggerReason,
};
use ocentra_lan_core::network_inventory::LanPassiveRuntimeLocalNetworkIdentity;
use tokio::sync::watch;

use crate::lan_pairing::LanPairingRuntime;

const PASSIVE_DISCOVERY_INTERVAL: Duration = Duration::from_secs(180);
const PASSIVE_DISCOVERY_READ_TIMEOUT: Duration = Duration::from_millis(50);
const PASSIVE_DISCOVERY_MAX_DATAGRAMS_PER_SOURCE: usize = 8;
const PASSIVE_DISCOVERY_RETRY_BASE: Duration = Duration::from_secs(1);
const PASSIVE_DISCOVERY_RETRY_MAX: Duration = Duration::from_secs(60);
const PASSIVE_DISCOVERY_RECONCILIATION_MIN_INTERVAL: Duration = Duration::from_secs(120);

const PASSIVE_DISCOVERY_UDP_SOURCES: [LanPassiveDiscoverySource; 6] = [
    LanPassiveDiscoverySource::Dhcp,
    LanPassiveDiscoverySource::Mdns,
    LanPassiveDiscoverySource::Ssdp,
    LanPassiveDiscoverySource::WsDiscovery,
    LanPassiveDiscoverySource::Llmnr,
    LanPassiveDiscoverySource::Netbios,
];

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct LanPassiveDiscoveryRuntimeObservedState {
    heartbeat_loss_recorded: bool,
    last_local_network_identity: Option<LanPassiveRuntimeLocalNetworkIdentity>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LanPassiveDiscoveryLocalNetworkChangeTrigger {
    pub(crate) reason: LanPassiveDiscoveryTriggerReason,
    pub(crate) summary: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LanPassiveDiscoveryRefreshSignal {
    sequence: u64,
    source: Option<LanPassiveDiscoverySource>,
    trigger_reason: LanPassiveDiscoveryTriggerReason,
    observed_at: String,
}

struct LanPassiveDiscoveryRefreshSignalSender {
    deliberate: watch::Sender<Option<LanPassiveDiscoveryRefreshSignal>>,
    passive: watch::Sender<Option<LanPassiveDiscoveryRefreshSignal>>,
}

struct LanPassiveDiscoveryRefreshSignalReceiver {
    deliberate: watch::Receiver<Option<LanPassiveDiscoveryRefreshSignal>>,
    passive: watch::Receiver<Option<LanPassiveDiscoveryRefreshSignal>>,
}

pub(super) struct LanPassiveDiscoveryRuntimeSliceOutcome {
    running: bool,
    network_changed: bool,
    refresh_signals: Vec<LanPassiveDiscoveryRefreshSignal>,
}

#[derive(Clone)]
pub(crate) struct LanPassiveDiscoveryServiceRuntime {
    _owner: Arc<LanPassiveDiscoveryServiceOwner>,
}

struct LanPassiveDiscoveryServiceOwner {
    listener_state: Weak<Mutex<LanPassiveDiscoveryListenerState>>,
}

impl Drop for LanPassiveDiscoveryServiceOwner {
    fn drop(&mut self) {
        if let Some(listener_state) = self.listener_state.upgrade() {
            if let Ok(mut listener_state) = listener_state.lock() {
                listener_state.stop();
            }
        }
    }
}

pub(crate) fn spawn_lan_passive_discovery_runtime(runtime: LanPairingRuntime) {
    let pipeline_health = pipeline_health::LanPassiveDiscoveryPipelineHealth::starting();
    capability_store::record_starting(&runtime, &pipeline_health.snapshot());
    let (sender, receiver) = refresh_signal_channel();
    listener_runtime::spawn(runtime.clone(), sender, pipeline_health.clone());
    reconciliation::spawn(runtime, receiver, pipeline_health);
}

pub(crate) fn start_lan_passive_discovery_service_runtime(
    runtime: LanPairingRuntime,
) -> LanPassiveDiscoveryServiceRuntime {
    let owner = Arc::new(LanPassiveDiscoveryServiceOwner {
        listener_state: Arc::downgrade(&runtime.passive_discovery_listener_state),
    });
    let pipeline_health = pipeline_health::LanPassiveDiscoveryPipelineHealth::starting();
    capability_store::record_starting(&runtime, &pipeline_health.snapshot());
    let (sender, receiver) = refresh_signal_channel();
    listener_runtime::spawn(runtime.clone(), sender, pipeline_health.clone());
    reconciliation::spawn(runtime, receiver, pipeline_health);
    LanPassiveDiscoveryServiceRuntime { _owner: owner }
}

fn refresh_signal_channel() -> (
    LanPassiveDiscoveryRefreshSignalSender,
    LanPassiveDiscoveryRefreshSignalReceiver,
) {
    let (deliberate_sender, deliberate_receiver) = watch::channel(None);
    let (passive_sender, passive_receiver) = watch::channel(None);
    (
        LanPassiveDiscoveryRefreshSignalSender {
            deliberate: deliberate_sender,
            passive: passive_sender,
        },
        LanPassiveDiscoveryRefreshSignalReceiver {
            deliberate: deliberate_receiver,
            passive: passive_receiver,
        },
    )
}

pub(crate) fn local_network_change_triggers(
    previous_identity: Option<&LanPassiveRuntimeLocalNetworkIdentity>,
    current_identity: &LanPassiveRuntimeLocalNetworkIdentity,
) -> Vec<LanPassiveDiscoveryLocalNetworkChangeTrigger> {
    change_triggers::local_network_change_triggers(previous_identity, current_identity)
}

pub(crate) fn passive_discovery_udp_sources() -> &'static [LanPassiveDiscoverySource] {
    &PASSIVE_DISCOVERY_UDP_SOURCES
}

impl LanPassiveDiscoveryRefreshSignal {
    fn is_coherent_after(&self, previous_sequence: u64) -> bool {
        if self.sequence <= previous_sequence || self.observed_at.trim().is_empty() {
            return false;
        }
        match self.trigger_reason {
            LanPassiveDiscoveryTriggerReason::PassivePacketObserved => self.source.is_some(),
            _ => self.source.is_none(),
        }
    }
}

impl LanPassiveDiscoveryRefreshSignalSender {
    fn send_replace(
        &self,
        signal: Option<LanPassiveDiscoveryRefreshSignal>,
    ) -> Option<LanPassiveDiscoveryRefreshSignal> {
        match signal.as_ref().map(|signal| &signal.trigger_reason) {
            Some(LanPassiveDiscoveryTriggerReason::PassivePacketObserved) => {
                self.passive.send_replace(signal)
            }
            _ => self.deliberate.send_replace(signal),
        }
    }
}
