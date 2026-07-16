use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::dns::replay_dns_observations;
use ocentra_network_evidence::fixtures::dns_query_pcap_fixture;
use ocentra_network_evidence::fixtures::visibility::*;
use ocentra_network_evidence::flow::*;
use ocentra_network_evidence::http::*;
use ocentra_network_evidence::tls::*;
use ocentra_network_evidence::zeek::*;

#[test]
fn zeek_generator_produces_conn_dns_http_tls_and_ssl_rows_with_matching_comparisons() {
    let proof = generate_network_zeek_analyzer_proof(complete_input())
        .expect_value("complete Zeek-style fixture comparison should pass");

    assert_eq!(proof.analyzer_run_ref, "zeek-analyzer-run-43");
    assert_eq!(proof.source_fixture_ref, "pcap-fixture-network-43");
    assert_eq!(proof.connection_rows.len(), 1);
    assert_eq!(proof.dns_rows.len(), 1);
    assert_eq!(proof.http_rows.len(), 1);
    assert_eq!(proof.tls_rows.len(), 1);
    assert_eq!(proof.ssl_rows.len(), 1);
    assert_eq!(
        proof.comparison_state,
        NetworkZeekAnalyzerComparisonState::Matched
    );
    assert_eq!(proof.missing_comparison_log_kinds, Vec::new());

    let conn = &proof.connection_rows[0];
    assert_eq!(conn.source_ip, "192.168.1.25");
    assert_eq!(conn.source_port, 53_000);
    assert_eq!(conn.destination_ip, "1.1.1.1");
    assert_eq!(conn.destination_port, 53);
    assert_eq!(conn.origin_packets, 1);
    assert_eq!(conn.response_packets, 0);

    let dns = &proof.dns_rows[0];
    assert_eq!(dns.query_name, "video.example.test");
    assert_eq!(dns.query_type, "A");

    let http = &proof.http_rows[0];
    assert_eq!(http.evidence_ref, "http-evidence-43");
    assert_eq!(http.host, Some("video.example.test".to_owned()));
    assert_eq!(http.visibility_state, NetworkZeekVisibilityState::Visible);
    assert!(!http.exact_url_available);
    assert!(!http.decrypted_payload_available);
    assert!(!http.page_content_available);

    let tls = &proof.tls_rows[0];
    assert_eq!(tls.server_name, Some("video.example.test".to_owned()));
    assert_eq!(tls.visibility_state, NetworkZeekVisibilityState::Visible);
    assert!(!proof.exact_url_available);
    assert!(!proof.decrypted_payload_available);
    assert!(!proof.page_content_available);
    assert!(!proof.signature_alert_ingested);
    assert!(!proof.live_analyzer_invoked);
    assert!(!proof.policy_authority);
    assert!(!proof.adapter_authority);
    assert!(!proof.enforcement_command_published);
}

