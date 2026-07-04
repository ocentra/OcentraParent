use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};
use std::time::Duration;

use chrono::Utc;

use super::summaries::{passive_native_datagram_device_id, passive_native_datagram_summary};
use super::{
    LanPassiveDiscoveryListenerState, LanPassiveDiscoveryPacketIngestOutcome,
    LanPassiveDiscoveryPacketParseError, LanPassiveDiscoveryRecordOutcome,
    LanPassiveDiscoverySource, LanPassiveDiscoveryTriggerReason,
    LanPassiveDiscoveryUdpMulticastCaptureOutcome, LanPassiveDiscoveryUdpMulticastSupport,
    LAN_PASSIVE_DISCOVERY_MAX_PACKET_BYTES,
};

pub fn udp_multicast_support(
    source: LanPassiveDiscoverySource,
) -> LanPassiveDiscoveryUdpMulticastSupport {
    match source {
        LanPassiveDiscoverySource::Mdns => multicast_available(source, "224.0.0.251", 5353),
        LanPassiveDiscoverySource::Ssdp => multicast_available(source, "239.255.255.250", 1900),
        LanPassiveDiscoverySource::WsDiscovery => {
            multicast_available(source, "239.255.255.250", 3702)
        }
        LanPassiveDiscoverySource::Llmnr => multicast_available(source, "224.0.0.252", 5355),
        LanPassiveDiscoverySource::Dhcp => {
            LanPassiveDiscoveryUdpMulticastSupport::AvailableBroadcast { source, port: 67 }
        }
        LanPassiveDiscoverySource::Netbios => {
            LanPassiveDiscoveryUdpMulticastSupport::AvailableBroadcast { source, port: 137 }
        }
        LanPassiveDiscoverySource::Arp => unsupported_udp_source(
            source,
            "raw-socket passive capture is not implemented in lan-core",
        ),
        LanPassiveDiscoverySource::AllowedSnmpResponse => unsupported_udp_source(
            source,
            "passive SNMP response capture requires an explicit allowed probe socket",
        ),
        LanPassiveDiscoverySource::OcentraBeacon => unsupported_udp_source(
            source,
            "Ocentra beacon packets are accepted through the signed child hello path",
        ),
    }
}

pub fn collect_udp_multicast_passive_packets(
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
    let received_datagram_count =
        drain_udp_socket_packets(&socket, state, source, max_datagram_count);
    LanPassiveDiscoveryUdpMulticastCaptureOutcome::Captured {
        source,
        received_datagram_count,
    }
}

pub fn collect_allowed_snmp_response_packets(
    socket: &UdpSocket,
    state: &mut LanPassiveDiscoveryListenerState,
    max_datagram_count: usize,
) -> usize {
    drain_udp_socket_packets(
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
    ingest_passive_datagram(
        state,
        &LanPassiveDiscoverySource::AllowedSnmpResponse,
        payload,
    )
}

pub fn drain_udp_socket_packets(
    socket: &UdpSocket,
    state: &mut LanPassiveDiscoveryListenerState,
    source: LanPassiveDiscoverySource,
    max_datagram_count: usize,
) -> usize {
    drain_udp_socket_packets_with_observed_at(
        socket,
        state,
        source,
        max_datagram_count,
        &mut || Utc::now().to_rfc3339(),
    )
}

pub fn drain_udp_socket_packets_with_observed_at(
    socket: &UdpSocket,
    state: &mut LanPassiveDiscoveryListenerState,
    source: LanPassiveDiscoverySource,
    max_datagram_count: usize,
    observed_at: &mut dyn FnMut() -> String,
) -> usize {
    let mut received_datagram_count = 0_usize;
    let mut buffer = vec![0_u8; LAN_PASSIVE_DISCOVERY_MAX_PACKET_BYTES];
    while received_datagram_count < max_datagram_count {
        match socket.recv_from(&mut buffer) {
            Ok((received, _)) => {
                received_datagram_count += 1;
                let observed_at = observed_at();
                let _ = ingest_passive_datagram_with_observed_at(
                    state,
                    &source,
                    &buffer[..received],
                    &observed_at,
                );
            }
            Err(error)
                if error.kind() == std::io::ErrorKind::WouldBlock
                    || error.kind() == std::io::ErrorKind::TimedOut =>
            {
                break;
            }
            Err(_) => break,
        }
    }
    received_datagram_count
}

pub fn ingest_passive_datagram(
    state: &mut LanPassiveDiscoveryListenerState,
    source: &LanPassiveDiscoverySource,
    payload: &[u8],
) -> LanPassiveDiscoveryPacketIngestOutcome {
    let observed_at = Utc::now().to_rfc3339();
    ingest_passive_datagram_with_observed_at(state, source, payload, &observed_at)
}

pub fn ingest_passive_datagram_with_observed_at(
    state: &mut LanPassiveDiscoveryListenerState,
    source: &LanPassiveDiscoverySource,
    payload: &[u8],
    observed_at: &str,
) -> LanPassiveDiscoveryPacketIngestOutcome {
    match state.ingest_udp_packet(payload) {
        LanPassiveDiscoveryPacketIngestOutcome::Rejected(
            LanPassiveDiscoveryPacketParseError::MalformedPayload,
        ) => {}
        outcome => return outcome,
    }

    let Some(summary) = passive_native_datagram_summary(*source, payload) else {
        return LanPassiveDiscoveryPacketIngestOutcome::Rejected(
            LanPassiveDiscoveryPacketParseError::MalformedPayload,
        );
    };
    let device_id = passive_native_datagram_device_id(*source, payload);
    match state.record_passive_update(
        *source,
        LanPassiveDiscoveryTriggerReason::PassivePacketObserved,
        observed_at,
        device_id.as_ref().map(String::as_str),
        None,
        summary,
    ) {
        LanPassiveDiscoveryRecordOutcome::Recorded => {
            LanPassiveDiscoveryPacketIngestOutcome::Recorded
        }
        LanPassiveDiscoveryRecordOutcome::Deduplicated => {
            LanPassiveDiscoveryPacketIngestOutcome::Deduplicated
        }
        LanPassiveDiscoveryRecordOutcome::Stopped => {
            LanPassiveDiscoveryPacketIngestOutcome::Stopped
        }
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
        } => {
            let group = parse_passive_multicast_group(source, multicast_group)?;
            let socket = bind_passive_udp_socket(source, *port, read_timeout, true)?;
            Ok((socket, Some(group)))
        }
        LanPassiveDiscoveryUdpMulticastSupport::AvailableBroadcast { port, .. } => {
            let socket = bind_passive_udp_socket(source, *port, read_timeout, false)?;
            Ok((socket, None))
        }
        LanPassiveDiscoveryUdpMulticastSupport::Unsupported { .. } => Err(
            LanPassiveDiscoveryUdpMulticastCaptureOutcome::Unsupported(support.clone()),
        ),
    }
}

