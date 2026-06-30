use std::collections::HashSet;

use super::text::sanitize_mdns_text;
use super::super::neighbor_support::normalize_neighbor_hostname;
use super::{
    MdnsDnsSdPacket, MdnsDnsSdRecord, MdnsDnsSdTxtRecord, MdnsRecordData, MDNS_CLASS_IN,
    MDNS_HEADER_LEN, MDNS_MAX_LABELS, MDNS_MAX_POINTER_JUMPS, MDNS_SERVICE_ENUMERATION,
    MDNS_SERVICE_TYPES, MDNS_TYPE_A, MDNS_TYPE_AAAA, MDNS_TYPE_PTR, MDNS_TYPE_SRV, MDNS_TYPE_TXT,
    MDNS_UNICAST_RESPONSE_BIT,
};

pub fn parse_mdns_packet(payload: &[u8]) -> Option<MdnsDnsSdPacket> {
    if payload.len() < MDNS_HEADER_LEN {
        return None;
    }
    let question_count = usize::from(u16::from_be_bytes([payload[4], payload[5]]));
    let answer_count = usize::from(u16::from_be_bytes([payload[6], payload[7]]));
    let authority_count = usize::from(u16::from_be_bytes([payload[8], payload[9]]));
    let additional_count = usize::from(u16::from_be_bytes([payload[10], payload[11]]));

    let mut offset = skip_mdns_questions(payload, question_count);
    let mut records = Vec::new();
    for record_count in [answer_count, authority_count, additional_count] {
        for _ in 0..record_count {
            let (record, next_offset) = match parse_resource_record(payload, offset) {
                Some(value) => value,
                None => {
                    offset = offset.saturating_add(1);
                    if offset >= payload.len() {
                        break;
                    }
                    continue;
                }
            };
            if next_offset <= offset || next_offset > payload.len() {
                break;
            }
            records.push(record);
            offset = next_offset;
        }
    }

    Some(MdnsDnsSdPacket { records })
}

pub fn skip_mdns_questions(payload: &[u8], question_count: usize) -> usize {
    let mut offset = MDNS_HEADER_LEN;
    for _ in 0..question_count {
        let next_offset = match parse_dns_name(payload, offset) {
            Some((_, next_offset)) => next_offset,
            None => offset.saturating_add(1),
        };
        offset = match next_offset.checked_add(4) {
            Some(value) if value <= payload.len() => value,
            _ => break,
        };
    }
    offset
}

pub fn parse_resource_record(payload: &[u8], offset: usize) -> Option<(MdnsDnsSdRecord, usize)> {
    let (name, metadata_offset) = parse_dns_name(payload, offset)?;
    let end_of_metadata = metadata_offset.checked_add(10)?;
    if end_of_metadata > payload.len() {
        return None;
    }

    let record_type = u16::from_be_bytes([payload[metadata_offset], payload[metadata_offset + 1]]);
    let data_len = usize::from(u16::from_be_bytes([
        payload[metadata_offset + 8],
        payload[metadata_offset + 9],
    ]));
    let data_offset = end_of_metadata;
    let data_end = data_offset.checked_add(data_len)?;
    let data = payload.get(data_offset..data_end)?;
    let record_data = resource_record_data(payload, record_type, data_offset, data);

    Some((
        MdnsDnsSdRecord {
            name,
            data: record_data,
        },
        data_end,
    ))
}

pub fn resource_record_data(
    payload: &[u8],
    record_type: u16,
    data_offset: usize,
    data: &[u8],
) -> MdnsRecordData {
    match record_type {
        MDNS_TYPE_PTR => parse_dns_name(payload, data_offset)
            .map(|(target, _)| MdnsRecordData::Ptr(target))
            .unwrap_or(MdnsRecordData::Unknown),
        MDNS_TYPE_SRV => parse_srv_record(payload, data_offset),
        MDNS_TYPE_TXT => MdnsRecordData::Txt(parse_txt_records(data)),
        MDNS_TYPE_A => ipv4_record_data(data),
        MDNS_TYPE_AAAA => ipv6_record_data(data),
        _ => MdnsRecordData::Unknown,
    }
}

pub fn ipv4_record_data(data: &[u8]) -> MdnsRecordData {
    if data.len() == 4 {
        MdnsRecordData::A(format!("{}.{}.{}.{}", data[0], data[1], data[2], data[3]))
    } else {
        MdnsRecordData::Unknown
    }
}

