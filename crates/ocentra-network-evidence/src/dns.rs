use serde::{Deserialize, Serialize};

use crate::pcap::{parse_pcap_packets, PcapPacket, PcapReplayError};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkReplaySummary {
    pub packet_count: usize,
    pub dns_observations: Vec<DnsObservation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DnsObservation {
    pub transaction_id: u16,
    pub query_name: String,
    pub query_type: DnsQueryType,
    pub source_ip: String,
    pub destination_ip: String,
    pub source_port: u16,
    pub destination_port: u16,
    pub observed_at_micros: u64,
    pub evidence_grade: NetworkEvidenceGrade,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DnsQueryType {
    A,
    Aaaa,
    Unknown(u16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkEvidenceGrade {
    B,
    C,
    D,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkReplayError {
    Pcap(PcapReplayError),
    EthernetFrameTooShort,
    Ipv4HeaderTooShort,
    Ipv4PacketTruncated,
    UdpHeaderTooShort,
    UdpPacketTruncated,
    DnsPacketTooShort,
    DnsQuestionMissing,
    DnsCompressedQuestionName,
    DnsLabelOutOfBounds,
    DnsLabelNotUtf8,
    DnsQuestionTruncated,
}

const ETHERNET_HEADER_LEN: usize = 14;
const ETHER_TYPE_IPV4: u16 = 0x0800;
const IPV4_PROTOCOL_UDP: u8 = 17;
const UDP_HEADER_LEN: usize = 8;
const DNS_PORT: u16 = 53;
const DNS_HEADER_LEN: usize = 12;

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
    let Some(udp_packet) = udp_packet_from_frame(&packet.data)? else {
        return Ok(None);
    };
    if udp_packet.source_port != DNS_PORT && udp_packet.destination_port != DNS_PORT {
        return Ok(None);
    }

    let question = parse_dns_question(udp_packet.payload)?;
    Ok(Some(DnsObservation {
        transaction_id: question.transaction_id,
        query_name: question.query_name,
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

struct UdpPacketView<'a> {
    source_ip: String,
    destination_ip: String,
    source_port: u16,
    destination_port: u16,
    payload: &'a [u8],
}

fn udp_packet_from_frame(frame: &[u8]) -> Result<Option<UdpPacketView<'_>>, NetworkReplayError> {
    if frame.len() < ETHERNET_HEADER_LEN {
        return Err(NetworkReplayError::EthernetFrameTooShort);
    }

    let ether_type = u16::from_be_bytes([frame[12], frame[13]]);
    if ether_type != ETHER_TYPE_IPV4 {
        return Ok(None);
    }

    let ip_start = ETHERNET_HEADER_LEN;
    if frame.len() < ip_start + 20 {
        return Err(NetworkReplayError::Ipv4HeaderTooShort);
    }

    let ihl = usize::from(frame[ip_start] & 0x0f) * 4;
    if ihl < 20 {
        return Err(NetworkReplayError::Ipv4HeaderTooShort);
    }

    let total_len = usize::from(u16::from_be_bytes([
        frame[ip_start + 2],
        frame[ip_start + 3],
    ]));
    if frame.len() < ip_start + total_len {
        return Err(NetworkReplayError::Ipv4PacketTruncated);
    }
    if frame[ip_start + 9] != IPV4_PROTOCOL_UDP {
        return Ok(None);
    }

    let source_ip = ipv4_text(&frame[ip_start + 12..ip_start + 16]);
    let destination_ip = ipv4_text(&frame[ip_start + 16..ip_start + 20]);
    let udp_start = ip_start + ihl;
    if frame.len() < udp_start + UDP_HEADER_LEN {
        return Err(NetworkReplayError::UdpHeaderTooShort);
    }

    let source_port = u16::from_be_bytes([frame[udp_start], frame[udp_start + 1]]);
    let destination_port = u16::from_be_bytes([frame[udp_start + 2], frame[udp_start + 3]]);
    let udp_len = usize::from(u16::from_be_bytes([
        frame[udp_start + 4],
        frame[udp_start + 5],
    ]));
    if udp_len < UDP_HEADER_LEN || frame.len() < udp_start + udp_len {
        return Err(NetworkReplayError::UdpPacketTruncated);
    }

    Ok(Some(UdpPacketView {
        source_ip,
        destination_ip,
        source_port,
        destination_port,
        payload: &frame[udp_start + UDP_HEADER_LEN..udp_start + udp_len],
    }))
}

struct DnsQuestion {
    transaction_id: u16,
    query_name: String,
    query_type: DnsQueryType,
}

fn parse_dns_question(payload: &[u8]) -> Result<DnsQuestion, NetworkReplayError> {
    if payload.len() < DNS_HEADER_LEN {
        return Err(NetworkReplayError::DnsPacketTooShort);
    }

    let transaction_id = u16::from_be_bytes([payload[0], payload[1]]);
    let qdcount = u16::from_be_bytes([payload[4], payload[5]]);
    if qdcount == 0 {
        return Err(NetworkReplayError::DnsQuestionMissing);
    }

    let (query_name, offset) = parse_dns_qname(payload, DNS_HEADER_LEN)?;
    if payload.len() < offset + 4 {
        return Err(NetworkReplayError::DnsQuestionTruncated);
    }

    let raw_query_type = u16::from_be_bytes([payload[offset], payload[offset + 1]]);
    Ok(DnsQuestion {
        transaction_id,
        query_name,
        query_type: match raw_query_type {
            1 => DnsQueryType::A,
            28 => DnsQueryType::Aaaa,
            value => DnsQueryType::Unknown(value),
        },
    })
}

fn parse_dns_qname(
    payload: &[u8],
    mut offset: usize,
) -> Result<(String, usize), NetworkReplayError> {
    let mut labels = Vec::new();
    loop {
        let label_len = *payload
            .get(offset)
            .ok_or(NetworkReplayError::DnsLabelOutOfBounds)?;
        offset += 1;
        if label_len == 0 {
            break;
        }
        if label_len & 0b1100_0000 != 0 {
            return Err(NetworkReplayError::DnsCompressedQuestionName);
        }

        let label_len = usize::from(label_len);
        let next_offset = offset + label_len;
        let label = payload
            .get(offset..next_offset)
            .ok_or(NetworkReplayError::DnsLabelOutOfBounds)?;
        labels.push(
            std::str::from_utf8(label)
                .map_err(|_| NetworkReplayError::DnsLabelNotUtf8)?
                .to_ascii_lowercase(),
        );
        offset = next_offset;
    }

    Ok((labels.join("."), offset))
}

fn ipv4_text(bytes: &[u8]) -> String {
    format!("{}.{}.{}.{}", bytes[0], bytes[1], bytes[2], bytes[3])
}
