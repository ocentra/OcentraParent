use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};
use std::time::Duration;

use super::super::super::{
    LanPassiveDiscoverySource, LanPassiveDiscoveryUdpMulticastCaptureOutcome,
    LanPassiveDiscoveryUdpMulticastSupport,
};

pub(super) fn parse_passive_multicast_group(
    source: LanPassiveDiscoverySource,
    multicast_group: &str,
) -> Result<Ipv4Addr, LanPassiveDiscoveryUdpMulticastCaptureOutcome> {
    multicast_group.parse::<Ipv4Addr>().map_err(|_error| {
        unsupported_capture(
            source,
            "invalid multicast group for passive discovery listener".to_string(),
        )
    })
}

pub(super) fn bind_passive_udp_socket(
    source: LanPassiveDiscoverySource,
    port: u16,
    read_timeout: Duration,
    multicast: bool,
) -> Result<UdpSocket, LanPassiveDiscoveryUdpMulticastCaptureOutcome> {
    bind_udp_multicast_socket(port, read_timeout).map_err(|_error| {
        let transport = if multicast { "multicast" } else { "broadcast" };
        unsupported_capture(
            source,
            format!("unable to bind UDP {transport} listener for passive discovery"),
        )
    })
}

pub(super) fn join_passive_multicast_group(
    socket: &UdpSocket,
    source: LanPassiveDiscoverySource,
    multicast_group: Ipv4Addr,
) -> Result<(), LanPassiveDiscoveryUdpMulticastCaptureOutcome> {
    let Some(interface) = local_ipv4_multicast_interface() else {
        return Err(unsupported_capture(
            source,
            "no IPv4 multicast interface is available for passive discovery".to_string(),
        ));
    };
    socket
        .join_multicast_v4(&multicast_group, &interface)
        .map_err(|error| {
            unsupported_capture(
                source,
                format!("failed to join multicast group for passive discovery: {error}"),
            )
        })
}

fn unsupported_capture(
    source: LanPassiveDiscoverySource,
    reason: String,
) -> LanPassiveDiscoveryUdpMulticastCaptureOutcome {
    LanPassiveDiscoveryUdpMulticastCaptureOutcome::Unsupported(
        LanPassiveDiscoveryUdpMulticastSupport::Unsupported { source, reason },
    )
}

fn bind_udp_multicast_socket(port: u16, read_timeout: Duration) -> std::io::Result<UdpSocket> {
    let socket = UdpSocket::bind(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port))?;
    socket.set_read_timeout(Some(read_timeout))?;
    Ok(socket)
}

fn local_ipv4_multicast_interface() -> Option<Ipv4Addr> {
    crate::network_inventory_hardware::local_network_identity()?
        .ip_address?
        .parse::<Ipv4Addr>()
        .ok()
}
