use serde::{Deserialize, Serialize};

use crate::packet::types::PacketParseError;
use crate::pcap::PcapReplayError;

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DnsMessage {
    pub transaction_id: u16,
    pub is_response: bool,
    pub questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsResourceRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DnsQuestion {
    pub query_name: String,
    pub query_type: DnsQueryType,
    pub query_class: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DnsResourceRecord {
    pub record_name: String,
    pub record_type: DnsQueryType,
    pub record_class: u16,
    pub ttl_seconds: u32,
    pub data: DnsRecordData,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DnsRecordData {
    Ipv4Address(String),
    Raw { byte_len: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DnsQueryType {
    A,
    Aaaa,
    Unknown(u16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum NetworkEvidenceGrade {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkReplayError {
    Pcap(PcapReplayError),
    Packet(PacketParseError),
    DnsPacketTooShort,
    DnsQuestionMissing,
    DnsCompressedQuestionName,
    DnsLabelOutOfBounds,
    DnsLabelNotUtf8,
    DnsUnsupportedLabelMode,
    DnsNamePointerLoop,
    DnsQuestionTruncated,
    DnsResourceRecordTruncated,
}
