use std::sync::{Arc, Mutex};
use std::time::Instant;

use ocentra_lan_core::network_inventory::passive_discovery::udp_multicast::{
    ingest_passive_datagram_with_observed_at, LanPassiveDiscoveryUdpDatagram,
};
use ocentra_lan_core::network_inventory::passive_discovery::{
    LanPassiveDiscoveryListenerState, LanPassiveDiscoveryPacketIngestOutcome,
    LanPassiveDiscoveryTriggerReason,
};
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;

use crate::time::timestamp_now;

use super::super::{
    LanPassiveDiscoveryRefreshSignal, PASSIVE_DISCOVERY_MAX_DATAGRAMS_PER_SOURCE,
    PASSIVE_DISCOVERY_REBIND_INTERVAL,
};
use super::PassiveDiscoveryListenerRuntime;

impl PassiveDiscoveryListenerRuntime {
    pub(super) fn receive_listener_cycle(
        &mut self,
        listener_state: &Arc<Mutex<LanPassiveDiscoveryListenerState>>,
    ) {
        let mut listener_index = 0;
        while listener_index < self.listeners.len() {
            listener_index = self.receive_listener(listener_state, listener_index);
            if !is_running(listener_state) {
                break;
            }
        }
    }

    fn receive_listener(
        &mut self,
        listener_state: &Arc<Mutex<LanPassiveDiscoveryListenerState>>,
        listener_index: usize,
    ) -> usize {
        let datagrams = self.listeners[listener_index]
            .receive_bounded(PASSIVE_DISCOVERY_MAX_DATAGRAMS_PER_SOURCE);
        match datagrams {
            Ok(datagrams) => {
                self.ingest_datagrams(listener_state, datagrams);
                listener_index + 1
            }
            Err(_error) => {
                self.listeners.remove(listener_index);
                self.next_rebind = Instant::now() + PASSIVE_DISCOVERY_REBIND_INTERVAL;
                listener_index
            }
        }
    }

    fn ingest_datagrams(
        &mut self,
        listener_state: &Arc<Mutex<LanPassiveDiscoveryListenerState>>,
        datagrams: Vec<LanPassiveDiscoveryUdpDatagram>,
    ) {
        let refresh_signals = collect_refresh_signals(listener_state, datagrams);
        self.send_refresh_signals(refresh_signals);
    }

    pub(super) fn heartbeat_reachability(&self) -> Option<LanPairingDeviceReachability> {
        self.heartbeat.upgrade().and_then(|heartbeat| {
            heartbeat
                .lock()
                .ok()
                .and_then(|state| state.as_ref().map(|state| state.reachability.clone()))
        })
    }

    pub(super) fn send_refresh_signals(
        &mut self,
        refresh_signals: Vec<LanPassiveDiscoveryRefreshSignal>,
    ) {
        for signal in refresh_signals {
            self.signal_sequence = self.signal_sequence.saturating_add(1);
            let mut signal = signal;
            signal.sequence = self.signal_sequence;
            let _previous = self.refresh_sender.send_replace(Some(signal));
        }
    }
}

pub(super) fn is_running(listener_state: &Arc<Mutex<LanPassiveDiscoveryListenerState>>) -> bool {
    listener_state
        .lock()
        .map(|state| state.is_running())
        .unwrap_or(false)
}

fn normalized_refresh_signal(
    state: &mut LanPassiveDiscoveryListenerState,
    datagram: LanPassiveDiscoveryUdpDatagram,
) -> Option<LanPassiveDiscoveryRefreshSignal> {
    let observed_at: String = timestamp_now();
    let outcome = ingest_passive_datagram_with_observed_at(
        state,
        &datagram.source(),
        datagram.payload(),
        &observed_at,
    );
    if outcome != LanPassiveDiscoveryPacketIngestOutcome::Recorded {
        return None;
    }
    Some(LanPassiveDiscoveryRefreshSignal {
        sequence: 0,
        source: Some(datagram.source()),
        trigger_reason: LanPassiveDiscoveryTriggerReason::PassivePacketObserved,
        observed_at,
    })
}

fn collect_refresh_signals(
    listener_state: &Arc<Mutex<LanPassiveDiscoveryListenerState>>,
    datagrams: Vec<LanPassiveDiscoveryUdpDatagram>,
) -> Vec<LanPassiveDiscoveryRefreshSignal> {
    let Ok(mut state) = listener_state.lock() else {
        return Vec::new();
    };
    if !state.is_running() {
        return Vec::new();
    }
    datagrams
        .into_iter()
        .filter_map(|datagram| normalized_refresh_signal(&mut state, datagram))
        .collect()
}
