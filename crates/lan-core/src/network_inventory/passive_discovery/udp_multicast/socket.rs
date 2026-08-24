use std::time::Duration;

use super::super::{
    LanPassiveDiscoveryListenerState, LanPassiveDiscoverySource,
    LanPassiveDiscoveryUdpListenerIssue, LanPassiveDiscoveryUdpMulticastCaptureOutcome,
    LanPassiveDiscoveryUdpMulticastSupport,
};
use super::{LanPassiveDiscoveryUdpListener, LanPassiveDiscoveryUdpReceiveBatch};

mod errors;
mod listener;

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
