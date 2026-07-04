use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::ai_audit::*;
use ocentra_network_evidence::ai_detection::*;

#[test]
fn ai_audit_report_generates_parent_readable_narrative_with_cited_refs() {
    let signature_case = NetworkAiDetectionFixtureCase {
        detection_ref: "detect-signature-1".to_owned(),
        fixture_ref: "fixture-detect-signature-1".to_owned(),
        summary_ref: "summary-detect-signature-1".to_owned(),
        evidence_refs: vec!["evidence-detect-signature-1".to_owned()],
        analyzer_alert_refs: vec!["analyzer-alert-critical".to_owned()],
        expected_label: NetworkAiDetectionLabel::SignatureThreat,
        predicted_label: NetworkAiDetectionLabel::SignatureThreat,
        confidence_basis_points: 9_100,
        baseline_confidence_basis_points: 8_900,
        risk_level: NetworkAiDetectionRiskLevel::Critical,
        input_kinds: vec![
            NetworkAiDetectionInputKind::SummaryRefs,
            NetworkAiDetectionInputKind::EvidenceRefs,
            NetworkAiDetectionInputKind::FixtureLabel,
            NetworkAiDetectionInputKind::AnalyzerAlertRefs,
        ],
        raw_pcap_input_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        exact_url_claimed: false,
    };
    let report =
        build_network_ai_audit_report(&audit_input(detection_results(vec![signature_case])))
            .expect_value("high-risk detection should produce a parent-readable audit");

    assert_eq!(report.narrative_state, NetworkAiAuditNarrativeState::Ready);
    assert_eq!(
        report.narrative_headline,
        "Network AI audit recommends parent review for cited high-risk network detections."
    );
    assert_eq!(report.cited_detection_refs, vec!["detect-signature-1"]);
    assert_eq!(
        report.cited_evidence_refs,
        vec!["evidence-detect-signature-1"]
    );
    assert_eq!(
        report.cited_analyzer_alert_refs,
        vec!["analyzer-alert-critical"]
    );
    assert_eq!(
        report.cited_parent_rule_refs,
        vec!["parent-rule-network-review"]
    );
    assert_eq!(report.recommendations.len(), 2);
    assert_eq!(
        report.recommendations[0].kind,
        NetworkAiAuditRecommendationKind::ReviewWithParent
    );
    assert_eq!(
        report.recommendations[1].kind,
        NetworkAiAuditRecommendationKind::ReviewPolicyRule
    );
    assert!(report.recommendations.iter().all(|recommendation| {
        recommendation.advisory_only
            && !recommendation.policy_authority
            && !recommendation.adapter_authority
            && !recommendation.enforcement_command_published
            && recommendation.cited_evidence_refs == report.cited_evidence_refs
    }));
    assert!(report.parent_readable);
    assert!(report.advisory_only);
    assert!(!report.remote_ai_used);
    assert!(!report.policy_authority);
    assert!(!report.adapter_authority);
    assert_eq!(report.enforcement_commands_published, 0);
}

#[test]
fn ai_audit_report_recommends_confirmation_for_uncertain_detection() {
    let unknown_case = NetworkAiDetectionFixtureCase {
        detection_ref: "detect-unknown-1".to_owned(),
        fixture_ref: "fixture-detect-unknown-1".to_owned(),
        summary_ref: "summary-detect-unknown-1".to_owned(),
        evidence_refs: vec!["evidence-detect-unknown-1".to_owned()],
        analyzer_alert_refs: Vec::new(),
        expected_label: NetworkAiDetectionLabel::UnknownHighVolume,
        predicted_label: NetworkAiDetectionLabel::Unknown,
        confidence_basis_points: 4_200,
        baseline_confidence_basis_points: 4_100,
        risk_level: NetworkAiDetectionRiskLevel::Unknown,
        input_kinds: vec![
            NetworkAiDetectionInputKind::SummaryRefs,
            NetworkAiDetectionInputKind::EvidenceRefs,
            NetworkAiDetectionInputKind::FixtureLabel,
        ],
        raw_pcap_input_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        exact_url_claimed: false,
    };
    let report = build_network_ai_audit_report(&audit_input(detection_results(vec![unknown_case])))
        .expect_value("uncertain detection should produce advisory confirmation");

    assert_eq!(
        report.narrative_state,
        NetworkAiAuditNarrativeState::UncertainReviewRequired
    );
    assert_eq!(
        report.narrative_headline,
        "Network AI audit found uncertainty and recommends evidence confirmation before policy action."
    );
    assert!(report
        .uncertainty_codes
        .contains(&NetworkAiAuditUncertaintyCode::UnknownPrediction));
    assert!(report
        .uncertainty_codes
        .contains(&NetworkAiAuditUncertaintyCode::LowConfidence));
    assert_eq!(report.recommendations.len(), 2);
    assert_eq!(
        report.recommendations[0].kind,
        NetworkAiAuditRecommendationKind::ConfirmWithManagedBrowser
    );
    assert_eq!(
        report.recommendations[1].kind,
        NetworkAiAuditRecommendationKind::ConfirmWithScreenSummary
    );
    assert!(!report.exact_url_available);
    assert!(!report.page_content_available);
    assert!(!report.decrypted_payload_available);
}

