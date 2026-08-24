use std::net::UdpSocket;

use super::super::super::{
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
    let mut buffer = vec![0_u8; LAN_PASSIVE_DISCOVERY_MAX_PACKET_BYTES];
    while received_datagram_count < max_datagram_count {
        match socket.recv_from(&mut buffer) {
            Ok((received, _)) => {
                received_datagram_count += 1;
                let observed_at = observed_at();
                let _ = super::ingest_native_passive_datagram_with_observed_at(
                    state,
                    &source,
                    &buffer[..received],
                    &observed_at,
                );
            }
            Err(error)
                if error.kind() == std::io::ErrorKind::WouldBlock
                    || error.kind() == std::io::ErrorKind::TimedOut =>
            {
                break;
            }
            Err(_) => break,
        }
    }
    received_datagram_count
}