#[test]
fn zeek_generator_preserves_unknown_missing_and_ambiguous_states_without_guessing() {
    let mut input = complete_input();
    input.http_evidence = vec![
        NetworkZeekHttpEvidence {
            evidence_ref: "http-missing-ref-43".to_owned(),
            flow_ref: "flow-http-missing-43".to_owned(),
            observed_at_micros: 1_765_000_000_130_000,
            host: None,
            visibility_state: NetworkZeekVisibilityState::Missing,
        },
        NetworkZeekHttpEvidence {
            evidence_ref: "http-ambiguous-ref-43".to_owned(),
            flow_ref: "flow-http-ambiguous-43".to_owned(),
            observed_at_micros: 1_765_000_000_131_000,
            host: None,
            visibility_state: NetworkZeekVisibilityState::Ambiguous,
        },
    ];
    input.tls_evidence = vec![NetworkZeekTlsEvidence {
        evidence_ref: "tls-encrypted-ref-43".to_owned(),
        flow_ref: "flow-tls-encrypted-43".to_owned(),
        observed_at_micros: 1_765_000_000_132_000,
        server_name: None,
        visibility_state: NetworkZeekVisibilityState::Encrypted,
    }];
    input.ssl_evidence = vec![NetworkZeekTlsEvidence {
        evidence_ref: "ssl-unknown-ref-43".to_owned(),
        flow_ref: "flow-ssl-unknown-43".to_owned(),
        observed_at_micros: 1_765_000_000_133_000,
        server_name: None,
        visibility_state: NetworkZeekVisibilityState::Unknown,
    }];
    input.comparison_artifacts = comparison_artifacts(1, 1, 2, 1, 1, 4);

    let proof = generate_network_zeek_analyzer_proof(input)
        .expect_value("unknown and ambiguous rows should be matched as explicit states");

    assert_eq!(proof.http_rows[0].host, None);
    assert_eq!(
        proof.http_rows[0].visibility_state,
        NetworkZeekVisibilityState::Missing
    );
    assert_eq!(proof.http_rows[1].host, None);
    assert_eq!(
        proof.http_rows[1].visibility_state,
        NetworkZeekVisibilityState::Ambiguous
    );
    assert_eq!(proof.tls_rows[0].server_name, None);
    assert_eq!(
        proof.tls_rows[0].visibility_state,
        NetworkZeekVisibilityState::Encrypted
    );
    assert_eq!(proof.ssl_rows[0].server_name, None);
    assert_eq!(
        proof.ssl_rows[0].visibility_state,
        NetworkZeekVisibilityState::Unknown
    );
    assert_eq!(
        proof.comparison_artifacts[2].preserved_unknown_or_ambiguous_rows,
        4
    );
    assert!(!proof.exact_url_available);
    assert!(!proof.decrypted_payload_available);
}

#[test]
fn zeek_generator_rejects_missing_or_mismatched_comparison_artifacts() {
    let mut missing = complete_input();
    missing
        .comparison_artifacts
        .retain(|artifact| artifact.log_kind != NetworkZeekLogKind::Http);
    assert_eq!(
        generate_network_zeek_analyzer_proof(missing),
        Err(NetworkZeekAnalyzerError::MissingApprovedComparison(
            NetworkZeekLogKind::Http
        ))
    );

    let mut unapproved = complete_input();
    unapproved.comparison_artifacts[0].approved_fixture_output = false;
    assert_eq!(
        generate_network_zeek_analyzer_proof(unapproved),
        Err(NetworkZeekAnalyzerError::MissingApprovedComparison(
            NetworkZeekLogKind::Conn
        ))
    );

    let mut mismatch = complete_input();
    mismatch.comparison_artifacts[1].matched_row_count = 0;
    assert_eq!(
        generate_network_zeek_analyzer_proof(mismatch),
        Err(NetworkZeekAnalyzerError::ComparisonMismatch(
            NetworkZeekLogKind::Dns
        ))
    );
}

#[test]
fn zeek_generator_rejects_network_only_content_signature_live_and_authority_claims() {
    assert_eq!(
        generate_network_zeek_analyzer_proof(NetworkZeekAnalyzerInput {
            exact_url_claimed: true,
            ..complete_input()
        }),
        Err(NetworkZeekAnalyzerError::ExactUrlClaimRejected)
    );
    assert_eq!(
        generate_network_zeek_analyzer_proof(NetworkZeekAnalyzerInput {
            decrypted_payload_claimed: true,
            ..complete_input()
        }),
        Err(NetworkZeekAnalyzerError::DecryptedPayloadClaimRejected)
    );
    assert_eq!(
        generate_network_zeek_analyzer_proof(NetworkZeekAnalyzerInput {
            page_content_claimed: true,
            ..complete_input()
        }),
        Err(NetworkZeekAnalyzerError::PageContentClaimRejected)
    );
    assert_eq!(
        generate_network_zeek_analyzer_proof(NetworkZeekAnalyzerInput {
            signature_alert_claimed: true,
            ..complete_input()
        }),
        Err(NetworkZeekAnalyzerError::SignatureAlertClaimRejected)
    );
    assert_eq!(
        generate_network_zeek_analyzer_proof(NetworkZeekAnalyzerInput {
            live_analyzer_invocation_claimed: true,
            ..complete_input()
        }),
        Err(NetworkZeekAnalyzerError::LiveAnalyzerInvocationClaimRejected)
    );
    assert_eq!(
        generate_network_zeek_analyzer_proof(NetworkZeekAnalyzerInput {
            policy_authority_claimed: true,
            ..complete_input()
        }),
        Err(NetworkZeekAnalyzerError::PolicyAuthorityClaimRejected)
    );
    assert_eq!(
        generate_network_zeek_analyzer_proof(NetworkZeekAnalyzerInput {
            adapter_authority_claimed: true,
            ..complete_input()
        }),
        Err(NetworkZeekAnalyzerError::AdapterAuthorityClaimRejected)
    );
    assert_eq!(
        generate_network_zeek_analyzer_proof(NetworkZeekAnalyzerInput {
            enforcement_command_claimed: true,
            ..complete_input()
        }),
        Err(NetworkZeekAnalyzerError::EnforcementCommandClaimRejected)
    );
}

