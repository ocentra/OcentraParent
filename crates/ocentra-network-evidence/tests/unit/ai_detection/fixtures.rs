use ocentra_network_evidence::ai_detection::*;

pub(super) fn vpn_case() -> NetworkAiDetectionFixtureCase {
    NetworkAiDetectionFixtureCase {
        detection_ref: "detect-vpn-1".to_owned(),
        fixture_ref: "fixture-detect-vpn-1".to_owned(),
        summary_ref: "summary-detect-vpn-1".to_owned(),
        evidence_refs: vec!["evidence-detect-vpn-1".to_owned()],
        analyzer_alert_refs: Vec::new(),
        expected_label: NetworkAiDetectionLabel::VpnProxyTunnel,
        predicted_label: NetworkAiDetectionLabel::VpnProxyTunnel,
        confidence_basis_points: 8_700,
        baseline_confidence_basis_points: 8_500,
        risk_level: NetworkAiDetectionRiskLevel::High,
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

pub(super) fn signature_tp_case() -> NetworkAiDetectionFixtureCase {
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

pub(super) fn update_case() -> NetworkAiDetectionFixtureCase {
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

pub(super) fn vpn_fp_case() -> NetworkAiDetectionFixtureCase {
    NetworkAiDetectionFixtureCase {
        detection_ref: "detect-vpn-fp".to_owned(),
        fixture_ref: "fixture-detect-vpn-fp".to_owned(),
        summary_ref: "summary-detect-vpn-fp".to_owned(),
        evidence_refs: vec!["evidence-detect-vpn-fp".to_owned()],
        analyzer_alert_refs: Vec::new(),
        expected_label: NetworkAiDetectionLabel::BenignExpected,
        predicted_label: NetworkAiDetectionLabel::VpnProxyTunnel,
        confidence_basis_points: 8_600,
        baseline_confidence_basis_points: 2_300,
        risk_level: NetworkAiDetectionRiskLevel::High,
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

pub(super) fn social_fn_case() -> NetworkAiDetectionFixtureCase {
    NetworkAiDetectionFixtureCase {
        detection_ref: "detect-social-fn".to_owned(),
        fixture_ref: "fixture-detect-social-fn".to_owned(),
        summary_ref: "summary-detect-social-fn".to_owned(),
        evidence_refs: vec!["evidence-detect-social-fn".to_owned()],
        analyzer_alert_refs: Vec::new(),
        expected_label: NetworkAiDetectionLabel::SocialVideo,
        predicted_label: NetworkAiDetectionLabel::BenignExpected,
        confidence_basis_points: 6_300,
        baseline_confidence_basis_points: 8_500,
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

pub(super) fn unknown_case() -> NetworkAiDetectionFixtureCase {
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

pub(super) fn evaluation_input(
    cases: Vec<NetworkAiDetectionFixtureCase>,
) -> NetworkAiDetectionEvaluationInput {
    NetworkAiDetectionEvaluationInput {
        evaluation_run_ref: " ai-detection-eval-row46 ".to_owned(),
        fixture_set_ref: "network-ai-fixtures-row46".to_owned(),
        model_card_ref: "local-ai-model-card-network-row46".to_owned(),
        model_version_ref: "local-ai-model-version-fixture-row46".to_owned(),
        baseline_ref: "local-ai-baseline-fixture-row46".to_owned(),
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
    }
}

pub(super) fn passing_case() -> NetworkAiDetectionFixtureCase {
    vpn_case()
}
