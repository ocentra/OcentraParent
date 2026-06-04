pub mod dns;
pub mod fixtures;
pub mod pcap;

#[cfg(test)]
mod tests;

pub use dns::{
    replay_dns_observations, DnsObservation, DnsQueryType, NetworkEvidenceGrade,
    NetworkReplayError, NetworkReplaySummary,
};
pub use fixtures::{dns_query_pcap_fixture, dns_query_replay_expected};
pub use pcap::{parse_pcap_packets, PcapPacket, PcapReplayError};
