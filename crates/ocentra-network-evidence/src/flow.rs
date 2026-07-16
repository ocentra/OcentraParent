use serde::{Deserialize, Serialize};

use crate::dns::types::NetworkEvidenceGrade;
use crate::packet::types::PacketParseError;
use crate::pcap::PcapReplayError;

mod packets;
mod session;

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

pub fn aggregate_pcap_flows(
    bytes: &[u8],
    idle_timeout_micros: u64,
) -> Result<NetworkFlowSummary, NetworkFlowError> {
    let packets = packets::flow_packets_from_pcap(bytes)?;
    let sessions = session::aggregate_network_flows(&packets, idle_timeout_micros);
    Ok(NetworkFlowSummary {
        packet_count: packets.len(),
        flow_count: sessions.len(),
        sessions,
    })
}

pub fn aggregate_network_flows(
    packets: &[NetworkFlowPacket],
    idle_timeout_micros: u64,
) -> Vec<NetworkFlowSession> {
    session::aggregate_network_flows(packets, idle_timeout_micros)
}
