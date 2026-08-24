use std::time::Duration;

use super::super::{
    LanPassiveDiscoveryListenerState, LanPassiveDiscoverySource,
    LanPassiveDiscoveryUdpMulticastCaptureOutcome,
};
use super::{LanPassiveDiscoveryUdpDatagram, LanPassiveDiscoveryUdpListener};

mod errors;
mod listener;

pub(super) fn collect_udp_multicast_passive_packets(
    state: &mut LanPassiveDiscoveryListenerState,
    source: LanPassiveDiscoverySource,
    max_datagram_count: usize,
    read_timeout: Duration,
) -> LanPassiveDiscoveryUdpMulticastCaptureOutcome {
    let listener = match bind_passive_udp_listener(source, read_timeout) {
        Ok(listener) => listener,
        Err(outcome) => return outcome,
    };
    let datagrams = listener
        .receive_bounded(max_datagram_count)
        .unwrap_or_default();
    let received_datagram_count = datagrams.len();
    for datagram in datagrams {
        let _ =
            super::ingest::ingest_passive_datagram(state, &datagram.source(), datagram.payload());
    }
    LanPassiveDiscoveryUdpMulticastCaptureOutcome::Captured {
        source,
        received_datagram_count,
    }
}

pub(super) fn bind_passive_udp_listener(
    source: LanPassiveDiscoverySource,
    read_timeout: Duration,
) -> Result<LanPassiveDiscoveryUdpListener, LanPassiveDiscoveryUdpMulticastCaptureOutcome> {
    listener::bind_passive_udp_listener(source, read_timeout)
}

pub(super) fn receive_bounded(
    listener: &LanPassiveDiscoveryUdpListener,
    max_datagram_count: usize,
) -> std::io::Result<Vec<LanPassiveDiscoveryUdpDatagram>> {
    listener::receive_bounded(listener, max_datagram_count)
}
