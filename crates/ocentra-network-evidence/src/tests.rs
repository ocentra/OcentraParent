use crate::{
    dns_query_frame_fixture, dns_query_pcap_fixture, dns_query_replay_expected,
    dns_response_payload_fixture, icmp_echo_frame_fixture, parse_dns_message, parse_network_packet,
    parse_pcap_packets, replay_dns_observations, tcp_syn_frame_fixture, DnsQueryType,
    DnsRecordData, IpProtocol, NetworkEvidenceGrade, NetworkReplayError, PacketParseError,
    PcapReplayError, TransportPacketMetadata,
};

mod ai_audit;
mod ai_detection;
mod android_vpn_service_gate;
mod apple_network_extension_gate;
mod bundle;
mod cascade;
mod category;
mod classifier;
mod dns_adapter;
mod domain;
mod flow;
mod linux_adapter_gate;
mod live_capture;
mod local_ai_queue;
mod managed_browser;
mod notification;
mod performance;
mod policy;
mod process;
mod readiness;
mod risk_budget;
mod signature_alert;
mod transfer;
mod tunnel;
mod visibility;
mod windows_firewall_adapter;
mod windows_wfp_gate;
mod zeek;

#[test]
fn deterministic_pcap_replay_extracts_metadata_only_dns_query() {
    let fixture = dns_query_pcap_fixture();
    let summary = replay_dns_observations(&fixture).expect("fixture should parse");
    let expected = dns_query_replay_expected();

    assert_eq!(summary.packet_count, 1);
    assert_eq!(summary.dns_observations, vec![expected]);
    assert_eq!(
        summary.dns_observations[0].evidence_grade,
        NetworkEvidenceGrade::B
    );
    assert!(!summary.dns_observations[0].exact_url_available);
    assert!(!summary.dns_observations[0].decrypted_payload_available);
}

#[test]
fn packet_parser_extracts_ethernet_ipv4_udp_metadata() {
    let parsed = parse_network_packet(&dns_query_frame_fixture()).expect("udp frame should parse");
    let ipv4 = parsed.ipv4.expect("fixture is IPv4");

    assert_eq!(parsed.ethernet.destination_mac, "aa:bb:cc:dd:ee:ff");
    assert_eq!(parsed.ethernet.source_mac, "10:20:30:40:50:60");
    assert_eq!(parsed.ethernet.ether_type, 0x0800);
    assert_eq!(ipv4.source_ip, "192.168.1.25");
    assert_eq!(ipv4.destination_ip, "1.1.1.1");
    assert_eq!(ipv4.protocol, IpProtocol::Udp);
    assert_eq!(ipv4.header_len, 20);
    assert!(matches!(
        parsed.transport,
        Some(TransportPacketMetadata::Udp {
            source_port: 53_000,
            destination_port: 53,
            payload_len: 36
        })
    ));
}

#[test]
fn packet_parser_extracts_tcp_metadata() {
    let parsed = parse_network_packet(&tcp_syn_frame_fixture()).expect("tcp frame should parse");

    assert!(matches!(
        parsed.transport,
        Some(TransportPacketMetadata::Tcp {
            source_port: 53_001,
            destination_port: 443,
            header_len: 20,
            payload_len: 0
        })
    ));
}

#[test]
fn packet_parser_extracts_icmp_metadata() {
    let parsed = parse_network_packet(&icmp_echo_frame_fixture()).expect("icmp frame should parse");

    assert!(matches!(
        parsed.transport,
        Some(TransportPacketMetadata::Icmp {
            icmp_type: 8,
            code: 0,
            payload_len: 4
        })
    ));
}

#[test]
fn packet_parser_rejects_truncated_ethernet_frame() {
    let result = parse_network_packet(&[0; 13]);

    assert_eq!(result, Err(PacketParseError::EthernetFrameTooShort));
}

#[test]
fn dns_parser_extracts_query_and_compressed_response_answer() {
    let message =
        parse_dns_message(&dns_response_payload_fixture()).expect("dns response should parse");

    assert_eq!(message.transaction_id, 0x1234);
    assert!(message.is_response);
    assert_eq!(message.questions.len(), 1);
    assert_eq!(message.questions[0].query_name, "video.example.test");
    assert_eq!(message.questions[0].query_type, DnsQueryType::A);
    assert_eq!(message.answers.len(), 1);
    assert_eq!(message.answers[0].record_name, "video.example.test");
    assert_eq!(message.answers[0].ttl_seconds, 300);
    assert_eq!(
        message.answers[0].data,
        DnsRecordData::Ipv4Address("203.0.113.7".to_owned())
    );
}

#[test]
fn dns_parser_rejects_truncated_answer_data() {
    let mut response = dns_response_payload_fixture();
    response.truncate(response.len() - 2);

    let result = parse_dns_message(&response);

    assert_eq!(result, Err(NetworkReplayError::DnsResourceRecordTruncated));
}

#[test]
fn pcap_parser_rejects_truncated_global_header() {
    let result = parse_pcap_packets(&[0xd4, 0xc3, 0xb2]);

    assert_eq!(result, Err(PcapReplayError::TruncatedGlobalHeader));
}

#[test]
fn pcap_parser_rejects_unsupported_link_type() {
    let mut fixture = dns_query_pcap_fixture();
    fixture[20..24].copy_from_slice(&101_u32.to_le_bytes());

    let result = parse_pcap_packets(&fixture);

    assert_eq!(result, Err(PcapReplayError::UnsupportedLinkType(101)));
}

#[test]
fn pcap_parser_rejects_truncated_packet_data() {
    let mut fixture = dns_query_pcap_fixture();
    fixture.truncate(fixture.len() - 4);

    let result = parse_pcap_packets(&fixture);

    assert!(matches!(
        result,
        Err(PcapReplayError::TruncatedPacketData {
            expected: _,
            actual: _,
            offset: _
        })
    ));
}

#[test]
fn dns_replay_rejects_compressed_question_name_in_fixture_query() {
    let mut fixture = dns_query_pcap_fixture();
    let dns_question_start = 24 + 16 + 14 + 20 + 8 + 12;
    fixture[dns_question_start] = 0xc0;

    let result = replay_dns_observations(&fixture);

    assert_eq!(result, Err(NetworkReplayError::DnsCompressedQuestionName));
}
