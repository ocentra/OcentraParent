pub mod dns;
pub mod fixtures;
pub mod packet;
pub mod pcap;

#[cfg(test)]
mod tests;

pub use dns::{
    parse_dns_message, replay_dns_observations, DnsMessage, DnsObservation, DnsQueryType,
    DnsQuestion, DnsRecordData, DnsResourceRecord, NetworkEvidenceGrade, NetworkReplayError,
    NetworkReplaySummary,
};
pub use fixtures::{
    dns_query_frame_fixture, dns_query_pcap_fixture, dns_query_replay_expected,
    dns_response_payload_fixture, icmp_echo_frame_fixture, tcp_syn_frame_fixture,
};
pub use packet::{
    parse_network_packet, udp_payload_from_ethernet_ipv4, EthernetFrameMetadata, IpProtocol,
    Ipv4PacketMetadata, PacketParseError, ParsedNetworkPacket, TransportPacketMetadata,
    UdpPayloadView,
};
pub use pcap::{parse_pcap_packets, PcapPacket, PcapReplayError};
