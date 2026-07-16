use std::net::Ipv4Addr;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanChildMdnsAdvertisement, LanParentMdnsAdvertisement,
};

use super::LanNetworkInventoryDevice;

pub mod accumulator;
pub mod advertisement;
pub mod merge;
pub mod packet;
pub mod query;
pub mod text;

pub const MDNS_IPV4_MULTICAST: Ipv4Addr = Ipv4Addr::new(224, 0, 0, 251);
pub const MDNS_PORT: u16 = 5353;
pub const MDNS_RESPONSE_TIMEOUT_MS: u64 = 150;
pub const MDNS_MAX_POINTER_JUMPS: usize = 8;
pub const MDNS_HEADER_LEN: usize = 12;
pub const MDNS_CLASS_IN: u16 = 1;
pub const MDNS_UNICAST_RESPONSE_BIT: u16 = 0x8000;
pub const MDNS_TYPE_A: u16 = 1;
pub const MDNS_TYPE_PTR: u16 = 12;
pub const MDNS_TYPE_TXT: u16 = 16;
pub const MDNS_TYPE_AAAA: u16 = 28;
pub const MDNS_TYPE_SRV: u16 = 33;
pub const MDNS_SERVICE_ENUMERATION: &str = "_services._dns-sd._udp.local";
pub const MDNS_SERVICE_TYPES: &[&str] = &[
    "_workstation._tcp.local",
    "_ipp._tcp.local",
    "_printer._tcp.local",
    "_airplay._tcp.local",
    "_raop._tcp.local",
    "_googlecast._tcp.local",
    "_companion-link._tcp.local",
    constants::lan_pairing::MDNS_PARENT_SERVICE_TYPE,
    constants::lan_pairing::MDNS_CHILD_SERVICE_TYPE,
];
pub const MDNS_MAX_TEXT_BYTES: usize = 128;
pub const MDNS_MAX_LABELS: usize = 32;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MdnsDnsSdDiscovery {
    pub observed_at: String,
    pub service_types: Vec<String>,
    pub service_instances: Vec<MdnsDnsSdServiceInstance>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MdnsDnsSdServiceInstance {
    pub service_type: String,
    pub instance_name: String,
    pub display_name: Option<String>,
    pub target_hostname: Option<String>,
    pub port: Option<u16>,
    pub addresses: Vec<String>,
    pub txt_records: Vec<MdnsDnsSdTxtRecord>,
    pub parent_advertisement: Option<LanParentMdnsAdvertisement>,
    pub child_advertisement: Option<LanChildMdnsAdvertisement>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MdnsDnsSdTxtRecord {
    pub key: String,
    pub value: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MdnsDnsSdSrvRecord {
    pub target_hostname: Option<String>,
    pub port: Option<u16>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MdnsDnsSdPacket {
    pub records: Vec<MdnsDnsSdRecord>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MdnsDnsSdRecord {
    pub name: String,
    pub data: MdnsRecordData,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MdnsRecordData {
    Ptr(String),
    Srv {
        target_hostname: Option<String>,
        port: Option<u16>,
    },
    Txt(Vec<MdnsDnsSdTxtRecord>),
    A(String),
    Aaaa(String),
    Unknown,
}

pub fn enrich_mdns_dns_sd_devices(
    devices: &mut Vec<LanNetworkInventoryDevice>,
    selected_interface: Option<&str>,
) {
    let Some(discovery) = query::query_mdns_dns_sd() else {
        return;
    };
    merge::merge_mdns_dns_sd_discovery_with_selected_interface(
        devices,
        &discovery,
        selected_interface,
    );
}

pub fn parse_dns_name(payload: &[u8], offset: usize) -> Option<(String, usize)> {
    packet::parse_dns_name(payload, offset)
}

pub fn passive_mdns_dns_sd_summary(payload: &[u8]) -> Option<String> {
    accumulator::passive_mdns_dns_sd_summary(payload)
}

pub fn passive_mdns_dns_sd_device_id(payload: &[u8]) -> Option<String> {
    accumulator::passive_mdns_dns_sd_device_id(payload)
}