fn parse_passive_multicast_group(
    source: LanPassiveDiscoverySource,
    multicast_group: &str,
) -> Result<Ipv4Addr, LanPassiveDiscoveryUdpMulticastCaptureOutcome> {
    multicast_group.parse::<Ipv4Addr>().map_err(|_| {
        LanPassiveDiscoveryUdpMulticastCaptureOutcome::Unsupported(
            LanPassiveDiscoveryUdpMulticastSupport::Unsupported {
                source,
                reason: "invalid multicast group for passive discovery listener".to_string(),
            },
        )
    })
}

fn bind_passive_udp_socket(
    source: LanPassiveDiscoverySource,
    port: u16,
    read_timeout: Duration,
    multicast: bool,
) -> Result<UdpSocket, LanPassiveDiscoveryUdpMulticastCaptureOutcome> {
    bind_udp_multicast_socket(port, read_timeout).map_err(|_| {
        let transport = if multicast { "multicast" } else { "broadcast" };
        LanPassiveDiscoveryUdpMulticastCaptureOutcome::Unsupported(
            LanPassiveDiscoveryUdpMulticastSupport::Unsupported {
                source,
                reason: format!("unable to bind UDP {transport} listener for passive discovery"),
            },
        )
    })
}

fn join_passive_multicast_group(
    socket: &UdpSocket,
    source: LanPassiveDiscoverySource,
    multicast_group: Ipv4Addr,
) -> Result<(), LanPassiveDiscoveryUdpMulticastCaptureOutcome> {
    let Some(interface) = local_ipv4_multicast_interface() else {
        return Err(LanPassiveDiscoveryUdpMulticastCaptureOutcome::Unsupported(
            LanPassiveDiscoveryUdpMulticastSupport::Unsupported {
                source,
                reason: "no IPv4 multicast interface is available for passive discovery"
                    .to_string(),
            },
        ));
    };
    socket
        .join_multicast_v4(&multicast_group, &interface)
        .map_err(|error| {
            LanPassiveDiscoveryUdpMulticastCaptureOutcome::Unsupported(
                LanPassiveDiscoveryUdpMulticastSupport::Unsupported {
                    source,
                    reason: format!(
                        "failed to join multicast group for passive discovery: {error}"
                    ),
                },
            )
        })
}

fn multicast_available(
    source: LanPassiveDiscoverySource,
    multicast_group: &str,
    port: u16,
) -> LanPassiveDiscoveryUdpMulticastSupport {
    LanPassiveDiscoveryUdpMulticastSupport::Available {
        source,
        multicast_group: multicast_group.to_string(),
        port,
    }
}

fn unsupported_udp_source(
    source: LanPassiveDiscoverySource,
    reason: &str,
) -> LanPassiveDiscoveryUdpMulticastSupport {
    LanPassiveDiscoveryUdpMulticastSupport::Unsupported {
        source,
        reason: reason.to_string(),
    }
}

fn bind_udp_multicast_socket(port: u16, read_timeout: Duration) -> std::io::Result<UdpSocket> {
    let socket = UdpSocket::bind(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port))?;
    socket.set_read_timeout(Some(read_timeout))?;
    Ok(socket)
}

fn local_ipv4_multicast_interface() -> Option<Ipv4Addr> {
    let identity = crate::network_inventory_hardware::local_network_identity()?;
    let ip_address = identity.ip_address?;
    ip_address.parse::<Ipv4Addr>().ok()
}
