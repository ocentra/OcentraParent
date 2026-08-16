use std::io;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};

mod packet;

use crate::lan_pairing::LanMdnsAdvertisementPlatformSupport;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanChildMdnsAdvertisement, LanMdnsTxtRecord, LanParentMdnsAdvertisement,
};

const MDNS_MULTICAST_IPV4: Ipv4Addr = Ipv4Addr::new(224, 0, 0, 251);
const MDNS_PORT: u16 = 5353;
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
    packet::hashed_mdns_identifier("parent", &[family_hash])
}

pub fn derive_child_advertisement_id(family_hash: &str, opaque_device_id: &str) -> String {
    packet::hashed_mdns_identifier("child", &[family_hash, opaque_device_id])
}

pub fn parent_instance(advertisement: &LanParentMdnsAdvertisement) -> LanMdnsAdvertisementInstance {
    let label = packet::hashed_mdns_label("parent", &advertisement.advertisement_id);
    LanMdnsAdvertisementInstance {
        service_type: advertisement.service_type.clone(),
        instance_name: format!("{label}.{}", advertisement.service_type),
        txt_records: advertisement.txt_records.clone(),
    }
}

pub fn child_instance(advertisement: &LanChildMdnsAdvertisement) -> LanMdnsAdvertisementInstance {
    let label = packet::hashed_mdns_label("child", &advertisement.advertisement_id);
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
    packet::encode_advertisement_packet(instances, ttl_seconds)
}
