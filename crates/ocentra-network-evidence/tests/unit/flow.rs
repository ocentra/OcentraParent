use std::net::Ipv4Addr;

use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::dns::types::*;
use ocentra_network_evidence::fixtures::dns_query_pcap_fixture;
use ocentra_network_evidence::flow::*;
use ocentra_network_evidence::pcap::{parse_pcap_packets, PcapReplayError};

#[test]
fn flow_aggregation_merges_reverse_direction_into_single_session() {
    let packets = vec![
        flow_packet(
            Ipv4Addr::new(192, 168, 1, 25),
            53_001,
            Ipv4Addr::new(203, 0, 113, 7),
            443,
            1_000,
            74,
        ),
        flow_packet(
            Ipv4Addr::new(203, 0, 113, 7),
            443,
            Ipv4Addr::new(192, 168, 1, 25),
            53_001,
            1_500,
            98,
        ),
    ];

    let sessions = aggregate_network_flows(&packets, 30_000_000);

    assert_eq!(sessions.len(), 1);
    assert_eq!(sessions[0].packet_count, 2);
    assert_eq!(sessions[0].initiator_to_responder_packets, 1);
    assert_eq!(sessions[0].responder_to_initiator_packets, 1);
    assert_eq!(sessions[0].initiator_to_responder_bytes, 74);
    assert_eq!(sessions[0].responder_to_initiator_bytes, 98);
    assert_eq!(sessions[0].duration_micros, 500);
    assert_eq!(sessions[0].evidence_grade, NetworkEvidenceGrade::C);
    assert!(!sessions[0].exact_url_available);
    assert!(!sessions[0].decrypted_payload_available);
}

#[test]
fn flow_aggregation_splits_same_tuple_after_idle_timeout() {
    let packets = vec![
        flow_packet(
            Ipv4Addr::new(192, 168, 1, 25),
            53_001,
            Ipv4Addr::new(203, 0, 113, 7),
            443,
            1_000,
            74,
        ),
        flow_packet(
            Ipv4Addr::new(203, 0, 113, 7),
            443,
            Ipv4Addr::new(192, 168, 1, 25),
            53_001,
            1_500,
            98,
        ),
        flow_packet(
            Ipv4Addr::new(192, 168, 1, 25),
            53_001,
            Ipv4Addr::new(203, 0, 113, 7),
            443,
            9_000,
            66,
        ),
    ];

    let sessions = aggregate_network_flows(&packets, 5_000);

    assert_eq!(sessions.len(), 2);
    assert_eq!(sessions[0].packet_count, 2);
    assert_eq!(sessions[0].duration_micros, 500);
    assert_eq!(sessions[1].packet_count, 1);
    assert_eq!(sessions[1].first_seen_micros, 9_000);
    assert_eq!(sessions[1].initiator_to_responder_bytes, 66);
}

#[test]
fn flow_aggregation_uses_pcap_parser_packet_metadata() {
    let summary = aggregate_pcap_flows(&dns_query_pcap_fixture(), 30_000_000)
        .expect_value("dns pcap fixture should aggregate");

    assert_eq!(summary.packet_count, 1);
    assert_eq!(summary.flow_count, 1);
    assert_eq!(summary.sessions[0].key.initiator_ip, "192.168.1.25");
    assert_eq!(summary.sessions[0].key.initiator_port, 53_000);
    assert_eq!(summary.sessions[0].key.responder_ip, "1.1.1.1");
    assert_eq!(summary.sessions[0].key.responder_port, 53);
    assert_eq!(summary.sessions[0].key.protocol, NetworkFlowProtocol::Udp);
    assert_eq!(summary.sessions[0].packet_count, 1);
    assert_eq!(summary.sessions[0].initiator_to_responder_bytes, 64);
}

#[test]
fn pcap_replay_rejects_truncated_and_unsupported_capture_headers() {
    assert_eq!(
        parse_pcap_packets(&[0xd4, 0xc3, 0xb2]),
        Err(PcapReplayError::TruncatedGlobalHeader)
    );

    let mut unsupported_magic = vec![0_u8; 24];
    unsupported_magic[..4].copy_from_slice(&[0, 1, 2, 3]);
    assert_eq!(
        parse_pcap_packets(&unsupported_magic),
        Err(PcapReplayError::UnsupportedMagic([0, 1, 2, 3]))
    );
}

fn flow_packet(
    source_ip: Ipv4Addr,
    source_port: u16,
    destination_ip: Ipv4Addr,
    destination_port: u16,
    observed_at_micros: u64,
    observed_bytes: usize,
) -> NetworkFlowPacket {
    NetworkFlowPacket {
        source_ip: source_ip.to_string(),
        destination_ip: destination_ip.to_string(),
        source_port,
        destination_port,
        protocol: NetworkFlowProtocol::Tcp,
        observed_at_micros,
        observed_bytes,
    }
}
