use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::signature_alert::*;

#[test]
fn signature_alert_ingestion_converts_suricata_and_snort_rows_to_typed_records() {
    let input = complete_input();
    let proof = ingest_network_signature_alerts(&input)
        .expect_value("signature alert fixtures should ingest as typed analyzer records");

    assert_eq!(proof.ingestion_run_ref, "signature-alert-run-44");
    assert_eq!(proof.fixture_ref, "signature-alert-fixture-44");
    assert_eq!(proof.records.len(), 2);
    assert_eq!(proof.suricata_record_count, 1);
    assert_eq!(proof.snort_compatible_record_count, 1);
    assert_eq!(proof.false_positive_record_count, 0);
    assert_eq!(proof.detection_candidate_count, 1);
    assert_eq!(proof.analyzer_alert_events_published, 2);
    assert_eq!(proof.adapter_calls_authorized, 0);
    assert_eq!(proof.enforcement_commands_published, 0);
    assert!(!proof.live_suricata_invoked);
    assert!(!proof.live_snort_invoked);
    assert!(!proof.ips_prevention_claimed);

    let suricata = &proof.records[0];
    assert_eq!(suricata.alert_ref, "suricata-alert-44");
    assert_eq!(suricata.source, NetworkSignatureAlertSource::Suricata);
    assert_eq!(suricata.signature_id, "sid:2027444");
    assert_eq!(suricata.rule_source_ref, "rule-source-et-open-44");
    assert_eq!(suricata.flow_ref, "flow-ref-44");
    assert_eq!(suricata.evidence_ref, "network-evidence-ref-44");
    assert_eq!(suricata.custody_ref, "custody-ref-44");
    assert_eq!(
        suricata.alert_state,
        NetworkSignatureAlertState::ReviewCandidate
    );
    assert!(suricata.analyzer_alert_event_published);
    assert!(suricata.detection_candidate);
    assert!(suricata.parent_review_candidate);
    assert!(!suricata.policy_authority);
    assert!(!suricata.adapter_authority);
    assert!(!suricata.enforcement_command_published);
    assert!(!suricata.exact_url_available);
    assert!(!suricata.decrypted_payload_available);
    assert!(!suricata.page_content_available);

    let snort = &proof.records[1];
    assert_eq!(snort.source, NetworkSignatureAlertSource::SnortCompatible);
    assert_eq!(
        snort.alert_state,
        NetworkSignatureAlertState::AnalyzerEvidenceOnly
    );
    assert!(!snort.detection_candidate);
    assert!(!snort.parent_review_candidate);
}

#[test]
fn signature_alert_ingestion_keeps_false_positive_fixture_non_enforcing() {
    let input = NetworkSignatureAlertIngestionInput {
        rows: vec![NetworkSignatureAlertFixtureRow {
            known_false_positive: true,
            severity: NetworkSignatureAlertSeverity::Critical,
            ..suricata_row()
        }],
        ..complete_input()
    };
    let proof = ingest_network_signature_alerts(&input)
        .expect_value("known false-positive alert fixture should remain recordable");

    assert_eq!(proof.records.len(), 1);
    assert_eq!(proof.false_positive_record_count, 1);
    assert_eq!(proof.detection_candidate_count, 0);
    assert_eq!(proof.adapter_calls_authorized, 0);
    assert_eq!(proof.enforcement_commands_published, 0);
    assert_eq!(
        proof.records[0].alert_state,
        NetworkSignatureAlertState::FalsePositiveNonEnforcing
    );
    assert!(proof.records[0].false_positive);
    assert!(proof.records[0].analyzer_alert_event_published);
    assert!(!proof.records[0].detection_candidate);
    assert!(!proof.records[0].adapter_authority);
    assert!(!proof.records[0].enforcement_command_published);
}

