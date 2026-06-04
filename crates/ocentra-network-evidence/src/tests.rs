use crate::{
    dns_query_pcap_fixture, dns_query_replay_expected, parse_pcap_packets, replay_dns_observations,
    NetworkEvidenceGrade, NetworkReplayError, PcapReplayError,
};

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
