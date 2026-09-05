use std::time::Instant;

use crate::network_inventory::passive_discovery::{
    LanPassiveDiscoveryUdpListenerIssueKind, LAN_PASSIVE_DISCOVERY_MAX_PACKET_BYTES,
};

use super::super::super::super::{
    LanPassiveDiscoveryUdpListener, LanPassiveDiscoveryUdpReceiveBatch,
};
use super::super::super::errors::listener_io_issue;
use super::super::super::timeout_guard::ReadTimeoutRestoreGuard;

pub(super) fn receive_bounded_until(
    listener: &LanPassiveDiscoveryUdpListener,
    max_datagram_count: usize,
    deadline: Instant,
) -> LanPassiveDiscoveryUdpReceiveBatch {
    if max_datagram_count == 0 || super::super::super::remaining_read_timeout(deadline).is_none() {
        return LanPassiveDiscoveryUdpReceiveBatch {
            datagrams: Vec::new(),
            issue: None,
        };
    }

    let previous_timeout = match listener.socket.read_timeout() {
        Ok(timeout) => timeout,
        Err(error) => {
            return LanPassiveDiscoveryUdpReceiveBatch {
                datagrams: Vec::new(),
                issue: Some(listener_io_issue(
                    listener.source,
                    LanPassiveDiscoveryUdpListenerIssueKind::SocketConfigurationFailed,
                    &error,
                )),
            };
        }
    };
    let mut timeout_guard = ReadTimeoutRestoreGuard::new(&listener.socket, previous_timeout);
    let mut datagrams = Vec::with_capacity(max_datagram_count);
    let mut issue = None;
    let mut buffer = vec![0_u8; LAN_PASSIVE_DISCOVERY_MAX_PACKET_BYTES.saturating_add(1)];
    let mut receive_attempts = 0_usize;
    let max_receive_attempts = max_datagram_count.saturating_add(1);
    while datagrams.len() < max_datagram_count && receive_attempts < max_receive_attempts {
        let Some(timeout) = super::super::super::remaining_read_timeout(deadline) else {
            break;
        };
        if let Err(error) = listener.socket.set_read_timeout(Some(timeout)) {
            issue = Some(listener_io_issue(
                listener.source,
                LanPassiveDiscoveryUdpListenerIssueKind::SocketConfigurationFailed,
                &error,
            ));
            break;
        }
        receive_attempts = receive_attempts.saturating_add(1);
        match super::receive_one(listener, &mut buffer, &mut datagrams) {
            super::ReceiveStep::Continue => {}
            super::ReceiveStep::Stop => break,
            super::ReceiveStep::Failed(receive_issue) => {
                issue = Some(receive_issue);
                break;
            }
        }
    }
    let mut batch = LanPassiveDiscoveryUdpReceiveBatch { datagrams, issue };
    if let Err(error) = timeout_guard.restore() {
        batch.issue = Some(listener_io_issue(
            listener.source,
            LanPassiveDiscoveryUdpListenerIssueKind::SocketConfigurationFailed,
            &error,
        ));
    }
    batch
}