#[test]
fn signature_alert_ingestion_rejects_content_live_ips_and_authority_claims() {
    assert_eq!(
        ingest_network_signature_alerts(&NetworkSignatureAlertIngestionInput {
            rows: vec![NetworkSignatureAlertFixtureRow {
                exact_url_claimed: true,
                ..suricata_row()
            }],
            ..complete_input()
        }),
        Err(NetworkSignatureAlertIngestionError::ExactUrlClaimRejected)
    );
    assert_eq!(
        ingest_network_signature_alerts(&NetworkSignatureAlertIngestionInput {
            rows: vec![NetworkSignatureAlertFixtureRow {
                decrypted_payload_claimed: true,
                ..suricata_row()
            }],
            ..complete_input()
        }),
        Err(NetworkSignatureAlertIngestionError::DecryptedPayloadClaimRejected)
    );
    assert_eq!(
        ingest_network_signature_alerts(&NetworkSignatureAlertIngestionInput {
            rows: vec![NetworkSignatureAlertFixtureRow {
                page_content_claimed: true,
                ..suricata_row()
            }],
            ..complete_input()
        }),
        Err(NetworkSignatureAlertIngestionError::PageContentClaimRejected)
    );
    assert_eq!(
        ingest_network_signature_alerts(&NetworkSignatureAlertIngestionInput {
            live_suricata_invocation_claimed: true,
            ..complete_input()
        }),
        Err(NetworkSignatureAlertIngestionError::LiveSuricataInvocationClaimRejected)
    );
    assert_eq!(
        ingest_network_signature_alerts(&NetworkSignatureAlertIngestionInput {
            live_snort_invocation_claimed: true,
            ..complete_input()
        }),
        Err(NetworkSignatureAlertIngestionError::LiveSnortInvocationClaimRejected)
    );
    assert_eq!(
        ingest_network_signature_alerts(&NetworkSignatureAlertIngestionInput {
            ips_prevention_claimed: true,
            ..complete_input()
        }),
        Err(NetworkSignatureAlertIngestionError::IpsPreventionClaimRejected)
    );
    assert_eq!(
        ingest_network_signature_alerts(&NetworkSignatureAlertIngestionInput {
            policy_authority_claimed: true,
            ..complete_input()
        }),
        Err(NetworkSignatureAlertIngestionError::PolicyAuthorityClaimRejected)
    );
    assert_eq!(
        ingest_network_signature_alerts(&NetworkSignatureAlertIngestionInput {
            adapter_authority_claimed: true,
            ..complete_input()
        }),
        Err(NetworkSignatureAlertIngestionError::AdapterAuthorityClaimRejected)
    );
    assert_eq!(
        ingest_network_signature_alerts(&NetworkSignatureAlertIngestionInput {
            enforcement_command_claimed: true,
            ..complete_input()
        }),
        Err(NetworkSignatureAlertIngestionError::EnforcementCommandClaimRejected)
    );
}

#[test]
fn signature_alert_ingestion_rejects_empty_ingestion_refs() {
    assert_eq!(
        ingest_network_signature_alerts(&NetworkSignatureAlertIngestionInput {
            ingestion_run_ref: " ".to_owned(),
            ..complete_input()
        }),
        Err(NetworkSignatureAlertIngestionError::EmptyIngestionRunRef)
    );
    assert_eq!(
        ingest_network_signature_alerts(&NetworkSignatureAlertIngestionInput {
            fixture_ref: " ".to_owned(),
            ..complete_input()
        }),
        Err(NetworkSignatureAlertIngestionError::EmptyFixtureRef)
    );
    assert_eq!(
        ingest_network_signature_alerts(&NetworkSignatureAlertIngestionInput {
            rows: Vec::new(),
            ..complete_input()
        }),
        Err(NetworkSignatureAlertIngestionError::EmptyAlertRows)
    );
}

#[test]
fn signature_alert_ingestion_rejects_empty_alert_identity_refs() {
    assert_eq!(
        ingest_network_signature_alerts(&NetworkSignatureAlertIngestionInput {
            rows: vec![NetworkSignatureAlertFixtureRow {
                alert_ref: " ".to_owned(),
                ..suricata_row()
            }],
            ..complete_input()
        }),
        Err(NetworkSignatureAlertIngestionError::EmptyAlertRef)
    );
    assert_eq!(
        ingest_network_signature_alerts(&NetworkSignatureAlertIngestionInput {
            rows: vec![NetworkSignatureAlertFixtureRow {
                signature_id: " ".to_owned(),
                ..suricata_row()
            }],
            ..complete_input()
        }),
        Err(NetworkSignatureAlertIngestionError::EmptySignatureId)
    );
    assert_eq!(
        ingest_network_signature_alerts(&NetworkSignatureAlertIngestionInput {
            rows: vec![NetworkSignatureAlertFixtureRow {
                rule_source_ref: " ".to_owned(),
                ..suricata_row()
            }],
            ..complete_input()
        }),
        Err(NetworkSignatureAlertIngestionError::EmptyRuleSourceRef)
    );
}

