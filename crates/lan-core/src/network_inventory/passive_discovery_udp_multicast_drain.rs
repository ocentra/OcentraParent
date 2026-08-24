use std::net::UdpSocket;

use super::super::{
    LanPassiveDiscoveryListenerState, LanPassiveDiscoverySource,
    LAN_PASSIVE_DISCOVERY_MAX_PACKET_BYTES,
};

pub(super) fn drain_udp_socket_packets_with_observed_at(
    socket: &UdpSocket,
    state: &mut LanPassiveDiscoveryListenerState,
    source: LanPassiveDiscoverySource,
    max_datagram_count: usize,
    observed_at: &mut dyn FnMut() -> String,
) -> usize {
    let mut received_datagram_count = 0_usize;
    let mut receive_attempts = 0_usize;
    let max_receive_attempts = max_datagram_count.saturating_add(1);
    let mut buffer = vec![0_u8; LAN_PASSIVE_DISCOVERY_MAX_PACKET_BYTES.saturating_add(1)];
    while received_datagram_count < max_datagram_count && receive_attempts < max_receive_attempts {
        receive_attempts = receive_attempts.saturating_add(1);
        if !receive_one(socket, state, source, observed_at, &mut buffer) {
            break;
        }
        received_datagram_count += 1;
    }
    received_datagram_count
}

fn receive_one(
    socket: &UdpSocket,
    state: &mut LanPassiveDiscoveryListenerState,
    source: LanPassiveDiscoverySource,
    observed_at: &mut dyn FnMut() -> String,
    buffer: &mut [u8],
) -> bool {
    let Ok((received, _)) = socket.recv_from(buffer) else {
        return false;
    };
    if received > LAN_PASSIVE_DISCOVERY_MAX_PACKET_BYTES {
        return true;
    }
    let observed_at = observed_at();
    let _ = super::ingest::ingest_native_passive_datagram_with_observed_at(
        state,
        &source,
        &buffer[..received],
        &observed_at,
    );
    true
}