pub fn ipv6_record_data(data: &[u8]) -> MdnsRecordData {
    if data.len() != 16 {
        return MdnsRecordData::Unknown;
    }
    let address = std::net::Ipv6Addr::from([
        data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8], data[9],
        data[10], data[11], data[12], data[13], data[14], data[15],
    ]);
    MdnsRecordData::Aaaa(address.to_string())
}

pub fn parse_srv_record(payload: &[u8], offset: usize) -> MdnsRecordData {
    let data = match payload.get(offset..) {
        Some(data) => data,
        None => return MdnsRecordData::Unknown,
    };
    if data.len() < 6 {
        return MdnsRecordData::Unknown;
    }
    let port = u16::from_be_bytes([data[4], data[5]]);
    let target_hostname = parse_dns_name(payload, offset + 6)
        .and_then(|(hostname, _)| normalize_neighbor_hostname(&hostname))
        .filter(|hostname| !hostname.is_empty());
    MdnsRecordData::Srv {
        target_hostname,
        port: Some(port),
    }
}

pub fn parse_txt_records(data: &[u8]) -> Vec<MdnsDnsSdTxtRecord> {
    let mut offset = 0;
    let mut records = Vec::new();
    while offset < data.len() {
        let len = match data.get(offset) {
            Some(value) => usize::from(*value),
            None => break,
        };
        offset += 1;
        let Some(end) = offset.checked_add(len) else {
            break;
        };
        if offset > data.len() || end > data.len() {
            break;
        }
        push_txt_record_entry(&mut records, &data[offset..end]);
        offset = end;
    }
    records
}

pub fn push_txt_record_entry(records: &mut Vec<MdnsDnsSdTxtRecord>, entry: &[u8]) {
    let entry = std::string::String::from_utf8_lossy(entry).to_string();
    if entry.is_empty() {
        return;
    }
    let (key, value) = if let Some((key, value)) = entry.split_once('=') {
        let key = sanitize_mdns_text(key);
        let value = sanitize_mdns_text(value).filter(|value| !value.is_empty());
        (key, value)
    } else {
        (sanitize_mdns_text(&entry), None)
    };
    if let Some(key) = key {
        records.push(MdnsDnsSdTxtRecord { key, value });
    }
}

pub fn parse_dns_name(payload: &[u8], offset: usize) -> Option<(String, usize)> {
    let mut labels = Vec::new();
    let mut cursor = offset;
    let mut next_offset = offset;
    let mut jumped = false;
    let mut jumps = 0_usize;
    let mut visited = HashSet::new();
    visited.insert(offset);

    loop {
        if labels.len() >= MDNS_MAX_LABELS {
            return None;
        }
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
            if pointer >= payload.len() || !visited.insert(pointer) {
                return None;
            }
            if !jumped {
                next_offset = cursor + 2;
            }
            cursor = pointer;
            jumped = true;
            jumps += 1;
            if jumps > MDNS_MAX_POINTER_JUMPS {
                return None;
            }
            continue;
        }
        if label_len == 0 || label_len > 63 || label_len & 0b1100_0000 != 0 {
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
    }

    Some((labels.join("."), next_offset))
}

pub fn encode_mdns_query(query_name: &str) -> Vec<u8> {
    let mut packet = Vec::new();
    packet.extend_from_slice(&0_u16.to_be_bytes());
    packet.extend_from_slice(&0_u16.to_be_bytes());
    packet.extend_from_slice(&1_u16.to_be_bytes());
    packet.extend_from_slice(&0_u16.to_be_bytes());
    packet.extend_from_slice(&0_u16.to_be_bytes());
    packet.extend_from_slice(&0_u16.to_be_bytes());
    for label in query_name.split('.') {
        packet.push(label.len() as u8);
        packet.extend_from_slice(label.as_bytes());
    }
    packet.push(0);
    packet.extend_from_slice(&MDNS_TYPE_PTR.to_be_bytes());
    packet.extend_from_slice(&(MDNS_CLASS_IN | MDNS_UNICAST_RESPONSE_BIT).to_be_bytes());
    packet
}

pub fn mdns_query_names() -> Vec<&'static str> {
    let mut names = vec![MDNS_SERVICE_ENUMERATION];
    names.extend_from_slice(MDNS_SERVICE_TYPES);
    names
}
