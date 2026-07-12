use std::net::{Ipv4Addr, UdpSocket};
use std::time::Duration;

use self::errors::{
    bind_passive_udp_socket, join_passive_multicast_group, parse_passive_multicast_group,
};
use super::super::{
    LanPassiveDiscoveryListenerState, LanPassiveDiscoverySource,
    LanPassiveDiscoveryUdpMulticastCaptureOutcome, LanPassiveDiscoveryUdpMulticastSupport,
};
use super::ingest::drain_udp_socket_packets;
use super::support::udp_multicast_support;

mod errors;

pub(super) fn collect_udp_multicast_passive_packets(
    state: &mut LanPassiveDiscoveryListenerState,
    source: LanPassiveDiscoverySource,
    max_datagram_count: usize,
    read_timeout: Duration,
) -> LanPassiveDiscoveryUdpMulticastCaptureOutcome {
    let support = udp_multicast_support(source);
    let (socket, multicast_group) =
        match passive_udp_socket_for_support(source, &support, read_timeout) {
            Ok(value) => value,
            Err(outcome) => return outcome,
        };
    if let Some(multicast_group) = multicast_group {
        if let Err(outcome) = join_passive_multicast_group(&socket, source, multicast_group) {
            return outcome;
        }
    }
    LanPassiveDiscoveryUdpMulticastCaptureOutcome::Captured {
        source,
        received_datagram_count: drain_udp_socket_packets(
            &socket,
            state,
            source,
            max_datagram_count,
        ),
    }
}

fn passive_udp_socket_for_support(
    source: LanPassiveDiscoverySource,
    support: &LanPassiveDiscoveryUdpMulticastSupport,
    read_timeout: Duration,
) -> Result<(UdpSocket, Option<Ipv4Addr>), LanPassiveDiscoveryUdpMulticastCaptureOutcome> {
    match support {
        LanPassiveDiscoveryUdpMulticastSupport::Available {
            multicast_group,
            port,
            ..
        } => Ok((
            bind_passive_udp_socket(source, *port, read_timeout, true)?,
            Some(parse_passive_multicast_group(source, multicast_group)?),
        )),
        LanPassiveDiscoveryUdpMulticastSupport::AvailableBroadcast { port, .. } => Ok((
            bind_passive_udp_socket(source, *port, read_timeout, false)?,
            None,
        )),
        LanPassiveDiscoveryUdpMulticastSupport::Unsupported { .. } => Err(
            LanPassiveDiscoveryUdpMulticastCaptureOutcome::Unsupported(support.clone()),
        ),
    }
}
