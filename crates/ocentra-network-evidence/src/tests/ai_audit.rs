use crate::{
    build_network_ai_audit_report, evaluate_network_ai_detection_fixtures,
    NetworkAiAuditNarrativeState, NetworkAiAuditRecommendationKind, NetworkAiAuditReportError,
    NetworkAiAuditReportInput, NetworkAiAuditUncertaintyCode, NetworkAiDetectionEvaluationInput,
    NetworkAiDetectionFixtureCase, NetworkAiDetectionInputKind, NetworkAiDetectionLabel,
    NetworkAiDetectionResult, NetworkAiDetectionRiskLevel,
};

#[test]
fn ai_audit_report_generates_parent_readable_narrative_with_cited_refs() {
    let report =
        build_network_ai_audit_report(audit_input(detection_results(vec![detection_case(
            "detect-signature-1",
            NetworkAiDetectionLabel::SignatureThreat,
            NetworkAiDetectionLabel::SignatureThreat,
            9_100,
            8_900,
            NetworkAiDetectionRiskLevel::Critical,
            vec!["analyzer-alert-critical"],
        )])))
        .expect("high-risk detection should produce a parent-readable audit");

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
    let report =
        build_network_ai_audit_report(audit_input(detection_results(vec![detection_case(
            "detect-unknown-1",
            NetworkAiDetectionLabel::UnknownHighVolume,
            NetworkAiDetectionLabel::Unknown,
            4_200,
            4_100,
            NetworkAiDetectionRiskLevel::Unknown,
            vec![],
        )])))
        .expect("uncertain detection should produce advisory confirmation");

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
    let report =
        build_network_ai_audit_report(audit_input(detection_results(vec![detection_case(
            "detect-update-1",
            NetworkAiDetectionLabel::BenignExpected,
            NetworkAiDetectionLabel::BenignExpected,
            8_200,
            8_100,
            NetworkAiDetectionRiskLevel::Low,
            vec![],
        )])))
        .expect("benign cited detection should produce monitor-only audit");

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
    assert_eq!(
        build_network_ai_audit_report(NetworkAiAuditReportInput {
            remote_ai_claimed: true,
            ..audit_input(detection_results(vec![passing_detection_case()]))
        }),
        Err(NetworkAiAuditReportError::RemoteAiClaimRejected)
    );
    assert_eq!(
        build_network_ai_audit_report(NetworkAiAuditReportInput {
            exact_url_claimed: true,
            ..audit_input(detection_results(vec![passing_detection_case()]))
        }),
        Err(NetworkAiAuditReportError::ExactUrlClaimRejected)
    );
    assert_eq!(
        build_network_ai_audit_report(NetworkAiAuditReportInput {
            private_message_claimed: true,
            ..audit_input(detection_results(vec![passing_detection_case()]))
        }),
        Err(NetworkAiAuditReportError::PrivateMessageClaimRejected)
    );
    assert_eq!(
        build_network_ai_audit_report(NetworkAiAuditReportInput {
            policy_authority_claimed: true,
            ..audit_input(detection_results(vec![passing_detection_case()]))
        }),
        Err(NetworkAiAuditReportError::PolicyAuthorityClaimRejected)
    );
    let mut detection = detection_results(vec![passing_detection_case()])
        .pop()
        .expect("test detection exists");
    detection.adapter_authority = true;
    assert_eq!(
        build_network_ai_audit_report(audit_input(vec![detection])),
        Err(NetworkAiAuditReportError::AdapterAuthorityClaimRejected)
    );
}

#[test]
fn ai_audit_report_rejects_missing_citations_and_duplicate_detections() {
    assert_eq!(
        build_network_ai_audit_report(NetworkAiAuditReportInput {
            audit_report_ref: " ".to_owned(),
            ..audit_input(detection_results(vec![passing_detection_case()]))
        }),
        Err(NetworkAiAuditReportError::EmptyAuditReportRef)
    );
    assert_eq!(
        build_network_ai_audit_report(NetworkAiAuditReportInput {
            detection_results: vec![],
            ..audit_input(detection_results(vec![passing_detection_case()]))
        }),
        Err(NetworkAiAuditReportError::EmptyDetectionResults)
    );
    assert_eq!(
        build_network_ai_audit_report(NetworkAiAuditReportInput {
            parent_rule_refs: vec![],
            ..audit_input(detection_results(vec![passing_detection_case()]))
        }),
        Err(NetworkAiAuditReportError::EmptyParentRuleRefs)
    );

    let duplicate = detection_results(vec![passing_detection_case()]);
    assert_eq!(
        build_network_ai_audit_report(audit_input(vec![
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
    evaluate_network_ai_detection_fixtures(NetworkAiDetectionEvaluationInput {
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
    .expect("detection fixture should evaluate")
    .results
}

fn passing_detection_case() -> NetworkAiDetectionFixtureCase {
    detection_case(
        "detect-signature-1",
        NetworkAiDetectionLabel::SignatureThreat,
        NetworkAiDetectionLabel::SignatureThreat,
        9_100,
        8_900,
        NetworkAiDetectionRiskLevel::Critical,
        vec!["analyzer-alert-critical"],
    )
}

fn detection_case(
    detection_ref: &str,
    expected_label: NetworkAiDetectionLabel,
    predicted_label: NetworkAiDetectionLabel,
    confidence_basis_points: u16,
    baseline_confidence_basis_points: u16,
    risk_level: NetworkAiDetectionRiskLevel,
    analyzer_alert_refs: Vec<&str>,
) -> NetworkAiDetectionFixtureCase {
    let mut input_kinds = vec![
        NetworkAiDetectionInputKind::SummaryRefs,
        NetworkAiDetectionInputKind::EvidenceRefs,
        NetworkAiDetectionInputKind::FixtureLabel,
    ];
    if !analyzer_alert_refs.is_empty() {
        input_kinds.push(NetworkAiDetectionInputKind::AnalyzerAlertRefs);
    }

    NetworkAiDetectionFixtureCase {
        detection_ref: detection_ref.to_owned(),
        fixture_ref: format!("fixture-{detection_ref}"),
        summary_ref: format!("summary-{detection_ref}"),
        evidence_refs: vec![format!("evidence-{detection_ref}")],
        analyzer_alert_refs: analyzer_alert_refs.into_iter().map(str::to_owned).collect(),
        expected_label,
        predicted_label,
        confidence_basis_points,
        baseline_confidence_basis_points,
        risk_level,
        input_kinds,
        raw_pcap_input_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        exact_url_claimed: false,
    }
}
