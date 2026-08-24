use std::net::Ipv4Addr;
use std::time::Duration;

use super::super::super::{
    LanPassiveDiscoverySource, LanPassiveDiscoveryUdpListenerIssue,
    LanPassiveDiscoveryUdpListenerIssueKind, LanPassiveDiscoveryUdpMulticastSupport,
    LAN_PASSIVE_DISCOVERY_MAX_PACKET_BYTES,
};
use super::super::support::udp_multicast_support;
use super::super::{
    LanPassiveDiscoveryUdpDatagram, LanPassiveDiscoveryUdpListener,
    LanPassiveDiscoveryUdpReceiveBatch,
};
use super::errors::{
    bind_passive_udp_socket, join_passive_multicast_group, listener_io_issue,
    parse_passive_multicast_group, unsupported_source_issue,
};

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
) -> LanPassiveDiscoveryUdpReceiveBatch {
    let mut datagrams = Vec::with_capacity(max_datagram_count);
    let mut issue = None;
    let mut buffer = vec![0_u8; LAN_PASSIVE_DISCOVERY_MAX_PACKET_BYTES];
    while datagrams.len() < max_datagram_count {
        match listener.socket.recv_from(&mut buffer) {
            Ok((received, _peer)) => datagrams.push(LanPassiveDiscoveryUdpDatagram {
                source: listener.source,
                payload: buffer[..received].to_vec(),
            }),
            Err(error)
                if error.kind() == std::io::ErrorKind::WouldBlock
                    || error.kind() == std::io::ErrorKind::TimedOut =>
            {
                break;
            }
            Err(error) => {
                issue = Some(listener_io_issue(
                    listener.source,
                    LanPassiveDiscoveryUdpListenerIssueKind::ReceiveFailed,
                    &error,
                ));
                break;
            }
        }
    }
    LanPassiveDiscoveryUdpReceiveBatch { datagrams, issue }
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
