use chrono::Utc;

use super::super::name_evidence::{
    llmnr_name_evidence, netbios_name_evidence, LanNeighborNameEvidence,
};
use super::text::compact_summary;

mod name;
mod netbios;

pub fn passive_llmnr_summary(payload: &[u8]) -> Option<String> {
    let (header, evidence) = passive_llmnr_name_evidence(payload)?;
    Some(compact_summary(format!(
        "LLMNR packet: name={}; normalized={}; source={}; confidence={}; firstSeenAt={}; lastSeenAt={}; interface={}; questions={}; answers={}",
        evidence.value,
        evidence.normalized_value,
        evidence.source_label(),
        evidence.confidence_label(),
        evidence.first_seen_at,
        evidence.last_seen_at,
        evidence.network_interface.as_deref().unwrap_or("n/a"),
        header.question_count,
        header.answer_count
    )))
}

pub fn passive_llmnr_device_id(payload: &[u8]) -> Option<String> {
    let (_, query_name) = passive_llmnr_name_evidence(payload)?;
    Some(query_name.normalized_value)
}

pub fn passive_netbios_summary(payload: &[u8]) -> Option<String> {
    let (header, evidence) = passive_netbios_name_evidence(payload)?;
    Some(compact_summary(format!(
        "NetBIOS name packet: name={}; normalized={}; source={}; confidence={}; firstSeenAt={}; lastSeenAt={}; interface={}; questions={}; answers={}",
        evidence.value,
        evidence.normalized_value,
        evidence.source_label(),
        evidence.confidence_label(),
        evidence.first_seen_at,
        evidence.last_seen_at,
        evidence.network_interface.as_deref().unwrap_or("n/a"),
        header.question_count,
        header.answer_count
    )))
}

pub fn passive_netbios_device_id(payload: &[u8]) -> Option<String> {
    let (_, query_name) = passive_netbios_name_evidence(payload)?;
    Some(query_name.normalized_value)
}

pub fn passive_llmnr_name_evidence(
    payload: &[u8],
) -> Option<(DnsLikeHeader, LanNeighborNameEvidence)> {
    let header = dns_like_counts(payload)?;
    let (query_name, _) = dns_like_name(payload, 12)?;
    let observed_at = Utc::now().to_rfc3339();
    let evidence = llmnr_name_evidence(&query_name, &observed_at, None)?;
    Some((header, evidence))
}

pub fn passive_netbios_name_evidence(
    payload: &[u8],
) -> Option<(DnsLikeHeader, LanNeighborNameEvidence)> {
    let header = dns_like_counts(payload)?;
    let (encoded_name, _) = dns_like_name(payload, 12)?;
    let decoded_name = decode_netbios_name(&encoded_name).unwrap_or(encoded_name);
    let observed_at = Utc::now().to_rfc3339();
    let evidence = netbios_name_evidence(&decoded_name, &observed_at, None)?;
    Some((header, evidence))
}

#[derive(Clone, Copy)]
pub struct DnsLikeHeader {
    pub question_count: u16,
    pub answer_count: u16,
}

pub fn dns_like_counts(payload: &[u8]) -> Option<DnsLikeHeader> {
    if payload.len() < 12 {
        return None;
    }
    let question_count = u16::from_be_bytes([payload[4], payload[5]]);
    let answer_count = u16::from_be_bytes([payload[6], payload[7]]);
    if question_count == 0 && answer_count == 0 {
        return None;
    }
    Some(DnsLikeHeader {
        question_count,
        answer_count,
    })
}

pub fn dns_like_name(payload: &[u8], offset: usize) -> Option<(String, usize)> {
    name::dns_like_name(payload, offset)
}

pub fn decode_netbios_name(encoded_name: &str) -> Option<String> {
    netbios::decode_netbios_name(encoded_name)
}
