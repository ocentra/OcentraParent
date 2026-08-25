use std::io;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
use std::time::Duration;

use socket2::{Domain, Protocol, Socket, Type};

use super::super::super::{
    LanPassiveDiscoverySource, LanPassiveDiscoveryUdpListenerIssue,
    LanPassiveDiscoveryUdpListenerIssueKind,
};

pub(super) fn parse_passive_multicast_group(
    source: LanPassiveDiscoverySource,
    multicast_group: &str,
) -> Result<Ipv4Addr, LanPassiveDiscoveryUdpListenerIssue> {
    multicast_group.parse::<Ipv4Addr>().map_err(|_error| {
        typed_listener_issue(
            source,
            LanPassiveDiscoveryUdpListenerIssueKind::InvalidMulticastGroup,
            None,
        )
    })
}

pub(super) fn bind_passive_udp_socket(
    source: LanPassiveDiscoverySource,
    port: u16,
    read_timeout: Duration,
) -> Result<UdpSocket, LanPassiveDiscoveryUdpListenerIssue> {
    bind_reusable_udp_socket(source, port, read_timeout)
}

pub(super) fn join_passive_multicast_group(
    socket: &UdpSocket,
    source: LanPassiveDiscoverySource,
    multicast_group: Ipv4Addr,
) -> Result<(), LanPassiveDiscoveryUdpListenerIssue> {
    let Some(interface) = local_ipv4_multicast_interface() else {
        return Err(no_local_ipv4_issue(source));
    };
    socket
        .join_multicast_v4(&multicast_group, &interface)
        .map_err(|error| {
            listener_io_issue(
                source,
                LanPassiveDiscoveryUdpListenerIssueKind::MulticastJoinFailed,
                &error,
            )
        })
}

pub(super) fn listener_io_issue(
    source: LanPassiveDiscoverySource,
    fallback_kind: LanPassiveDiscoveryUdpListenerIssueKind,
    error: &io::Error,
) -> LanPassiveDiscoveryUdpListenerIssue {
    let kind = match error.kind() {
        io::ErrorKind::AddrInUse => LanPassiveDiscoveryUdpListenerIssueKind::AddressInUse,
        io::ErrorKind::PermissionDenied if cfg!(target_vendor = "apple") => {
            LanPassiveDiscoveryUdpListenerIssueKind::AppleLocalNetworkPermissionRequired
        }
        io::ErrorKind::PermissionDenied => {
            LanPassiveDiscoveryUdpListenerIssueKind::PermissionDenied
        }
        _ => fallback_kind,
    };
    typed_listener_issue(source, kind, error.raw_os_error())
}

pub(super) fn unsupported_source_issue(
    source: LanPassiveDiscoverySource,
) -> LanPassiveDiscoveryUdpListenerIssue {
    typed_listener_issue(
        source,
        LanPassiveDiscoveryUdpListenerIssueKind::UnsupportedSource,
        None,
    )
}

fn no_local_ipv4_issue(source: LanPassiveDiscoverySource) -> LanPassiveDiscoveryUdpListenerIssue {
    let kind = if cfg!(target_vendor = "apple") {
        LanPassiveDiscoveryUdpListenerIssueKind::AppleLocalNetworkPermissionRequired
    } else {
        LanPassiveDiscoveryUdpListenerIssueKind::NoLocalIpv4Interface
    };
    typed_listener_issue(source, kind, None)
}

fn typed_listener_issue(
    source: LanPassiveDiscoverySource,
    kind: LanPassiveDiscoveryUdpListenerIssueKind,
    os_error_code: Option<i32>,
) -> LanPassiveDiscoveryUdpListenerIssue {
    LanPassiveDiscoveryUdpListenerIssue {
        source,
        kind,
        os_error_code,
    }
}

fn bind_reusable_udp_socket(
    source: LanPassiveDiscoverySource,
    port: u16,
    read_timeout: Duration,
) -> Result<UdpSocket, LanPassiveDiscoveryUdpListenerIssue> {
    let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP)).map_err(|error| {
        listener_io_issue(
            source,
            LanPassiveDiscoveryUdpListenerIssueKind::SocketConfigurationFailed,
            &error,
        )
    })?;
    socket.set_reuse_address(true).map_err(|error| {
        listener_io_issue(
            source,
            LanPassiveDiscoveryUdpListenerIssueKind::SocketConfigurationFailed,
            &error,
        )
    })?;
    #[cfg(all(
        unix,
        not(any(
            target_os = "solaris",
            target_os = "illumos",
            target_os = "cygwin",
            target_os = "wasi"
        ))
    ))]
    socket.set_reuse_port(true).map_err(|error| {
        listener_io_issue(
            source,
            LanPassiveDiscoveryUdpListenerIssueKind::SocketConfigurationFailed,
            &error,
        )
    })?;
    let address = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port));
    socket.bind(&address.into()).map_err(|error| {
        listener_io_issue(
            source,
            LanPassiveDiscoveryUdpListenerIssueKind::BindFailed,
            &error,
        )
    })?;
    let socket = UdpSocket::from(socket);
    socket
        .set_read_timeout(Some(read_timeout))
        .map_err(|error| {
            listener_io_issue(
                source,
                LanPassiveDiscoveryUdpListenerIssueKind::SocketConfigurationFailed,
                &error,
            )
        })?;
    Ok(socket)
}

fn local_ipv4_multicast_interface() -> Option<Ipv4Addr> {
    crate::network_inventory_hardware::local_network_identity_with_timeout(Duration::from_millis(
        250,
    ))?
    .ip_address?
    .parse::<Ipv4Addr>()
    .ok()
}
