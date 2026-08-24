use std::time::{Duration, Instant};

use super::{socket, LanPassiveDiscoveryUdpListener, LanPassiveDiscoveryUdpReceiveBatch};

impl LanPassiveDiscoveryUdpListener {
    pub fn receive_bounded_with_timeout(
        &self,
        max_datagram_count: usize,
        read_timeout: Duration,
    ) -> LanPassiveDiscoveryUdpReceiveBatch {
        socket::receive_bounded_with_timeout(self, max_datagram_count, read_timeout)
    }

    pub fn receive_bounded_until(
        &self,
        max_datagram_count: usize,
        deadline: Instant,
    ) -> LanPassiveDiscoveryUdpReceiveBatch {
        socket::receive_bounded_until(self, max_datagram_count, deadline)
    }
}
