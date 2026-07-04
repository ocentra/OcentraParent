use chrono::Utc;

use super::super::name_evidence::{
    llmnr_name_evidence, netbios_name_evidence, LanNeighborNameEvidence,
};
use super::text::compact_summary;

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
    let mut labels = Vec::new();
    let mut cursor = offset;
    let mut next_offset = offset;
    let mut jumped = false;
    let mut jumps = 0_usize;
    loop {
        let label_len = *payload.get(cursor)?;
        if label_len == 0 {
            cursor += 1;
            if !jumped {
                next_offset = cursor;
            }
            break;
        }
        if label_len & 0b1100_0000 == 0b1100_0000 {
            let low = *payload.get(cursor + 1)?;
            let pointer = usize::from((u16::from(label_len & 0x3f) << 8) | u16::from(low));
            if pointer >= payload.len() {
                return None;
            }
            if !jumped {
                next_offset = cursor + 2;
            }
            cursor = pointer;
            jumped = true;
            jumps += 1;
            if jumps > 8 {
                return None;
            }
            continue;
        }
        if label_len > 63 || label_len & 0b1100_0000 != 0 {
            return None;
        }
        cursor += 1;
        let label_end = cursor.checked_add(usize::from(label_len))?;
        let label = payload.get(cursor..label_end)?;
        labels.push(String::from_utf8_lossy(label).to_string());
        cursor = label_end;
        if !jumped {
            next_offset = cursor;
        }
        if labels.len() > 16 {
            return None;
        }
    }
    Some((labels.join("."), next_offset))
}

pub fn decode_netbios_name(encoded_name: &str) -> Option<String> {
    let encoded = encoded_name.split('.').next().unwrap_or(encoded_name);
    if encoded.len() != 32 {
        return None;
    }

    let mut bytes = Vec::with_capacity(16);
    let mut chars = encoded.bytes();
    while let (Some(high), Some(low)) = (chars.next(), chars.next()) {
        if !(b'A'..=b'P').contains(&high) || !(b'A'..=b'P').contains(&low) {
            return None;
        }
        bytes.push(((high - b'A') << 4) | (low - b'A'));
    }
    let name_bytes = bytes.get(..15)?;
    let decoded = String::from_utf8_lossy(name_bytes)
        .trim_end()
        .trim_matches(char::from(0))
        .to_string();
    (!decoded.is_empty()).then_some(decoded)
}
