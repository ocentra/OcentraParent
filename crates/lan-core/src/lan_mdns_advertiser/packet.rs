use std::collections::BTreeSet;

use sha2::{Digest, Sha256};

use ocentra_parent_agent_protocol::lan_pairing::LanMdnsTxtRecord;

use super::LanMdnsAdvertisementInstance;

const MDNS_RESPONSE_FLAGS: u16 = 0x8400;
const MDNS_CLASS_IN: u16 = 0x0001;
const MDNS_SERVICE_ENUMERATION: &str = "_services._dns-sd._udp.local";
const MDNS_RECORD_TYPE_PTR: u16 = 12;
const MDNS_RECORD_TYPE_TXT: u16 = 16;

pub(super) fn encode_advertisement_packet(
    instances: &[LanMdnsAdvertisementInstance],
    ttl_seconds: u32,
) -> Vec<u8> {
    let records = encoded_records(instances, ttl_seconds);
    let mut packet = response_header(records.len());
    for record in records {
        packet.extend_from_slice(&encode_name(record.name.as_str()));
        packet.extend_from_slice(&record.record_type.to_be_bytes());
        packet.extend_from_slice(&MDNS_CLASS_IN.to_be_bytes());
        packet.extend_from_slice(&record.ttl_seconds.to_be_bytes());
        packet.extend_from_slice(&(record.data.len() as u16).to_be_bytes());
        packet.extend_from_slice(record.data.as_slice());
    }
    packet
}

pub(super) fn hashed_mdns_identifier(prefix: &str, parts: &[&str]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(prefix.as_bytes());
    for part in parts {
        hasher.update([0]);
        hasher.update(part.as_bytes());
    }
    format!("sha256:{}", hex_string(hasher.finalize().as_slice()))
}

pub(super) fn hashed_mdns_label(prefix: &str, seed: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(prefix.as_bytes());
    hasher.update([0]);
    hasher.update(seed.as_bytes());
    let digest = hex_string(hasher.finalize().as_slice());
    format!("{prefix}-{}", &digest[..12])
}

fn encoded_records(
    instances: &[LanMdnsAdvertisementInstance],
    ttl_seconds: u32,
) -> Vec<EncodedRecord> {
    let mut records = Vec::new();
    let mut announced_service_types = BTreeSet::new();
    for instance in instances {
        if announced_service_types.insert(instance.service_type.clone()) {
            records.push(EncodedRecord::service_enumeration(instance, ttl_seconds));
        }
        records.push(EncodedRecord::service_instance(instance, ttl_seconds));
        records.push(EncodedRecord::txt(instance, ttl_seconds));
    }
    records
}

fn response_header(record_count: usize) -> Vec<u8> {
    let mut packet = Vec::new();
    packet.extend_from_slice(&0_u16.to_be_bytes());
    packet.extend_from_slice(&MDNS_RESPONSE_FLAGS.to_be_bytes());
    packet.extend_from_slice(&0_u16.to_be_bytes());
    packet.extend_from_slice(&(record_count as u16).to_be_bytes());
    packet.extend_from_slice(&0_u16.to_be_bytes());
    packet.extend_from_slice(&0_u16.to_be_bytes());
    packet
}

fn hex_string(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0F) as usize] as char);
    }
    output
}

fn encode_name(name: &str) -> Vec<u8> {
    let mut encoded = Vec::new();
    for label in name.split('.').filter(|label| !label.is_empty()) {
        encoded.push(label.len() as u8);
        encoded.extend_from_slice(label.as_bytes());
    }
    encoded.push(0);
    encoded
}

fn encode_txt_data(records: &[LanMdnsTxtRecord]) -> Vec<u8> {
    let mut data = Vec::new();
    for record in records {
        let entry = format!("{}={}", record.key, record.value);
        let entry_bytes = entry.as_bytes();
        if entry_bytes.len() <= u8::MAX as usize {
            data.push(entry_bytes.len() as u8);
            data.extend_from_slice(entry_bytes);
        }
    }
    data
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct EncodedRecord {
    name: String,
    record_type: u16,
    ttl_seconds: u32,
    data: Vec<u8>,
}

impl EncodedRecord {
    fn service_enumeration(instance: &LanMdnsAdvertisementInstance, ttl_seconds: u32) -> Self {
        Self {
            name: MDNS_SERVICE_ENUMERATION.to_string(),
            record_type: MDNS_RECORD_TYPE_PTR,
            ttl_seconds,
            data: encode_name(instance.service_type.as_str()),
        }
    }

    fn service_instance(instance: &LanMdnsAdvertisementInstance, ttl_seconds: u32) -> Self {
        Self {
            name: instance.service_type.clone(),
            record_type: MDNS_RECORD_TYPE_PTR,
            ttl_seconds,
            data: encode_name(instance.instance_name.as_str()),
        }
    }

    fn txt(instance: &LanMdnsAdvertisementInstance, ttl_seconds: u32) -> Self {
        Self {
            name: instance.instance_name.clone(),
            record_type: MDNS_RECORD_TYPE_TXT,
            ttl_seconds,
            data: encode_txt_data(instance.txt_records.as_slice()),
        }
    }
}