#[test]
fn signature_alert_ingestion_rejects_empty_flow_custody_refs_and_duplicates() {
    assert_eq!(
        ingest_network_signature_alerts(&NetworkSignatureAlertIngestionInput {
            rows: vec![NetworkSignatureAlertFixtureRow {
                flow_ref: " ".to_owned(),
                ..suricata_row()
            }],
            ..complete_input()
        }),
        Err(NetworkSignatureAlertIngestionError::EmptyFlowRef)
    );
    assert_eq!(
        ingest_network_signature_alerts(&NetworkSignatureAlertIngestionInput {
            rows: vec![NetworkSignatureAlertFixtureRow {
                evidence_ref: " ".to_owned(),
                ..suricata_row()
            }],
            ..complete_input()
        }),
        Err(NetworkSignatureAlertIngestionError::EmptyEvidenceRef)
    );
    assert_eq!(
        ingest_network_signature_alerts(&NetworkSignatureAlertIngestionInput {
            rows: vec![NetworkSignatureAlertFixtureRow {
                custody_ref: " ".to_owned(),
                ..suricata_row()
            }],
            ..complete_input()
        }),
        Err(NetworkSignatureAlertIngestionError::EmptyCustodyRef)
    );
    assert_eq!(
        ingest_network_signature_alerts(&NetworkSignatureAlertIngestionInput {
            rows: vec![suricata_row(), suricata_row()],
            ..complete_input()
        }),
        Err(NetworkSignatureAlertIngestionError::DuplicateAlertRef)
    );
}

fn complete_input() -> NetworkSignatureAlertIngestionInput {
    NetworkSignatureAlertIngestionInput {
        ingestion_run_ref: " signature-alert-run-44 ".to_owned(),
        fixture_ref: " signature-alert-fixture-44 ".to_owned(),
        rows: vec![suricata_row(), snort_row()],
        live_suricata_invocation_claimed: false,
        live_snort_invocation_claimed: false,
        ips_prevention_claimed: false,
        policy_authority_claimed: false,
        adapter_authority_claimed: false,
        enforcement_command_claimed: false,
    }
}

fn suricata_row() -> NetworkSignatureAlertFixtureRow {
    NetworkSignatureAlertFixtureRow {
        alert_ref: " suricata-alert-44 ".to_owned(),
        source: NetworkSignatureAlertSource::Suricata,
        signature_id: " sid:2027444 ".to_owned(),
        signature_name: " Suspicious TLS SNI fixture ".to_owned(),
        rule_source_ref: " rule-source-et-open-44 ".to_owned(),
        severity: NetworkSignatureAlertSeverity::High,
        observed_at_micros: 1_765_000_000_140_000,
        flow_ref: " flow-ref-44 ".to_owned(),
        evidence_ref: " network-evidence-ref-44 ".to_owned(),
        custody_ref: " custody-ref-44 ".to_owned(),
        known_false_positive: false,
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
    }
}

fn snort_row() -> NetworkSignatureAlertFixtureRow {
    NetworkSignatureAlertFixtureRow {
        alert_ref: " snort-compatible-alert-44 ".to_owned(),
        source: NetworkSignatureAlertSource::SnortCompatible,
        signature_id: " gid:1 sid:10000044 ".to_owned(),
        signature_name: " Snort compatible DNS watchlist fixture ".to_owned(),
        rule_source_ref: " rule-source-snort-compatible-44 ".to_owned(),
        severity: NetworkSignatureAlertSeverity::Medium,
        observed_at_micros: 1_765_000_000_141_000,
        flow_ref: " flow-ref-44 ".to_owned(),
        evidence_ref: " network-evidence-ref-44 ".to_owned(),
        custody_ref: " custody-ref-44 ".to_owned(),
        known_false_positive: false,
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
    }
}
