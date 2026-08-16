mod parsing;

pub mod types;

use parsing::{
    ether_type, parse_ethernet_frame, parse_ipv4_packet, parse_transport, parse_udp_payload,
};
use types::{IpProtocol, PacketParseError, ParsedNetworkPacket, UdpPayloadView};

const ETHER_TYPE_IPV4: u16 = 0x0800;

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
    parse_ethernet_frame(frame)?;
    if ether_type(frame) != ETHER_TYPE_IPV4 {
        return Ok(None);
    }

    let ipv4_view = parse_ipv4_packet(frame)?;
    if ipv4_view.metadata.protocol != IpProtocol::Udp {
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