#[test]
fn ai_audit_report_uses_monitor_only_for_non_high_risk_cited_detection() {
    let update_case = NetworkAiDetectionFixtureCase {
        detection_ref: "detect-update-1".to_owned(),
        fixture_ref: "fixture-detect-update-1".to_owned(),
        summary_ref: "summary-detect-update-1".to_owned(),
        evidence_refs: vec!["evidence-detect-update-1".to_owned()],
        analyzer_alert_refs: Vec::new(),
        expected_label: NetworkAiDetectionLabel::BenignExpected,
        predicted_label: NetworkAiDetectionLabel::BenignExpected,
        confidence_basis_points: 8_200,
        baseline_confidence_basis_points: 8_100,
        risk_level: NetworkAiDetectionRiskLevel::Low,
        input_kinds: vec![
            NetworkAiDetectionInputKind::SummaryRefs,
            NetworkAiDetectionInputKind::EvidenceRefs,
            NetworkAiDetectionInputKind::FixtureLabel,
        ],
        raw_pcap_input_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        exact_url_claimed: false,
    };
    let report = build_network_ai_audit_report(&audit_input(detection_results(vec![update_case])))
        .expect_value("benign cited detection should produce monitor-only audit");

    assert_eq!(
        report.narrative_state,
        NetworkAiAuditNarrativeState::MonitorOnly
    );
    assert_eq!(report.recommendations.len(), 1);
    assert_eq!(
        report.recommendations[0].kind,
        NetworkAiAuditRecommendationKind::MonitorOnly
    );
    assert_eq!(report.cited_evidence_refs, vec!["evidence-detect-update-1"]);
    assert!(report.uncertainty_codes.is_empty());
}

#[test]
fn ai_audit_report_rejects_unsupported_content_and_authority_claims() {
    let passing_detection = NetworkAiDetectionFixtureCase {
        detection_ref: "detect-signature-1".to_owned(),
        fixture_ref: "fixture-detect-signature-1".to_owned(),
        summary_ref: "summary-detect-signature-1".to_owned(),
        evidence_refs: vec!["evidence-detect-signature-1".to_owned()],
        analyzer_alert_refs: vec!["analyzer-alert-critical".to_owned()],
        expected_label: NetworkAiDetectionLabel::SignatureThreat,
        predicted_label: NetworkAiDetectionLabel::SignatureThreat,
        confidence_basis_points: 9_100,
        baseline_confidence_basis_points: 8_900,
        risk_level: NetworkAiDetectionRiskLevel::Critical,
        input_kinds: vec![
            NetworkAiDetectionInputKind::SummaryRefs,
            NetworkAiDetectionInputKind::EvidenceRefs,
            NetworkAiDetectionInputKind::FixtureLabel,
            NetworkAiDetectionInputKind::AnalyzerAlertRefs,
        ],
        raw_pcap_input_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        exact_url_claimed: false,
    };
    assert_eq!(
        build_network_ai_audit_report(&NetworkAiAuditReportInput {
            remote_ai_claimed: true,
            ..audit_input(detection_results(vec![passing_detection.clone()]))
        }),
        Err(NetworkAiAuditReportError::RemoteAiClaimRejected)
    );
    assert_eq!(
        build_network_ai_audit_report(&NetworkAiAuditReportInput {
            exact_url_claimed: true,
            ..audit_input(detection_results(vec![passing_detection.clone()]))
        }),
        Err(NetworkAiAuditReportError::ExactUrlClaimRejected)
    );
    assert_eq!(
        build_network_ai_audit_report(&NetworkAiAuditReportInput {
            private_message_claimed: true,
            ..audit_input(detection_results(vec![passing_detection.clone()]))
        }),
        Err(NetworkAiAuditReportError::PrivateMessageClaimRejected)
    );
    assert_eq!(
        build_network_ai_audit_report(&NetworkAiAuditReportInput {
            policy_authority_claimed: true,
            ..audit_input(detection_results(vec![passing_detection.clone()]))
        }),
        Err(NetworkAiAuditReportError::PolicyAuthorityClaimRejected)
    );
    let mut detection = detection_results(vec![passing_detection])
        .pop()
        .expect_value("test detection exists");
    detection.adapter_authority = true;
    assert_eq!(
        build_network_ai_audit_report(&audit_input(vec![detection])),
        Err(NetworkAiAuditReportError::AdapterAuthorityClaimRejected)
    );
}

