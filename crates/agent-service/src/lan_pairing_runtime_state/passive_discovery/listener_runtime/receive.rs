use std::sync::{Arc, Mutex};
use std::time::Instant;

use super::super::{
    LanPassiveDiscoveryRefreshSignal, PASSIVE_DISCOVERY_MAX_CYCLE_DURATION,
    PASSIVE_DISCOVERY_MAX_DATAGRAMS_PER_CYCLE, PASSIVE_DISCOVERY_MAX_DATAGRAMS_PER_SOURCE,
};
use super::cycle_cursor::PassiveDiscoveryCycleCursor;
use super::PassiveDiscoveryListenerRuntime;
use ocentra_lan_core::network_inventory::passive_discovery::udp_multicast::LanPassiveDiscoveryUdpDatagram;
use ocentra_lan_core::network_inventory::passive_discovery::{
    LanPassiveDiscoveryListenerState, LanPassiveDiscoveryPacketIngestOutcome,
    LanPassiveDiscoveryTriggerReason,
};

impl PassiveDiscoveryListenerRuntime {
    pub(super) fn receive_listener_cycle(
        &mut self,
        listener_state: &Arc<Mutex<LanPassiveDiscoveryListenerState>>,
    ) {
        let listener_count = self.listener_slots.len();
        if listener_count == 0 {
            return;
        }
        let cycle_started = Instant::now();
        let cycle_deadline = cycle_started + PASSIVE_DISCOVERY_MAX_CYCLE_DURATION;
        let mut received_datagrams = 0;
        let mut cursor = PassiveDiscoveryCycleCursor::new(self.next_listener_index, listener_count);
        for _ in 0..listener_count {
            let remaining_cycle_time = cycle_deadline.saturating_duration_since(Instant::now());
            if !PassiveDiscoveryCycleCursor::should_continue(
                is_running(listener_state),
                received_datagrams,
                PASSIVE_DISCOVERY_MAX_DATAGRAMS_PER_CYCLE,
                remaining_cycle_time,
            ) {
                std::thread::yield_now();
                break;
            }
            let Some(listener_index) = cursor.take_next() else {
                break;
            };
            let remaining_budget = PASSIVE_DISCOVERY_MAX_DATAGRAMS_PER_CYCLE
                .saturating_sub(received_datagrams)
                .min(PASSIVE_DISCOVERY_MAX_DATAGRAMS_PER_SOURCE);
            received_datagrams = received_datagrams.saturating_add(self.receive_listener(
                listener_state,
                listener_index,
                remaining_budget,
                cycle_deadline,
            ));
            self.next_listener_index = cursor.resume_index();
            if !is_running(listener_state) {
                break;
            }
        }
        if received_datagrams >= PASSIVE_DISCOVERY_MAX_DATAGRAMS_PER_CYCLE
            || cycle_started.elapsed() >= PASSIVE_DISCOVERY_MAX_CYCLE_DURATION
        {
            std::thread::yield_now();
        }
    }

    fn receive_listener(
        &mut self,
        listener_state: &Arc<Mutex<LanPassiveDiscoveryListenerState>>,
        listener_index: usize,
        max_datagram_count: usize,
        cycle_deadline: Instant,
    ) -> usize {
        let Some(listener) = self.listener_slots[listener_index].listener.as_ref() else {
            return 0;
        };
        let (datagrams, issue) = listener
            .receive_bounded_until(max_datagram_count, cycle_deadline)
            .into_parts();
        let received_datagram_count = datagrams.len();
        self.ingest_datagrams(listener_state, datagrams);
        if let Some(issue) = issue {
            self.listener_slots[listener_index].record_failure(issue, Instant::now());
            self.persist_capability();
        }
        received_datagram_count
    }

    fn ingest_datagrams(
        &mut self,
        listener_state: &Arc<Mutex<LanPassiveDiscoveryListenerState>>,
        datagrams: Vec<LanPassiveDiscoveryUdpDatagram>,
    ) {
        let refresh_signals = collect_refresh_signals(listener_state, datagrams);
        self.send_refresh_signals(refresh_signals);
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
    let receipt = datagram.ingest_into(state);
    if receipt.outcome() != &LanPassiveDiscoveryPacketIngestOutcome::Recorded {
        return None;
    }
    Some(LanPassiveDiscoveryRefreshSignal {
        sequence: 0,
        source: Some(receipt.source()),
        trigger_reason: LanPassiveDiscoveryTriggerReason::PassivePacketObserved,
        observed_at: receipt.observed_at().to_string(),
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
