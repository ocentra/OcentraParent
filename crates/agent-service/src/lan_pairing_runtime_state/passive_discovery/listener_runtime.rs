use std::sync::{Arc, Mutex, Weak};
use std::time::Instant;

use ocentra_lan_core::network_inventory::passive_discovery::udp_multicast::LanPassiveDiscoveryUdpListener;
use ocentra_lan_core::network_inventory::passive_discovery::{
    LanPassiveDiscoveryListenerState, LanPassiveDiscoveryTriggerReason,
};
use ocentra_parent_agent_protocol::constants::lan_pairing as lan_pairing_constants;
use tokio::sync::watch;

use crate::{
    lan_pairing::LanPairingRuntime,
    lan_pairing_runtime_state::provider_heartbeat::LanAiProviderHeartbeatState,
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
    listeners: Vec<LanPassiveDiscoveryUdpListener>,
    next_maintenance: Instant,
    next_rebind: Instant,
    signal_sequence: u64,
}

pub(super) fn spawn(
    runtime: LanPairingRuntime,
    refresh_sender: watch::Sender<Option<LanPassiveDiscoveryRefreshSignal>>,
) {
    runtime.record_passive_rescan_trigger(
        LanPassiveDiscoveryTriggerReason::AppResumed,
        lan_pairing_constants::PASSIVE_DISCOVERY_RUNTIME_STARTED_SUMMARY,
    );
    let listener_state = Arc::downgrade(&runtime.passive_discovery_listener_state);
    let heartbeat = Arc::downgrade(&runtime.lan_ai_provider_heartbeat);
    tokio::task::spawn_blocking(move || {
        PassiveDiscoveryListenerRuntime::new(listener_state, heartbeat, refresh_sender).run();
    });
}
