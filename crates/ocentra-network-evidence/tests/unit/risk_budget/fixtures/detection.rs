use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::ai_detection::*;

use super::AuditFixtureCase;

pub(super) fn detection_results(
    cases: Vec<NetworkAiDetectionFixtureCase>,
) -> Vec<NetworkAiDetectionResult> {
    evaluate_network_ai_detection_fixtures(&NetworkAiDetectionEvaluationInput {
        evaluation_run_ref: "ai-detection-eval-row48".to_owned(),
        fixture_set_ref: "network-ai-fixtures-row48".to_owned(),
        model_card_ref: "local-ai-model-card-network-row48".to_owned(),
        model_version_ref: "local-ai-model-version-fixture-row48".to_owned(),
        baseline_ref: "local-ai-baseline-fixture-row48".to_owned(),
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

pub(super) fn detection_case(
    fixture: AuditFixtureCase,
    expected_label: NetworkAiDetectionLabel,
    predicted_label: NetworkAiDetectionLabel,
    confidence_basis_points: u16,
    baseline_confidence_basis_points: u16,
    risk_level: NetworkAiDetectionRiskLevel,
    critical_alert: bool,
) -> NetworkAiDetectionFixtureCase {
    let detection_ref = match fixture {
        AuditFixtureCase::Block => "detect-risk-block",
        AuditFixtureCase::Manual => "detect-risk-manual",
        AuditFixtureCase::Safe => "detect-safe",
        AuditFixtureCase::MissingProof => "detect-safe-missing",
        AuditFixtureCase::Signature => "detect-signature-only",
        AuditFixtureCase::Unsupported => "detect-unsupported",
        AuditFixtureCase::Low => "detect-low",
    };

    let analyzer_alert_refs = if critical_alert {
        vec!["analyzer-alert-critical".to_owned()]
    } else {
        Vec::new()
    };

    let mut input_kinds = vec![
        NetworkAiDetectionInputKind::SummaryRefs,
        NetworkAiDetectionInputKind::EvidenceRefs,
        NetworkAiDetectionInputKind::FixtureLabel,
    ];
    if critical_alert {
        input_kinds.push(NetworkAiDetectionInputKind::AnalyzerAlertRefs);
    }

    NetworkAiDetectionFixtureCase {
        detection_ref: detection_ref.to_owned(),
        fixture_ref: format!("fixture-{detection_ref}"),
        summary_ref: format!("summary-{detection_ref}"),
        evidence_refs: vec![format!("evidence-{detection_ref}")],
        analyzer_alert_refs,
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
