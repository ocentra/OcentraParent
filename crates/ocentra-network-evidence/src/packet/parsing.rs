#[path = "ethernet.rs"]
mod ethernet;
#[path = "ipv4.rs"]
mod ipv4;
#[path = "transport.rs"]
mod transport;

use super::types::{
    EthernetFrameMetadata, IpProtocol, Ipv4PacketMetadata, PacketParseError,
    TransportPacketMetadata,
};

const ETHERNET_HEADER_LEN: usize = 14;
const IPV4_PROTOCOL_ICMP: u8 = 1;
const IPV4_PROTOCOL_TCP: u8 = 6;
const IPV4_PROTOCOL_UDP: u8 = 17;
const ICMP_HEADER_LEN: usize = 4;
const TCP_HEADER_LEN: usize = 20;
const UDP_HEADER_LEN: usize = 8;

pub(super) struct Ipv4PacketView<'a> {
    pub(super) metadata: Ipv4PacketMetadata,
    pub(super) protocol: u8,
    pub(super) payload: &'a [u8],
}

pub(super) struct UdpPacketView<'a> {
    pub(super) source_port: u16,
    pub(super) destination_port: u16,
    pub(super) payload: &'a [u8],
}

pub(super) fn parse_ethernet_frame(
    frame: &[u8],
) -> Result<EthernetFrameMetadata, PacketParseError> {
    ethernet::parse_ethernet_frame(frame)
}

pub(super) fn parse_ipv4_packet(frame: &[u8]) -> Result<Ipv4PacketView<'_>, PacketParseError> {
    ipv4::parse_ipv4_packet(frame)
}

pub(super) fn parse_transport(
    view: &Ipv4PacketView<'_>,
) -> Result<TransportPacketMetadata, PacketParseError> {
    transport::parse_transport(view)
}

pub(super) fn parse_udp_payload<'a>(
    view: &Ipv4PacketView<'a>,
) -> Result<UdpPacketView<'a>, PacketParseError> {
    transport::parse_udp_payload(view)
}

pub(super) fn ether_type(frame: &[u8]) -> u16 {
    ethernet::ether_type(frame)
}
