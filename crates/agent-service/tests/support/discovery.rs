use std::net::UdpSocket;

use ocentra_lan_core::network_inventory::passive_discovery::{
    collect_allowed_snmp_response_packets, LanPassiveDiscoveryEventHistory,
    LanPassiveDiscoveryListenerState,
};

use crate::lan_pairing::LanPairingRuntime;

pub(crate) fn passive_discovery_history_snapshot(
    runtime: &LanPairingRuntime,
) -> LanPassiveDiscoveryEventHistory {
    match runtime.passive_discovery_listener_state.lock() {
        Ok(state) => state.snapshot(),
        Err(_) => LanPassiveDiscoveryListenerState::running(String::new()).snapshot(),
    }
}

pub(crate) fn record_allowed_snmp_probe_responses(
    runtime: &LanPairingRuntime,
    socket: &UdpSocket,
    max_datagram_count: usize,
) -> usize {
    if let Ok(mut state) = runtime.passive_discovery_listener_state.lock() {
        if !state.is_running() {
            return 0;
        }
        return collect_allowed_snmp_response_packets(socket, &mut state, max_datagram_count);
    }
    0
}