#[test]
fn ai_audit_report_rejects_missing_citations_and_duplicate_detections() {
    assert_eq!(
        build_network_ai_audit_report(&NetworkAiAuditReportInput {
            audit_report_ref: " ".to_owned(),
            ..audit_input(detection_results(vec![NetworkAiDetectionFixtureCase {
                detection_ref: "detect-signature-1".to_owned(),
                fixture_ref: "fixture-detect-signature-1".to_owned(),
                summary_ref: "summary-detect-signature-1".to_owned(),
                evidence_refs: vec!["evidence-detect-signature-1".to_owned()],
                analyzer_alert_refs: vec!["analyzer-alert-critical".to_owned()],
                expected_label: NetworkAiDetectionLabel::SignatureThreat,
                predicted_label: NetworkAiDetectionLabel::SignatureThreat,
                confidence_basis_points: 9_100,
                baseline_confidence_basis_points: 8_900,
                risk_level: NetworkAiDetectionRiskLevel::Critical,
                input_kinds: vec![
                    NetworkAiDetectionInputKind::SummaryRefs,
                    NetworkAiDetectionInputKind::EvidenceRefs,
                    NetworkAiDetectionInputKind::FixtureLabel,
                    NetworkAiDetectionInputKind::AnalyzerAlertRefs,
                ],
                raw_pcap_input_claimed: false,
                decrypted_payload_claimed: false,
                page_content_claimed: false,
                exact_url_claimed: false,
            }]))
        }),
        Err(NetworkAiAuditReportError::EmptyAuditReportRef)
    );
    assert_eq!(
        build_network_ai_audit_report(&NetworkAiAuditReportInput {
            detection_results: vec![],
            ..audit_input(detection_results(vec![NetworkAiDetectionFixtureCase {
                detection_ref: "detect-signature-1".to_owned(),
                fixture_ref: "fixture-detect-signature-1".to_owned(),
                summary_ref: "summary-detect-signature-1".to_owned(),
                evidence_refs: vec!["evidence-detect-signature-1".to_owned()],
                analyzer_alert_refs: vec!["analyzer-alert-critical".to_owned()],
                expected_label: NetworkAiDetectionLabel::SignatureThreat,
                predicted_label: NetworkAiDetectionLabel::SignatureThreat,
                confidence_basis_points: 9_100,
                baseline_confidence_basis_points: 8_900,
                risk_level: NetworkAiDetectionRiskLevel::Critical,
                input_kinds: vec![
                    NetworkAiDetectionInputKind::SummaryRefs,
                    NetworkAiDetectionInputKind::EvidenceRefs,
                    NetworkAiDetectionInputKind::FixtureLabel,
                    NetworkAiDetectionInputKind::AnalyzerAlertRefs,
                ],
                raw_pcap_input_claimed: false,
                decrypted_payload_claimed: false,
                page_content_claimed: false,
                exact_url_claimed: false,
            }]))
        }),
        Err(NetworkAiAuditReportError::EmptyDetectionResults)
    );
    assert_eq!(
        build_network_ai_audit_report(&NetworkAiAuditReportInput {
            parent_rule_refs: vec![],
            ..audit_input(detection_results(vec![NetworkAiDetectionFixtureCase {
                detection_ref: "detect-signature-1".to_owned(),
                fixture_ref: "fixture-detect-signature-1".to_owned(),
                summary_ref: "summary-detect-signature-1".to_owned(),
                evidence_refs: vec!["evidence-detect-signature-1".to_owned()],
                analyzer_alert_refs: vec!["analyzer-alert-critical".to_owned()],
                expected_label: NetworkAiDetectionLabel::SignatureThreat,
                predicted_label: NetworkAiDetectionLabel::SignatureThreat,
                confidence_basis_points: 9_100,
                baseline_confidence_basis_points: 8_900,
                risk_level: NetworkAiDetectionRiskLevel::Critical,
                input_kinds: vec![
                    NetworkAiDetectionInputKind::SummaryRefs,
                    NetworkAiDetectionInputKind::EvidenceRefs,
                    NetworkAiDetectionInputKind::FixtureLabel,
                    NetworkAiDetectionInputKind::AnalyzerAlertRefs,
                ],
                raw_pcap_input_claimed: false,
                decrypted_payload_claimed: false,
                page_content_claimed: false,
                exact_url_claimed: false,
            }]))
        }),
        Err(NetworkAiAuditReportError::EmptyParentRuleRefs)
    );

    let duplicate = detection_results(vec![NetworkAiDetectionFixtureCase {
        detection_ref: "detect-signature-1".to_owned(),
        fixture_ref: "fixture-detect-signature-1".to_owned(),
        summary_ref: "summary-detect-signature-1".to_owned(),
        evidence_refs: vec!["evidence-detect-signature-1".to_owned()],
        analyzer_alert_refs: vec!["analyzer-alert-critical".to_owned()],
        expected_label: NetworkAiDetectionLabel::SignatureThreat,
        predicted_label: NetworkAiDetectionLabel::SignatureThreat,
        confidence_basis_points: 9_100,
        baseline_confidence_basis_points: 8_900,
        risk_level: NetworkAiDetectionRiskLevel::Critical,
        input_kinds: vec![
            NetworkAiDetectionInputKind::SummaryRefs,
            NetworkAiDetectionInputKind::EvidenceRefs,
            NetworkAiDetectionInputKind::FixtureLabel,
            NetworkAiDetectionInputKind::AnalyzerAlertRefs,
        ],
        raw_pcap_input_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        exact_url_claimed: false,
    }]);
    assert_eq!(
        build_network_ai_audit_report(&audit_input(vec![
            duplicate[0].clone(),
            duplicate[0].clone()
        ])),
        Err(NetworkAiAuditReportError::DuplicateDetectionRef)
    );
}

