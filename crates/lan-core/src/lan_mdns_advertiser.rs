use std::collections::BTreeSet;
use std::io;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};

use sha2::{Digest, Sha256};

use crate::lan_pairing::LanMdnsAdvertisementPlatformSupport;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanChildMdnsAdvertisement, LanMdnsTxtRecord, LanParentMdnsAdvertisement,
};

const MDNS_MULTICAST_IPV4: Ipv4Addr = Ipv4Addr::new(224, 0, 0, 251);
const MDNS_PORT: u16 = 5353;
const MDNS_RESPONSE_FLAGS: u16 = 0x8400;
const MDNS_CLASS_IN: u16 = 0x0001;
const MDNS_SERVICE_ENUMERATION: &str = "_services._dns-sd._udp.local";
const MDNS_RECORD_TYPE_PTR: u16 = 12;
const MDNS_RECORD_TYPE_TXT: u16 = 16;
const MDNS_DEFAULT_TTL_SECONDS: u32 = 120;
const MDNS_GOODBYE_TTL_SECONDS: u32 = 0;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LanMdnsAdvertisementInstance {
    pub service_type: String,
    pub instance_name: String,
    pub txt_records: Vec<LanMdnsTxtRecord>,
}

pub trait LanMdnsPacketSink {
    fn send(&self, packet: &[u8]) -> io::Result<()>;
}

#[derive(Default)]
pub struct UdpMulticastMdnsPacketSink;

impl LanMdnsPacketSink for UdpMulticastMdnsPacketSink {
    fn send(&self, packet: &[u8]) -> io::Result<()> {
        let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0))?;
        socket.send_to(packet, SocketAddrV4::new(MDNS_MULTICAST_IPV4, MDNS_PORT))?;
        Ok(())
    }
}

pub fn current_platform_support() -> LanMdnsAdvertisementPlatformSupport {
    match std::env::consts::OS {
        "windows" | "linux" | "macos" => LanMdnsAdvertisementPlatformSupport::Supported,
        "android" | "ios" => LanMdnsAdvertisementPlatformSupport::Degraded,
        _ => LanMdnsAdvertisementPlatformSupport::UnsupportedPlatform,
    }
}

pub fn derive_parent_advertisement_id(family_hash: &str) -> String {
    hashed_mdns_identifier("parent", &[family_hash])
}

pub fn derive_child_advertisement_id(family_hash: &str, opaque_device_id: &str) -> String {
    hashed_mdns_identifier("child", &[family_hash, opaque_device_id])
}

pub fn parent_instance(advertisement: &LanParentMdnsAdvertisement) -> LanMdnsAdvertisementInstance {
    let label = hashed_mdns_label("parent", &advertisement.advertisement_id);
    LanMdnsAdvertisementInstance {
        service_type: advertisement.service_type.clone(),
        instance_name: format!("{label}.{}", advertisement.service_type),
        txt_records: advertisement.txt_records.clone(),
    }
}

pub fn child_instance(advertisement: &LanChildMdnsAdvertisement) -> LanMdnsAdvertisementInstance {
    let label = hashed_mdns_label("child", &advertisement.advertisement_id);
    LanMdnsAdvertisementInstance {
        service_type: advertisement.service_type.clone(),
        instance_name: format!("{label}.{}", advertisement.service_type),
        txt_records: advertisement.txt_records.clone(),
    }
}

pub fn send_advertisements(
    instances: &[LanMdnsAdvertisementInstance],
    sink: &dyn LanMdnsPacketSink,
) -> io::Result<()> {
    if instances.is_empty() {
        return Ok(());
    }
    sink.send(&encode_advertisement_packet(
        instances,
        MDNS_DEFAULT_TTL_SECONDS,
    ))
}

pub fn send_goodbye(
    instances: &[LanMdnsAdvertisementInstance],
    sink: &dyn LanMdnsPacketSink,
) -> io::Result<()> {
    if instances.is_empty() {
        return Ok(());
    }
    sink.send(&encode_advertisement_packet(
        instances,
        MDNS_GOODBYE_TTL_SECONDS,
    ))
}

pub fn encode_advertisement_packet(
    instances: &[LanMdnsAdvertisementInstance],
    ttl_seconds: u32,
) -> Vec<u8> {
    let mut records = Vec::new();
    let mut announced_service_types = BTreeSet::new();

    for instance in instances {
        if announced_service_types.insert(instance.service_type.clone()) {
            records.push(EncodedRecord {
                name: MDNS_SERVICE_ENUMERATION.to_string(),
                record_type: MDNS_RECORD_TYPE_PTR,
                ttl_seconds,
                data: encode_name(instance.service_type.as_str()),
            });
        }
        records.push(EncodedRecord {
            name: instance.service_type.clone(),
            record_type: MDNS_RECORD_TYPE_PTR,
            ttl_seconds,
            data: encode_name(instance.instance_name.as_str()),
        });
        records.push(EncodedRecord {
            name: instance.instance_name.clone(),
            record_type: MDNS_RECORD_TYPE_TXT,
            ttl_seconds,
            data: encode_txt_data(instance.txt_records.as_slice()),
        });
    }

    let mut packet = Vec::new();
    packet.extend_from_slice(&0_u16.to_be_bytes());
    packet.extend_from_slice(&MDNS_RESPONSE_FLAGS.to_be_bytes());
    packet.extend_from_slice(&0_u16.to_be_bytes());
    packet.extend_from_slice(&(records.len() as u16).to_be_bytes());
    packet.extend_from_slice(&0_u16.to_be_bytes());
    packet.extend_from_slice(&0_u16.to_be_bytes());

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

fn hashed_mdns_identifier(prefix: &str, parts: &[&str]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(prefix.as_bytes());
    for part in parts {
        hasher.update([0]);
        hasher.update(part.as_bytes());
    }
    let digest = hex_string(hasher.finalize().as_slice());
    format!("sha256:{digest}")
}

fn hashed_mdns_label(prefix: &str, seed: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(prefix.as_bytes());
    hasher.update([0]);
    hasher.update(seed.as_bytes());
    let digest = hex_string(hasher.finalize().as_slice());
    format!("{prefix}-{}", &digest[..12])
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
    for label in name.split('.') {
        if label.is_empty() {
            continue;
        }
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
        if entry_bytes.len() > u8::MAX as usize {
            continue;
        }
        data.push(entry_bytes.len() as u8);
        data.extend_from_slice(entry_bytes);
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
