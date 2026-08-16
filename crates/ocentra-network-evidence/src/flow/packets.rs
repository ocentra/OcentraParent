use crate::flow::{NetworkFlowError, NetworkFlowPacket, NetworkFlowProtocol};
use crate::packet::{parse_network_packet, types::TransportPacketMetadata};
use crate::pcap::parse_pcap_packets;

pub(super) fn flow_packets_from_pcap(
    bytes: &[u8],
) -> Result<Vec<NetworkFlowPacket>, NetworkFlowError> {
    let packets = parse_pcap_packets(bytes).map_err(NetworkFlowError::Pcap)?;
    let mut flow_packets = Vec::new();
    for packet in packets {
        let observed_at_micros =
            u64::from(packet.timestamp_seconds) * 1_000_000 + u64::from(packet.timestamp_fraction);
        if let Some(flow_packet) = flow_packet_from_frame(&packet.data, observed_at_micros)? {
            flow_packets.push(flow_packet);
        }
    }

    Ok(flow_packets)
}

fn flow_packet_from_frame(
    frame: &[u8],
    observed_at_micros: u64,
) -> Result<Option<NetworkFlowPacket>, NetworkFlowError> {
    let parsed = parse_network_packet(frame).map_err(NetworkFlowError::Packet)?;
    let Some(ipv4) = parsed.ipv4 else {
        return Ok(None);
    };
    let Some(transport) = parsed.transport else {
        return Ok(None);
    };
    let (source_port, destination_port, protocol) = transport_flow_tuple(&transport);

    Ok(Some(NetworkFlowPacket {
        source_ip: ipv4.source_ip,
        destination_ip: ipv4.destination_ip,
        source_port,
        destination_port,
        protocol,
        observed_at_micros,
        observed_bytes: ipv4.total_len,
    }))
}

fn transport_flow_tuple(transport: &TransportPacketMetadata) -> (u16, u16, NetworkFlowProtocol) {
    match transport {
        TransportPacketMetadata::Udp {
            source_port,
            destination_port,
            payload_len: _,
        } => (*source_port, *destination_port, NetworkFlowProtocol::Udp),
        TransportPacketMetadata::Tcp {
            source_port,
            destination_port,
            header_len: _,
            payload_len: _,
        } => (*source_port, *destination_port, NetworkFlowProtocol::Tcp),
        TransportPacketMetadata::Icmp {
            icmp_type: _,
            code: _,
            payload_len: _,
        } => (0, 0, NetworkFlowProtocol::Icmp),
        TransportPacketMetadata::Other {
            protocol,
            payload_len: _,
        } => (0, 0, NetworkFlowProtocol::Other(*protocol)),
    }
}
