pub mod types;

use types::{
    EthernetFrameMetadata, IpProtocol, Ipv4PacketMetadata, PacketParseError, ParsedNetworkPacket,
    TransportPacketMetadata, UdpPayloadView,
};

const ETHERNET_HEADER_LEN: usize = 14;
const ETHER_TYPE_IPV4: u16 = 0x0800;
const IPV4_PROTOCOL_ICMP: u8 = 1;
const IPV4_PROTOCOL_TCP: u8 = 6;
const IPV4_PROTOCOL_UDP: u8 = 17;
const ICMP_HEADER_LEN: usize = 4;
const TCP_HEADER_LEN: usize = 20;
const UDP_HEADER_LEN: usize = 8;

pub fn parse_network_packet(frame: &[u8]) -> Result<ParsedNetworkPacket, PacketParseError> {
    let ethernet = parse_ethernet_frame(frame)?;
    if ethernet.ether_type != ETHER_TYPE_IPV4 {
        return Ok(ParsedNetworkPacket {
            ethernet,
            ipv4: None,
            transport: None,
        });
    }

    let ipv4_view = parse_ipv4_packet(frame)?;
    let transport = parse_transport(&ipv4_view)?;
    Ok(ParsedNetworkPacket {
        ethernet,
        ipv4: Some(ipv4_view.metadata),
        transport: Some(transport),
    })
}

pub fn udp_payload_from_ethernet_ipv4(
    frame: &[u8],
) -> Result<Option<UdpPayloadView<'_>>, PacketParseError> {
    if frame.len() < ETHERNET_HEADER_LEN {
        return Err(PacketParseError::EthernetFrameTooShort);
    }
    if ether_type(frame) != ETHER_TYPE_IPV4 {
        return Ok(None);
    }

    let ipv4_view = parse_ipv4_packet(frame)?;
    if ipv4_view.protocol != IPV4_PROTOCOL_UDP {
        return Ok(None);
    }

    let udp = parse_udp_payload(&ipv4_view)?;
    Ok(Some(UdpPayloadView {
        source_ip: ipv4_view.metadata.source_ip,
        destination_ip: ipv4_view.metadata.destination_ip,
        source_port: udp.source_port,
        destination_port: udp.destination_port,
        payload: udp.payload,
    }))
}

struct Ipv4PacketView<'a> {
    metadata: Ipv4PacketMetadata,
    protocol: u8,
    payload: &'a [u8],
}

struct UdpPacketView<'a> {
    source_port: u16,
    destination_port: u16,
    payload: &'a [u8],
}

fn parse_ethernet_frame(frame: &[u8]) -> Result<EthernetFrameMetadata, PacketParseError> {
    if frame.len() < ETHERNET_HEADER_LEN {
        return Err(PacketParseError::EthernetFrameTooShort);
    }

    Ok(EthernetFrameMetadata {
        destination_mac: mac_text(&frame[0..6]),
        source_mac: mac_text(&frame[6..12]),
        ether_type: ether_type(frame),
    })
}

fn parse_ipv4_packet(frame: &[u8]) -> Result<Ipv4PacketView<'_>, PacketParseError> {
    let ip_start = ETHERNET_HEADER_LEN;
    if frame.len() < ip_start + 20 {
        return Err(PacketParseError::Ipv4HeaderTooShort);
    }

    let header_len = usize::from(frame[ip_start] & 0x0f) * 4;
    if header_len < 20 {
        return Err(PacketParseError::Ipv4HeaderTooShort);
    }

    let total_len = usize::from(u16::from_be_bytes([
        frame[ip_start + 2],
        frame[ip_start + 3],
    ]));
    if total_len < header_len || frame.len() < ip_start + total_len {
        return Err(PacketParseError::Ipv4PacketTruncated);
    }

    let protocol = frame[ip_start + 9];
    let payload_start = ip_start + header_len;
    let payload_end = ip_start + total_len;
    Ok(Ipv4PacketView {
        metadata: Ipv4PacketMetadata {
            source_ip: ipv4_text(&frame[ip_start + 12..ip_start + 16]),
            destination_ip: ipv4_text(&frame[ip_start + 16..ip_start + 20]),
            protocol: match protocol {
                IPV4_PROTOCOL_ICMP => IpProtocol::Icmp,
                IPV4_PROTOCOL_TCP => IpProtocol::Tcp,
                IPV4_PROTOCOL_UDP => IpProtocol::Udp,
                value => IpProtocol::Other(value),
            },
            header_len,
            total_len,
        },
        protocol,
        payload: &frame[payload_start..payload_end],
    })
}

