use std::time::Instant;

use super::super::super::super::{
    LanPassiveDiscoveryUdpListenerIssueKind, LAN_PASSIVE_DISCOVERY_MAX_PACKET_BYTES,
};
use super::super::super::{
    LanPassiveDiscoveryUdpDatagram, LanPassiveDiscoveryUdpListener,
    LanPassiveDiscoveryUdpReceiveBatch,
};
use super::super::errors::listener_io_issue;
mod timeout;

pub(super) fn receive_bounded(
    listener: &LanPassiveDiscoveryUdpListener,
    max_datagram_count: usize,
) -> LanPassiveDiscoveryUdpReceiveBatch {
    let mut datagrams = Vec::with_capacity(max_datagram_count);
    let mut issue = None;
    let mut buffer = vec![0_u8; LAN_PASSIVE_DISCOVERY_MAX_PACKET_BYTES.saturating_add(1)];
    let mut receive_attempts = 0_usize;
    let max_receive_attempts = max_datagram_count.saturating_add(1);
    while datagrams.len() < max_datagram_count && receive_attempts < max_receive_attempts {
        receive_attempts = receive_attempts.saturating_add(1);
        match receive_one(listener, &mut buffer, &mut datagrams) {
            ReceiveStep::Continue => {}
            ReceiveStep::Stop => break,
            ReceiveStep::Failed(receive_issue) => {
                issue = Some(receive_issue);
                break;
            }
        }
    }
    LanPassiveDiscoveryUdpReceiveBatch { datagrams, issue }
}

pub(super) fn receive_bounded_until(
    listener: &LanPassiveDiscoveryUdpListener,
    max_datagram_count: usize,
    deadline: Instant,
) -> LanPassiveDiscoveryUdpReceiveBatch {
    timeout::receive_bounded_until(listener, max_datagram_count, deadline)
}

pub(super) fn receive_bounded_with_timeout(
    listener: &LanPassiveDiscoveryUdpListener,
    max_datagram_count: usize,
    read_timeout: std::time::Duration,
) -> LanPassiveDiscoveryUdpReceiveBatch {
    let started = Instant::now();
    let deadline = started.checked_add(read_timeout).unwrap_or(started);
    timeout::receive_bounded_until(listener, max_datagram_count, deadline)
}

enum ReceiveStep {
    Continue,
    Stop,
    Failed(super::super::super::super::LanPassiveDiscoveryUdpListenerIssue),
}

fn receive_one(
    listener: &LanPassiveDiscoveryUdpListener,
    buffer: &mut [u8],
    datagrams: &mut Vec<LanPassiveDiscoveryUdpDatagram>,
) -> ReceiveStep {
    match listener.socket.recv_from(buffer) {
        Ok((received, _peer)) if received <= LAN_PASSIVE_DISCOVERY_MAX_PACKET_BYTES => {
            datagrams.push(LanPassiveDiscoveryUdpDatagram {
                source: listener.source,
                payload: buffer[..received].to_vec(),
            });
            ReceiveStep::Continue
        }
        Ok((_received, _peer)) => {
            // A datagram that filled MAX+1 bytes is a bounded oversize drop.
            // Never pass a possibly truncated payload to protocol ingestion.
            ReceiveStep::Continue
        }
        Err(error) if is_oversized_datagram_error(&error) => {
            // Windows reports a datagram larger than the receive buffer as
            // WSAEMSGSIZE instead of returning the truncated byte count.
            // The socket remains usable; consume and discard this packet.
            ReceiveStep::Continue
        }
        Err(error)
            if error.kind() == std::io::ErrorKind::WouldBlock
                || error.kind() == std::io::ErrorKind::TimedOut =>
        {
            ReceiveStep::Stop
        }
        Err(error) => ReceiveStep::Failed(listener_io_issue(
            listener.source,
            LanPassiveDiscoveryUdpListenerIssueKind::ReceiveFailed,
            &error,
        )),
    }
}

fn is_oversized_datagram_error(error: &std::io::Error) -> bool {
    #[cfg(windows)]
    const OVERSIZED_DATAGRAM_ERROR: i32 = 10040; // WSAEMSGSIZE
    #[cfg(not(windows))]
    const OVERSIZED_DATAGRAM_ERROR: i32 = 90; // EMSGSIZE

    error.raw_os_error() == Some(OVERSIZED_DATAGRAM_ERROR)
}
