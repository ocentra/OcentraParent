use std::net::UdpSocket;
use std::time::Duration;

use super::{
    LanPassiveDiscoveryListenerState, LanPassiveDiscoveryPacketIngestOutcome,
    LanPassiveDiscoverySource, LanPassiveDiscoveryUdpListenerIssue,
    LanPassiveDiscoveryUdpMulticastCaptureOutcome, LanPassiveDiscoveryUdpMulticastSupport,
};

mod ingest;
mod socket;
mod support;

pub struct LanPassiveDiscoveryUdpListener {
    source: LanPassiveDiscoverySource,
    socket: UdpSocket,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LanPassiveDiscoveryUdpDatagram {
    source: LanPassiveDiscoverySource,
    payload: Vec<u8>,
}

#[derive(Debug)]
pub struct LanPassiveDiscoveryUdpReceiveBatch {
    datagrams: Vec<LanPassiveDiscoveryUdpDatagram>,
    issue: Option<LanPassiveDiscoveryUdpListenerIssue>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LanPassiveDiscoveryUdpDatagramIngestReceipt {
    source: LanPassiveDiscoverySource,
    observed_at: String,
    outcome: LanPassiveDiscoveryPacketIngestOutcome,
}

impl LanPassiveDiscoveryUdpListener {
    pub fn source(&self) -> LanPassiveDiscoverySource {
        self.source
    }

    pub fn receive_bounded(&self, max_datagram_count: usize) -> LanPassiveDiscoveryUdpReceiveBatch {
        socket::receive_bounded(self, max_datagram_count)
    }
}

impl LanPassiveDiscoveryUdpDatagram {
    pub fn source(&self) -> LanPassiveDiscoverySource {
        self.source
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    pub fn ingest_into(
        self,
        state: &mut LanPassiveDiscoveryListenerState,
    ) -> LanPassiveDiscoveryUdpDatagramIngestReceipt {
        let observed_at = chrono::Utc::now().to_rfc3339();
        let outcome = ingest::ingest_native_passive_datagram_with_observed_at(
            state,
            &self.source,
            &self.payload,
            &observed_at,
        );
        LanPassiveDiscoveryUdpDatagramIngestReceipt {
            source: self.source,
            observed_at,
            outcome,
        }
    }
}

impl LanPassiveDiscoveryUdpDatagramIngestReceipt {
    pub fn source(&self) -> LanPassiveDiscoverySource {
        self.source
    }

    pub fn observed_at(&self) -> &str {
        &self.observed_at
    }

    pub fn outcome(&self) -> &LanPassiveDiscoveryPacketIngestOutcome {
        &self.outcome
    }
}

impl LanPassiveDiscoveryUdpReceiveBatch {
    pub fn into_parts(
        self,
    ) -> (
        Vec<LanPassiveDiscoveryUdpDatagram>,
        Option<LanPassiveDiscoveryUdpListenerIssue>,
    ) {
        (self.datagrams, self.issue)
    }
}

pub fn udp_multicast_support(
    source: LanPassiveDiscoverySource,
) -> LanPassiveDiscoveryUdpMulticastSupport {
    support::udp_multicast_support(source)
}

pub fn collect_udp_multicast_passive_packets(
    state: &mut LanPassiveDiscoveryListenerState,
    source: LanPassiveDiscoverySource,
    max_datagram_count: usize,
    read_timeout: Duration,
) -> LanPassiveDiscoveryUdpMulticastCaptureOutcome {
    socket::collect_udp_multicast_passive_packets(state, source, max_datagram_count, read_timeout)
}

pub fn bind_passive_udp_listener(
    source: LanPassiveDiscoverySource,
    read_timeout: Duration,
) -> Result<LanPassiveDiscoveryUdpListener, LanPassiveDiscoveryUdpListenerIssue> {
    socket::bind_passive_udp_listener(source, read_timeout)
}

pub fn collect_allowed_snmp_response_packets(
    socket: &UdpSocket,
    state: &mut LanPassiveDiscoveryListenerState,
    max_datagram_count: usize,
) -> usize {
    ingest::drain_udp_socket_packets(
        socket,
        state,
        LanPassiveDiscoverySource::AllowedSnmpResponse,
        max_datagram_count,
    )
}

pub fn ingest_allowed_snmp_response_packet(
    state: &mut LanPassiveDiscoveryListenerState,
    payload: &[u8],
) -> LanPassiveDiscoveryPacketIngestOutcome {
    let observed_at = chrono::Utc::now().to_rfc3339();
    ingest::ingest_native_passive_datagram_with_observed_at(
        state,
        &LanPassiveDiscoverySource::AllowedSnmpResponse,
        payload,
        &observed_at,
    )
}

pub fn drain_udp_socket_packets(
    socket: &UdpSocket,
    state: &mut LanPassiveDiscoveryListenerState,
    source: LanPassiveDiscoverySource,
    max_datagram_count: usize,
) -> usize {
    ingest::drain_udp_socket_packets(socket, state, source, max_datagram_count)
}

pub fn drain_udp_socket_packets_with_observed_at(
    socket: &UdpSocket,
    state: &mut LanPassiveDiscoveryListenerState,
    source: LanPassiveDiscoverySource,
    max_datagram_count: usize,
    observed_at: &mut dyn FnMut() -> String,
) -> usize {
    ingest::drain_udp_socket_packets_with_observed_at(
        socket,
        state,
        source,
        max_datagram_count,
        observed_at,
    )
}

pub fn ingest_passive_datagram(
    state: &mut LanPassiveDiscoveryListenerState,
    source: &LanPassiveDiscoverySource,
    payload: &[u8],
) -> LanPassiveDiscoveryPacketIngestOutcome {
    ingest::ingest_passive_datagram(state, source, payload)
}

pub fn ingest_passive_datagram_with_observed_at(
    state: &mut LanPassiveDiscoveryListenerState,
    source: &LanPassiveDiscoverySource,
    payload: &[u8],
    observed_at: &str,
) -> LanPassiveDiscoveryPacketIngestOutcome {
    ingest::ingest_passive_datagram_with_observed_at(state, source, payload, observed_at)
}