fn audit_input(detection_results: Vec<NetworkAiDetectionResult>) -> NetworkAiAuditReportInput {
    NetworkAiAuditReportInput {
        audit_report_ref: " network-ai-audit-row47 ".to_owned(),
        narrative_template_ref: "network-ai-audit-template-row47".to_owned(),
        model_version_ref: "local-ai-model-version-fixture-row46".to_owned(),
        policy_context_ref: "network-policy-context-row47".to_owned(),
        detection_results,
        parent_rule_refs: vec![
            " parent-rule-network-review ".to_owned(),
            "parent-rule-network-review".to_owned(),
        ],
        remote_ai_claimed: false,
        raw_pcap_input_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        exact_url_claimed: false,
        private_message_claimed: false,
        search_query_claimed: false,
        policy_authority_claimed: false,
        adapter_authority_claimed: false,
        enforcement_command_claimed: false,
    }
}

fn detection_results(cases: Vec<NetworkAiDetectionFixtureCase>) -> Vec<NetworkAiDetectionResult> {
    evaluate_network_ai_detection_fixtures(&NetworkAiDetectionEvaluationInput {
        evaluation_run_ref: "ai-detection-eval-row47".to_owned(),
        fixture_set_ref: "network-ai-fixtures-row47".to_owned(),
        model_card_ref: "local-ai-model-card-network-row47".to_owned(),
        model_version_ref: "local-ai-model-version-fixture-row47".to_owned(),
        baseline_ref: "local-ai-baseline-fixture-row47".to_owned(),
        cases,
        minimum_precision_basis_points: 6_000,
        minimum_recall_basis_points: 6_000,
        maximum_average_drift_basis_points: 500,
        model_execution_claimed: false,
        remote_ai_claimed: false,
        raw_pcap_input_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        exact_url_claimed: false,
        policy_authority_claimed: false,
        adapter_authority_claimed: false,
        enforcement_command_claimed: false,
    })
    .expect_value("detection fixture should evaluate")
    .results
}
