use std::net::Ipv4Addr;
use std::time::Duration;

use super::super::super::{
    LanPassiveDiscoverySource, LanPassiveDiscoveryUdpListenerIssue,
    LanPassiveDiscoveryUdpMulticastSupport,
};
use super::super::support::udp_multicast_support;
use super::super::LanPassiveDiscoveryUdpListener;
use super::errors::{
    bind_passive_udp_socket, join_passive_multicast_group, parse_passive_multicast_group,
    unsupported_source_issue,
};

mod receive;

pub(super) fn bind_passive_udp_listener(
    source: LanPassiveDiscoverySource,
    read_timeout: Duration,
) -> Result<LanPassiveDiscoveryUdpListener, LanPassiveDiscoveryUdpListenerIssue> {
    let support = udp_multicast_support(source);
    let (socket, multicast_group) = socket_for_support(source, &support, read_timeout)?;
    if let Some(multicast_group) = multicast_group {
        join_passive_multicast_group(&socket, source, multicast_group)?;
    }
    Ok(LanPassiveDiscoveryUdpListener { source, socket })
}

pub(super) fn receive_bounded(
    listener: &LanPassiveDiscoveryUdpListener,
    max_datagram_count: usize,
) -> super::super::LanPassiveDiscoveryUdpReceiveBatch {
    receive::receive_bounded(listener, max_datagram_count)
}

pub(super) fn receive_bounded_with_timeout(
    listener: &LanPassiveDiscoveryUdpListener,
    max_datagram_count: usize,
    read_timeout: Duration,
) -> super::super::LanPassiveDiscoveryUdpReceiveBatch {
    receive::receive_bounded_with_timeout(listener, max_datagram_count, read_timeout)
}

fn socket_for_support(
    source: LanPassiveDiscoverySource,
    support: &LanPassiveDiscoveryUdpMulticastSupport,
    read_timeout: Duration,
) -> Result<(std::net::UdpSocket, Option<Ipv4Addr>), LanPassiveDiscoveryUdpListenerIssue> {
    match support {
        LanPassiveDiscoveryUdpMulticastSupport::Available {
            multicast_group,
            port,
            ..
        } => Ok((
            bind_passive_udp_socket(source, *port, read_timeout)?,
            Some(parse_passive_multicast_group(source, multicast_group)?),
        )),
        LanPassiveDiscoveryUdpMulticastSupport::AvailableBroadcast { port, .. } => {
            Ok((bind_passive_udp_socket(source, *port, read_timeout)?, None))
        }
        LanPassiveDiscoveryUdpMulticastSupport::Unsupported { .. } => {
            Err(unsupported_source_issue(source))
        }
    }
}
