use super::{
    Ipv4PacketView, PacketParseError, TransportPacketMetadata, UdpPacketView, ICMP_HEADER_LEN,
    IPV4_PROTOCOL_ICMP, IPV4_PROTOCOL_TCP, IPV4_PROTOCOL_UDP, TCP_HEADER_LEN, UDP_HEADER_LEN,
};

pub(super) fn parse_transport(
    view: &Ipv4PacketView<'_>,
) -> Result<TransportPacketMetadata, PacketParseError> {
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

pub(super) fn parse_udp_payload<'a>(
    view: &Ipv4PacketView<'a>,
) -> Result<UdpPacketView<'a>, PacketParseError> {
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
