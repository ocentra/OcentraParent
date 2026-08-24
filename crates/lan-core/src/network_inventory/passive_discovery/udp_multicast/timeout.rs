use std::time::Duration;

use super::{socket, LanPassiveDiscoveryUdpListener, LanPassiveDiscoveryUdpReceiveBatch};

impl LanPassiveDiscoveryUdpListener {
    pub fn receive_bounded_with_timeout(
        &self,
        max_datagram_count: usize,
        read_timeout: Duration,
    ) -> LanPassiveDiscoveryUdpReceiveBatch {
        socket::receive_bounded_with_timeout(self, max_datagram_count, read_timeout)
    }
}
