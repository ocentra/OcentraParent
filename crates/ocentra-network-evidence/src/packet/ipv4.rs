use super::{
    IpProtocol, Ipv4PacketMetadata, Ipv4PacketView, PacketParseError, ETHERNET_HEADER_LEN,
    IPV4_PROTOCOL_ICMP, IPV4_PROTOCOL_TCP, IPV4_PROTOCOL_UDP,
};

pub(super) fn parse_ipv4_packet(frame: &[u8]) -> Result<Ipv4PacketView<'_>, PacketParseError> {
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
            protocol: ip_protocol(protocol),
            header_len,
            total_len,
        },
        protocol,
        payload: &frame[payload_start..payload_end],
    })
}

fn ip_protocol(protocol: u8) -> IpProtocol {
    match protocol {
        IPV4_PROTOCOL_ICMP => IpProtocol::Icmp,
        IPV4_PROTOCOL_TCP => IpProtocol::Tcp,
        IPV4_PROTOCOL_UDP => IpProtocol::Udp,
        value => IpProtocol::Other(value),
    }
}

fn ipv4_text(bytes: &[u8]) -> String {
    format!("{}.{}.{}.{}", bytes[0], bytes[1], bytes[2], bytes[3])
}
