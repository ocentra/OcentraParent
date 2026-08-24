use std::time::Duration;

use crate::network_inventory::passive_discovery::LanPassiveDiscoveryUdpListenerIssueKind;

use super::super::super::super::{
    LanPassiveDiscoveryUdpListener, LanPassiveDiscoveryUdpReceiveBatch,
};
use super::super::super::errors::listener_io_issue;

pub(super) fn receive_bounded_with_timeout(
    listener: &LanPassiveDiscoveryUdpListener,
    max_datagram_count: usize,
    read_timeout: Duration,
) -> LanPassiveDiscoveryUdpReceiveBatch {
    if max_datagram_count == 0 || read_timeout.is_zero() {
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
    if let Err(error) = listener.socket.set_read_timeout(Some(read_timeout)) {
        return LanPassiveDiscoveryUdpReceiveBatch {
            datagrams: Vec::new(),
            issue: Some(listener_io_issue(
                listener.source,
                LanPassiveDiscoveryUdpListenerIssueKind::SocketConfigurationFailed,
                &error,
            )),
        };
    }

    let mut batch = super::receive_bounded(listener, max_datagram_count);
    if let Err(error) = listener.socket.set_read_timeout(previous_timeout) {
        batch.issue = Some(listener_io_issue(
            listener.source,
            LanPassiveDiscoveryUdpListenerIssueKind::SocketConfigurationFailed,
            &error,
        ));
    }
    batch
}
