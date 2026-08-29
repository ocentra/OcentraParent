use std::net::UdpSocket;
use std::time::{Duration, Instant};

use super::super::{
    LanPassiveDiscoveryListenerState, LanPassiveDiscoverySource,
    LanPassiveDiscoveryUdpListenerIssue, LanPassiveDiscoveryUdpMulticastCaptureOutcome,
    LanPassiveDiscoveryUdpMulticastSupport,
};
use super::{LanPassiveDiscoveryUdpListener, LanPassiveDiscoveryUdpReceiveBatch};

mod errors;
mod listener;
mod deadline;
mod timeout_guard;

pub(super) fn collect_udp_multicast_passive_packets(
    state: &mut LanPassiveDiscoveryListenerState,
    source: LanPassiveDiscoverySource,
    max_datagram_count: usize,
    read_timeout: Duration,
) -> LanPassiveDiscoveryUdpMulticastCaptureOutcome {
    let support = super::support::udp_multicast_support(source);
    if matches!(
        support,
        LanPassiveDiscoveryUdpMulticastSupport::Unsupported { .. }
    ) {
        return LanPassiveDiscoveryUdpMulticastCaptureOutcome::Unsupported(support);
    }
    let listener = match bind_passive_udp_listener(source, read_timeout) {
        Ok(listener) => listener,
        Err(issue) => {
            return LanPassiveDiscoveryUdpMulticastCaptureOutcome::Failed {
                source,
                received_datagram_count: 0,
                issue,
            };
        }
    };
    let (datagrams, issue) = listener.receive_bounded(max_datagram_count).into_parts();
    let received_datagram_count = datagrams.len();
    for datagram in datagrams {
        let _receipt = datagram.ingest_into(state);
    }
    if let Some(issue) = issue {
        return LanPassiveDiscoveryUdpMulticastCaptureOutcome::Failed {
            source,
            received_datagram_count,
            issue,
        };
    }
    LanPassiveDiscoveryUdpMulticastCaptureOutcome::Captured {
        source,
        received_datagram_count,
    }
}

pub(super) fn bind_passive_udp_listener(
    source: LanPassiveDiscoverySource,
    read_timeout: Duration,
) -> Result<LanPassiveDiscoveryUdpListener, LanPassiveDiscoveryUdpListenerIssue> {
    listener::bind_passive_udp_listener(source, read_timeout)
}

pub(super) fn receive_bounded(
    listener: &LanPassiveDiscoveryUdpListener,
    max_datagram_count: usize,
) -> LanPassiveDiscoveryUdpReceiveBatch {
    listener::receive_bounded(listener, max_datagram_count)
}

pub(super) fn receive_bounded_with_timeout(
    listener: &LanPassiveDiscoveryUdpListener,
    max_datagram_count: usize,
    read_timeout: Duration,
) -> LanPassiveDiscoveryUdpReceiveBatch {
    listener::receive_bounded_with_timeout(listener, max_datagram_count, read_timeout)
}

pub(super) fn receive_bounded_until(
    listener: &LanPassiveDiscoveryUdpListener,
    max_datagram_count: usize,
    deadline: Instant,
) -> LanPassiveDiscoveryUdpReceiveBatch {
    listener::receive_bounded_until(listener, max_datagram_count, deadline)
}

pub(super) fn remaining_read_timeout(deadline: Instant) -> Option<Duration> {
    deadline::remaining_read_timeout_at(deadline, Instant::now())
}
