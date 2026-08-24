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
    let remaining = deadline.saturating_duration_since(Instant::now());
    if remaining.is_zero() {
        return None;
    }
    #[cfg(windows)]
    if remaining < Duration::from_millis(1) {
        return None;
    }
    Some(remaining)
}

pub(super) struct ReadTimeoutRestoreGuard<'a> {
    socket: &'a UdpSocket,
    previous_timeout: Option<Duration>,
    armed: bool,
}

impl<'a> ReadTimeoutRestoreGuard<'a> {
    pub(super) fn new(socket: &'a UdpSocket, previous_timeout: Option<Duration>) -> Self {
        Self {
            socket,
            previous_timeout,
            armed: true,
        }
    }

    pub(super) fn restore(&mut self) -> std::io::Result<()> {
        if !self.armed {
            return Ok(());
        }
        let result = self.socket.set_read_timeout(self.previous_timeout);
        if result.is_ok() {
            self.armed = false;
        }
        result
    }
}

impl Drop for ReadTimeoutRestoreGuard<'_> {
    fn drop(&mut self) {
        if self.armed {
            let _ = self.socket.set_read_timeout(self.previous_timeout);
        }
    }
}
