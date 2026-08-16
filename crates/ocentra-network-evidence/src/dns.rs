pub mod message;
pub mod types;

use message::parse_dns_message;
use types::{
    DnsMessage, DnsObservation, DnsQueryType, DnsQuestion, DnsRecordData, DnsResourceRecord,
    NetworkEvidenceGrade, NetworkReplayError, NetworkReplaySummary,
};

use crate::packet::udp_payload_from_ethernet_ipv4;
use crate::pcap::{parse_pcap_packets, PcapPacket};

const DNS_PORT: u16 = 53;

pub fn replay_dns_observations(bytes: &[u8]) -> Result<NetworkReplaySummary, NetworkReplayError> {
    let packets = parse_pcap_packets(bytes).map_err(NetworkReplayError::Pcap)?;
    let mut dns_observations = Vec::new();
    for packet in &packets {
        if let Some(observation) = dns_observation_from_packet(packet)? {
            dns_observations.push(observation);
        }
    }

    Ok(NetworkReplaySummary {
        packet_count: packets.len(),
        dns_observations,
    })
}

fn dns_observation_from_packet(
    packet: &PcapPacket,
) -> Result<Option<DnsObservation>, NetworkReplayError> {
    let Some(udp_packet) =
        udp_payload_from_ethernet_ipv4(&packet.data).map_err(NetworkReplayError::Packet)?
    else {
        return Ok(None);
    };
    if udp_packet.source_port != DNS_PORT && udp_packet.destination_port != DNS_PORT {
        return Ok(None);
    }

    let message = parse_dns_message(udp_packet.payload)?;
    let question = message
        .questions
        .first()
        .ok_or(NetworkReplayError::DnsQuestionMissing)?;
    Ok(Some(DnsObservation {
        transaction_id: message.transaction_id,
        query_name: question.query_name.clone(),
        query_type: question.query_type,
        source_ip: udp_packet.source_ip,
        destination_ip: udp_packet.destination_ip,
        source_port: udp_packet.source_port,
        destination_port: udp_packet.destination_port,
        observed_at_micros: u64::from(packet.timestamp_seconds) * 1_000_000
            + u64::from(packet.timestamp_fraction),
        evidence_grade: NetworkEvidenceGrade::B,
        exact_url_available: false,
        decrypted_payload_available: false,
    }))
}