#[test]
fn zeek_generator_rejects_empty_refs_and_visible_value_mismatch() {
    assert_eq!(
        generate_network_zeek_analyzer_proof(NetworkZeekAnalyzerInput {
            analyzer_run_ref: " ".to_owned(),
            ..complete_input()
        }),
        Err(NetworkZeekAnalyzerError::EmptyAnalyzerRunRef)
    );
    assert_eq!(
        generate_network_zeek_analyzer_proof(NetworkZeekAnalyzerInput {
            source_fixture_ref: " ".to_owned(),
            ..complete_input()
        }),
        Err(NetworkZeekAnalyzerError::EmptySourceFixtureRef)
    );
    assert_eq!(
        generate_network_zeek_analyzer_proof(NetworkZeekAnalyzerInput {
            http_evidence: vec![NetworkZeekHttpEvidence {
                evidence_ref: " ".to_owned(),
                ..http_evidence()
            }],
            ..complete_input()
        }),
        Err(NetworkZeekAnalyzerError::EmptyHttpEvidenceRef)
    );
    assert_eq!(
        generate_network_zeek_analyzer_proof(NetworkZeekAnalyzerInput {
            tls_evidence: vec![NetworkZeekTlsEvidence {
                flow_ref: " ".to_owned(),
                ..tls_evidence()
            }],
            ..complete_input()
        }),
        Err(NetworkZeekAnalyzerError::EmptyTlsFlowRef)
    );
    assert_eq!(
        generate_network_zeek_analyzer_proof(NetworkZeekAnalyzerInput {
            http_evidence: vec![NetworkZeekHttpEvidence {
                host: None,
                ..http_evidence()
            }],
            ..complete_input()
        }),
        Err(NetworkZeekAnalyzerError::VisibleHttpHostMissing)
    );
    assert_eq!(
        generate_network_zeek_analyzer_proof(NetworkZeekAnalyzerInput {
            tls_evidence: vec![NetworkZeekTlsEvidence {
                server_name: None,
                ..tls_evidence()
            }],
            ..complete_input()
        }),
        Err(NetworkZeekAnalyzerError::VisibleTlsServerNameMissing)
    );
    assert_eq!(
        generate_network_zeek_analyzer_proof(NetworkZeekAnalyzerInput {
            comparison_artifacts: vec![NetworkZeekAnalyzerComparisonArtifact {
                artifact_ref: " ".to_owned(),
                ..comparison_artifact(NetworkZeekLogKind::Conn, 1, 0)
            }],
            ..complete_input()
        }),
        Err(NetworkZeekAnalyzerError::EmptyComparisonArtifactRef(
            NetworkZeekLogKind::Conn
        ))
    );
}

