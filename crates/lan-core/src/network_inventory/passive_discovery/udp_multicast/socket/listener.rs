use std::net::Ipv4Addr;
use std::time::Duration;

use super::super::super::{
    LanPassiveDiscoverySource, LanPassiveDiscoveryUdpMulticastCaptureOutcome,
    LanPassiveDiscoveryUdpMulticastSupport, LAN_PASSIVE_DISCOVERY_MAX_PACKET_BYTES,
};
use super::super::support::udp_multicast_support;
use super::super::{LanPassiveDiscoveryUdpDatagram, LanPassiveDiscoveryUdpListener};
use super::errors::{
    bind_passive_udp_socket, join_passive_multicast_group, parse_passive_multicast_group,
};

pub(super) fn bind_passive_udp_listener(
    source: LanPassiveDiscoverySource,
    read_timeout: Duration,
) -> Result<LanPassiveDiscoveryUdpListener, LanPassiveDiscoveryUdpMulticastCaptureOutcome> {
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
) -> std::io::Result<Vec<LanPassiveDiscoveryUdpDatagram>> {
    let mut datagrams = Vec::with_capacity(max_datagram_count);
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
            Err(_error) if !datagrams.is_empty() => break,
            Err(error) => return Err(error),
        }
    }
    Ok(datagrams)
}

fn socket_for_support(
    source: LanPassiveDiscoverySource,
    support: &LanPassiveDiscoveryUdpMulticastSupport,
    read_timeout: Duration,
) -> Result<(std::net::UdpSocket, Option<Ipv4Addr>), LanPassiveDiscoveryUdpMulticastCaptureOutcome>
{
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
