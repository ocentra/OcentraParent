use serde::{Deserialize, Serialize};

use crate::packet::{parse_network_packet, PacketParseError, TransportPacketMetadata};
use crate::pcap::{parse_pcap_packets, PcapReplayError};
use crate::NetworkEvidenceGrade;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkFlowProtocol {
    Tcp,
    Udp,
    Icmp,
    Other(u8),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkFlowPacket {
    pub source_ip: String,
    pub destination_ip: String,
    pub source_port: u16,
    pub destination_port: u16,
    pub protocol: NetworkFlowProtocol,
    pub observed_at_micros: u64,
    pub observed_bytes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkFlowKey {
    pub initiator_ip: String,
    pub initiator_port: u16,
    pub responder_ip: String,
    pub responder_port: u16,
    pub protocol: NetworkFlowProtocol,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkFlowSession {
    pub key: NetworkFlowKey,
    pub first_seen_micros: u64,
    pub last_seen_micros: u64,
    pub duration_micros: u64,
    pub packet_count: usize,
    pub initiator_to_responder_packets: usize,
    pub responder_to_initiator_packets: usize,
    pub initiator_to_responder_bytes: usize,
    pub responder_to_initiator_bytes: usize,
    pub evidence_grade: NetworkEvidenceGrade,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkFlowSummary {
    pub packet_count: usize,
    pub flow_count: usize,
    pub sessions: Vec<NetworkFlowSession>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkFlowError {
    Pcap(PcapReplayError),
    Packet(PacketParseError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FlowDirection {
    InitiatorToResponder,
    ResponderToInitiator,
}

pub fn aggregate_pcap_flows(
    bytes: &[u8],
    idle_timeout_micros: u64,
) -> Result<NetworkFlowSummary, NetworkFlowError> {
    let packets = flow_packets_from_pcap(bytes)?;
    let sessions = aggregate_network_flows(&packets, idle_timeout_micros);
    Ok(NetworkFlowSummary {
        packet_count: packets.len(),
        flow_count: sessions.len(),
        sessions,
    })
}

pub fn flow_packets_from_pcap(bytes: &[u8]) -> Result<Vec<NetworkFlowPacket>, NetworkFlowError> {
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

pub fn aggregate_network_flows(
    packets: &[NetworkFlowPacket],
    idle_timeout_micros: u64,
) -> Vec<NetworkFlowSession> {
    let mut ordered_packets = packets.iter().collect::<Vec<_>>();
    ordered_packets.sort_by_key(|packet| packet.observed_at_micros);
    let mut sessions: Vec<NetworkFlowSession> = Vec::new();
    for packet in ordered_packets {
        let mut matched = false;
        for session in &mut sessions {
            if let Some(direction) = session.match_direction(packet, idle_timeout_micros) {
                session.record_packet(packet, direction);
                matched = true;
                break;
            }
        }
        if !matched {
            sessions.push(NetworkFlowSession::from_packet(packet));
        }
    }

    sessions
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

impl NetworkFlowSession {
    fn from_packet(packet: &NetworkFlowPacket) -> Self {
        Self {
            key: NetworkFlowKey {
                initiator_ip: packet.source_ip.clone(),
                initiator_port: packet.source_port,
                responder_ip: packet.destination_ip.clone(),
                responder_port: packet.destination_port,
                protocol: packet.protocol,
            },
            first_seen_micros: packet.observed_at_micros,
            last_seen_micros: packet.observed_at_micros,
            duration_micros: 0,
            packet_count: 1,
            initiator_to_responder_packets: 1,
            responder_to_initiator_packets: 0,
            initiator_to_responder_bytes: packet.observed_bytes,
            responder_to_initiator_bytes: 0,
            evidence_grade: NetworkEvidenceGrade::C,
            exact_url_available: false,
            decrypted_payload_available: false,
        }
    }

    fn match_direction(
        &self,
        packet: &NetworkFlowPacket,
        idle_timeout_micros: u64,
    ) -> Option<FlowDirection> {
        if packet.protocol != self.key.protocol {
            return None;
        }
        if packet.observed_at_micros > self.last_seen_micros.saturating_add(idle_timeout_micros) {
            return None;
        }
        if self.matches_initiator_to_responder(packet) {
            return Some(FlowDirection::InitiatorToResponder);
        }
        if self.matches_responder_to_initiator(packet) {
            return Some(FlowDirection::ResponderToInitiator);
        }

        None
    }

    fn matches_initiator_to_responder(&self, packet: &NetworkFlowPacket) -> bool {
        packet.source_ip == self.key.initiator_ip
            && packet.source_port == self.key.initiator_port
            && packet.destination_ip == self.key.responder_ip
            && packet.destination_port == self.key.responder_port
    }

    fn matches_responder_to_initiator(&self, packet: &NetworkFlowPacket) -> bool {
        packet.source_ip == self.key.responder_ip
            && packet.source_port == self.key.responder_port
            && packet.destination_ip == self.key.initiator_ip
            && packet.destination_port == self.key.initiator_port
    }

    fn record_packet(&mut self, packet: &NetworkFlowPacket, direction: FlowDirection) {
        self.last_seen_micros = self.last_seen_micros.max(packet.observed_at_micros);
        self.duration_micros = self.last_seen_micros - self.first_seen_micros;
        self.packet_count += 1;
        match direction {
            FlowDirection::InitiatorToResponder => {
                self.initiator_to_responder_packets += 1;
                self.initiator_to_responder_bytes += packet.observed_bytes;
            }
            FlowDirection::ResponderToInitiator => {
                self.responder_to_initiator_packets += 1;
                self.responder_to_initiator_bytes += packet.observed_bytes;
            }
        }
    }
}