fn complete_input() -> NetworkZeekAnalyzerInput {
    let pcap = dns_query_pcap_fixture();
    let flow_summary = aggregate_pcap_flows(&pcap, 30_000_000)
        .expect_value("DNS fixture should produce a deterministic flow summary");
    let dns_summary =
        replay_dns_observations(&pcap).expect_value("DNS fixture should produce a DNS observation");

    NetworkZeekAnalyzerInput {
        analyzer_run_ref: " zeek-analyzer-run-43 ".to_owned(),
        source_fixture_ref: " pcap-fixture-network-43 ".to_owned(),
        flow_sessions: flow_summary.sessions,
        dns_observations: dns_summary.dns_observations,
        http_evidence: vec![http_evidence()],
        tls_evidence: vec![tls_evidence()],
        ssl_evidence: vec![ssl_evidence()],
        comparison_artifacts: comparison_artifacts(1, 1, 1, 1, 1, 0),
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        signature_alert_claimed: false,
        live_analyzer_invocation_claimed: false,
        policy_authority_claimed: false,
        adapter_authority_claimed: false,
        enforcement_command_claimed: false,
    }
}

fn http_evidence() -> NetworkZeekHttpEvidence {
    let observation = parse_http_host(&http_host_request_fixture())
        .expect_value("HTTP fixture should parse")
        .expect_value("HTTP fixture should expose host only");

    NetworkZeekHttpEvidence {
        evidence_ref: " http-evidence-43 ".to_owned(),
        flow_ref: " flow-http-43 ".to_owned(),
        observed_at_micros: 1_765_000_000_124_000,
        host: Some(observation.host),
        visibility_state: NetworkZeekVisibilityState::Visible,
    }
}

fn tls_evidence() -> NetworkZeekTlsEvidence {
    let visibility = parse_tls_client_hello_sni(&tls_client_hello_sni_fixture())
        .expect_value("TLS fixture should parse");

    NetworkZeekTlsEvidence {
        evidence_ref: " tls-evidence-43 ".to_owned(),
        flow_ref: " flow-tls-43 ".to_owned(),
        observed_at_micros: 1_765_000_000_125_000,
        server_name: visibility.sni,
        visibility_state: NetworkZeekVisibilityState::Visible,
    }
}

fn ssl_evidence() -> NetworkZeekTlsEvidence {
    let visibility = parse_tls_client_hello_sni(&tls_client_hello_no_sni_fixture())
        .expect_value("TLS fixture without SNI should parse");

    NetworkZeekTlsEvidence {
        evidence_ref: " ssl-evidence-43 ".to_owned(),
        flow_ref: " flow-ssl-43 ".to_owned(),
        observed_at_micros: 1_765_000_000_126_000,
        server_name: visibility.sni,
        visibility_state: NetworkZeekVisibilityState::Missing,
    }
}

fn comparison_artifacts(
    conn_count: usize,
    dns_count: usize,
    http_count: usize,
    tls_count: usize,
    ssl_count: usize,
    preserved_unknown_or_ambiguous_rows: usize,
) -> Vec<NetworkZeekAnalyzerComparisonArtifact> {
    vec![
        comparison_artifact(NetworkZeekLogKind::Conn, conn_count, 0),
        comparison_artifact(NetworkZeekLogKind::Dns, dns_count, 0),
        comparison_artifact(
            NetworkZeekLogKind::Http,
            http_count,
            preserved_unknown_or_ambiguous_rows,
        ),
        comparison_artifact(NetworkZeekLogKind::Tls, tls_count, 0),
        comparison_artifact(NetworkZeekLogKind::Ssl, ssl_count, 0),
    ]
}

fn comparison_artifact(
    log_kind: NetworkZeekLogKind,
    row_count: usize,
    preserved_unknown_or_ambiguous_rows: usize,
) -> NetworkZeekAnalyzerComparisonArtifact {
    NetworkZeekAnalyzerComparisonArtifact {
        log_kind,
        artifact_ref: format!(" approved-{log_kind:?}-comparison-artifact-43 "),
        expected_row_count: row_count,
        observed_row_count: row_count,
        matched_row_count: row_count,
        preserved_unknown_or_ambiguous_rows,
        approved_fixture_output: true,
    }
}