fn parse_transport(view: &Ipv4PacketView<'_>) -> Result<TransportPacketMetadata, PacketParseError> {
    match view.protocol {
        IPV4_PROTOCOL_UDP => {
            let udp = parse_udp_payload(view)?;
            Ok(TransportPacketMetadata::Udp {
                source_port: udp.source_port,
                destination_port: udp.destination_port,
                payload_len: udp.payload.len(),
            })
        }
        IPV4_PROTOCOL_TCP => parse_tcp_metadata(view.payload),
        IPV4_PROTOCOL_ICMP => parse_icmp_metadata(view.payload),
        value => Ok(TransportPacketMetadata::Other {
            protocol: value,
            payload_len: view.payload.len(),
        }),
    }
}

fn parse_udp_payload<'a>(view: &Ipv4PacketView<'a>) -> Result<UdpPacketView<'a>, PacketParseError> {
    if view.payload.len() < UDP_HEADER_LEN {
        return Err(PacketParseError::UdpHeaderTooShort);
    }

    let source_port = u16::from_be_bytes([view.payload[0], view.payload[1]]);
    let destination_port = u16::from_be_bytes([view.payload[2], view.payload[3]]);
    let udp_len = usize::from(u16::from_be_bytes([view.payload[4], view.payload[5]]));
    if udp_len < UDP_HEADER_LEN || view.payload.len() < udp_len {
        return Err(PacketParseError::UdpPacketTruncated);
    }

    Ok(UdpPacketView {
        source_port,
        destination_port,
        payload: &view.payload[UDP_HEADER_LEN..udp_len],
    })
}

fn parse_tcp_metadata(payload: &[u8]) -> Result<TransportPacketMetadata, PacketParseError> {
    if payload.len() < TCP_HEADER_LEN {
        return Err(PacketParseError::TcpHeaderTooShort);
    }

    let source_port = u16::from_be_bytes([payload[0], payload[1]]);
    let destination_port = u16::from_be_bytes([payload[2], payload[3]]);
    let header_len = usize::from(payload[12] >> 4) * 4;
    if header_len < TCP_HEADER_LEN || payload.len() < header_len {
        return Err(PacketParseError::TcpSegmentTruncated);
    }

    Ok(TransportPacketMetadata::Tcp {
        source_port,
        destination_port,
        header_len,
        payload_len: payload.len() - header_len,
    })
}

fn parse_icmp_metadata(payload: &[u8]) -> Result<TransportPacketMetadata, PacketParseError> {
    if payload.len() < ICMP_HEADER_LEN {
        return Err(PacketParseError::IcmpPacketTooShort);
    }

    Ok(TransportPacketMetadata::Icmp {
        icmp_type: payload[0],
        code: payload[1],
        payload_len: payload.len() - ICMP_HEADER_LEN,
    })
}

fn ether_type(frame: &[u8]) -> u16 {
    u16::from_be_bytes([frame[12], frame[13]])
}

fn mac_text(bytes: &[u8]) -> String {
    format!(
        "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]
    )
}

fn ipv4_text(bytes: &[u8]) -> String {
    format!("{}.{}.{}.{}", bytes[0], bytes[1], bytes[2], bytes[3])
}
