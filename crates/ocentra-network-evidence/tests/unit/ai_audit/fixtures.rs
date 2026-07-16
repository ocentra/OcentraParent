use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::ai_audit::*;
use ocentra_network_evidence::ai_detection::*;

pub(super) fn signature_detection_case() -> NetworkAiDetectionFixtureCase {
    NetworkAiDetectionFixtureCase {
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
    }
}

pub(super) fn unknown_detection_case() -> NetworkAiDetectionFixtureCase {
    NetworkAiDetectionFixtureCase {
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
    }
}

pub(super) fn update_detection_case() -> NetworkAiDetectionFixtureCase {
    NetworkAiDetectionFixtureCase {
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
    }
}

pub(super) fn audit_input(
    detection_results: Vec<NetworkAiDetectionResult>,
) -> NetworkAiAuditReportInput {
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

pub(super) fn detection_results(
    cases: Vec<NetworkAiDetectionFixtureCase>,
) -> Vec<NetworkAiDetectionResult> {
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
