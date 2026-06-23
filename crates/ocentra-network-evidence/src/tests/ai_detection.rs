use crate::{
    evaluate_network_ai_detection_fixtures, NetworkAiDetectionDriftState,
    NetworkAiDetectionEvaluationError, NetworkAiDetectionEvaluationInput,
    NetworkAiDetectionEvaluationState, NetworkAiDetectionFixtureCase, NetworkAiDetectionInputKind,
    NetworkAiDetectionLabel, NetworkAiDetectionPrecisionState, NetworkAiDetectionRecallState,
    NetworkAiDetectionRiskLevel, NetworkAiDetectionUncertaintyCode,
};
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn ai_detection_fixture_evaluation_meets_precision_recall_and_drift_thresholds() {
    let proof = evaluate_network_ai_detection_fixtures(&evaluation_input(vec![
        fixture_case(
            "detect-vpn-1",
            NetworkAiDetectionLabel::VpnProxyTunnel,
            NetworkAiDetectionLabel::VpnProxyTunnel,
            8_700,
            8_500,
            NetworkAiDetectionRiskLevel::High,
            vec![],
        ),
        fixture_case(
            "detect-signature-1",
            NetworkAiDetectionLabel::SignatureThreat,
            NetworkAiDetectionLabel::SignatureThreat,
            9_100,
            8_900,
            NetworkAiDetectionRiskLevel::Critical,
            vec!["analyzer-alert-critical"],
        ),
        fixture_case(
            "detect-update-1",
            NetworkAiDetectionLabel::BenignExpected,
            NetworkAiDetectionLabel::BenignExpected,
            8_200,
            8_100,
            NetworkAiDetectionRiskLevel::Low,
            vec![],
        ),
    ]))
    .expect_value("fixture gate should pass");

    assert_eq!(proof.fixture_count, 3);
    assert_eq!(proof.true_positive_count, 2);
    assert_eq!(proof.false_positive_count, 0);
    assert_eq!(proof.false_negative_count, 0);
    assert_eq!(proof.true_negative_count, 1);
    assert_eq!(proof.precision_basis_points, Some(10_000));
    assert_eq!(proof.recall_basis_points, Some(10_000));
    assert_eq!(proof.accuracy_basis_points, 10_000);
    assert_eq!(proof.average_confidence_drift_basis_points, 167);
    assert_eq!(
        proof.precision_state,
        NetworkAiDetectionPrecisionState::MeetsThreshold
    );
    assert_eq!(
        proof.recall_state,
        NetworkAiDetectionRecallState::MeetsThreshold
    );
    assert_eq!(
        proof.drift_state,
        NetworkAiDetectionDriftState::WithinTolerance
    );
    assert_eq!(
        proof.evaluation_state,
        NetworkAiDetectionEvaluationState::MeetsFixtureGate
    );
    assert!(!proof.model_executed);
    assert!(!proof.remote_ai_used);
    assert!(!proof.policy_authority);
    assert!(!proof.adapter_authority);
    assert_eq!(proof.enforcement_commands_published, 0);
    assert_eq!(
        proof.results[1].analyzer_alert_refs,
        vec!["analyzer-alert-critical"]
    );
    assert!(!proof.results[1].raw_pcap_available);
    assert!(!proof.results[1].exact_url_available);
    assert!(!proof.results[1].decrypted_payload_available);
    assert!(!proof.results[1].enforcement_command_published);
}

#[test]
fn ai_detection_fixture_evaluation_flags_precision_and_drift_regressions() {
    let mut input = evaluation_input(vec![
        fixture_case(
            "detect-vpn-fp",
            NetworkAiDetectionLabel::BenignExpected,
            NetworkAiDetectionLabel::VpnProxyTunnel,
            8_600,
            2_100,
            NetworkAiDetectionRiskLevel::High,
            vec![],
        ),
        fixture_case(
            "detect-signature-tp",
            NetworkAiDetectionLabel::SignatureThreat,
            NetworkAiDetectionLabel::SignatureThreat,
            8_500,
            8_500,
            NetworkAiDetectionRiskLevel::Critical,
            vec!["signature-alert-ref"],
        ),
        fixture_case(
            "detect-social-fn",
            NetworkAiDetectionLabel::SocialVideo,
            NetworkAiDetectionLabel::BenignExpected,
            6_300,
            8_500,
            NetworkAiDetectionRiskLevel::Low,
            vec![],
        ),
    ]);
    input.minimum_precision_basis_points = 8_000;
    input.minimum_recall_basis_points = 8_000;
    input.maximum_average_drift_basis_points = 1_000;

    let proof = evaluate_network_ai_detection_fixtures(&input)
        .expect_value("regression should be measured");

    assert_eq!(proof.true_positive_count, 1);
    assert_eq!(proof.false_positive_count, 1);
    assert_eq!(proof.false_negative_count, 1);
    assert_eq!(proof.precision_basis_points, Some(5_000));
    assert_eq!(proof.recall_basis_points, Some(5_000));
    assert_eq!(proof.average_confidence_drift_basis_points, 2_900);
    assert_eq!(
        proof.precision_state,
        NetworkAiDetectionPrecisionState::BelowThreshold
    );
    assert_eq!(
        proof.recall_state,
        NetworkAiDetectionRecallState::BelowThreshold
    );
    assert_eq!(
        proof.drift_state,
        NetworkAiDetectionDriftState::ExceededTolerance
    );
    assert_eq!(
        proof.evaluation_state,
        NetworkAiDetectionEvaluationState::BelowQualityAndDriftExceeded
    );
    assert!(proof.results[0]
        .uncertainty_codes
        .contains(&NetworkAiDetectionUncertaintyCode::FalsePositiveFixture));
    assert!(proof.results[2]
        .uncertainty_codes
        .contains(&NetworkAiDetectionUncertaintyCode::FalseNegativeFixture));
    assert!(!proof.policy_authority);
    assert!(!proof.adapter_authority);
}

#[test]
fn ai_detection_fixture_evaluation_preserves_unknown_and_low_confidence_states() {
    let proof = evaluate_network_ai_detection_fixtures(&evaluation_input(vec![fixture_case(
        "detect-unknown-1",
        NetworkAiDetectionLabel::UnknownHighVolume,
        NetworkAiDetectionLabel::Unknown,
        4_200,
        4_100,
        NetworkAiDetectionRiskLevel::Unknown,
        vec![],
    )]))
    .expect_value("unknown prediction should remain explicit");
    let result = &proof.results[0];

    assert_eq!(proof.precision_basis_points, None);
    assert_eq!(
        proof.precision_state,
        NetworkAiDetectionPrecisionState::NoPositivePredictions
    );
    assert_eq!(proof.recall_basis_points, Some(0));
    assert_eq!(
        proof.recall_state,
        NetworkAiDetectionRecallState::BelowThreshold
    );
    assert!(result.false_negative);
    assert!(!result.predicted_positive);
    assert!(result
        .uncertainty_codes
        .contains(&NetworkAiDetectionUncertaintyCode::UnknownPrediction));
    assert!(result
        .uncertainty_codes
        .contains(&NetworkAiDetectionUncertaintyCode::LowConfidence));
}

#[test]
fn ai_detection_fixture_evaluation_rejects_raw_content_and_authority_claims() {
    assert_eq!(
        evaluate_network_ai_detection_fixtures(&NetworkAiDetectionEvaluationInput {
            model_execution_claimed: true,
            ..evaluation_input(vec![passing_case()])
        }),
        Err(NetworkAiDetectionEvaluationError::ModelExecutionClaimRejected)
    );
    assert_eq!(
        evaluate_network_ai_detection_fixtures(&NetworkAiDetectionEvaluationInput {
            raw_pcap_input_claimed: true,
            ..evaluation_input(vec![passing_case()])
        }),
        Err(NetworkAiDetectionEvaluationError::RawPcapInputRejected)
    );
    assert_eq!(
        evaluate_network_ai_detection_fixtures(&NetworkAiDetectionEvaluationInput {
            policy_authority_claimed: true,
            ..evaluation_input(vec![passing_case()])
        }),
        Err(NetworkAiDetectionEvaluationError::PolicyAuthorityClaimRejected)
    );
    assert_eq!(
        evaluate_network_ai_detection_fixtures(&NetworkAiDetectionEvaluationInput {
            enforcement_command_claimed: true,
            ..evaluation_input(vec![passing_case()])
        }),
        Err(NetworkAiDetectionEvaluationError::EnforcementCommandClaimRejected)
    );
    assert_eq!(
        evaluate_network_ai_detection_fixtures(&evaluation_input(vec![
            NetworkAiDetectionFixtureCase {
                page_content_claimed: true,
                ..passing_case()
            }
        ])),
        Err(NetworkAiDetectionEvaluationError::PageContentClaimRejected)
    );
    assert_eq!(
        evaluate_network_ai_detection_fixtures(&evaluation_input(vec![
            NetworkAiDetectionFixtureCase {
                exact_url_claimed: true,
                ..passing_case()
            }
        ])),
        Err(NetworkAiDetectionEvaluationError::ExactUrlClaimRejected)
    );
}

#[test]
fn ai_detection_fixture_evaluation_rejects_invalid_refs_and_duplicate_fixtures() {
    assert_eq!(
        evaluate_network_ai_detection_fixtures(&NetworkAiDetectionEvaluationInput {
            evaluation_run_ref: " ".to_owned(),
            ..evaluation_input(vec![passing_case()])
        }),
        Err(NetworkAiDetectionEvaluationError::EmptyEvaluationRunRef)
    );
    assert_eq!(
        evaluate_network_ai_detection_fixtures(&evaluation_input(vec![])),
        Err(NetworkAiDetectionEvaluationError::EmptyFixtureCases)
    );
    assert_eq!(
        evaluate_network_ai_detection_fixtures(&evaluation_input(vec![
            NetworkAiDetectionFixtureCase {
                evidence_refs: vec![],
                ..passing_case()
            }
        ])),
        Err(NetworkAiDetectionEvaluationError::EmptyEvidenceRefs)
    );
    assert_eq!(
        evaluate_network_ai_detection_fixtures(&evaluation_input(vec![
            NetworkAiDetectionFixtureCase {
                input_kinds: vec![],
                ..passing_case()
            }
        ])),
        Err(NetworkAiDetectionEvaluationError::EmptyInputKinds)
    );
    assert_eq!(
        evaluate_network_ai_detection_fixtures(&evaluation_input(vec![
            passing_case(),
            NetworkAiDetectionFixtureCase {
                fixture_ref: "fixture-duplicate".to_owned(),
                ..passing_case()
            },
        ])),
        Err(NetworkAiDetectionEvaluationError::DuplicateDetectionRef)
    );
}

fn evaluation_input(
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

fn passing_case() -> NetworkAiDetectionFixtureCase {
    fixture_case(
        "detect-vpn-1",
        NetworkAiDetectionLabel::VpnProxyTunnel,
        NetworkAiDetectionLabel::VpnProxyTunnel,
        8_700,
        8_500,
        NetworkAiDetectionRiskLevel::High,
        vec![],
    )
}

fn fixture_case(
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
        evidence_refs: vec![
            format!("evidence-{detection_ref}"),
            format!("evidence-{detection_ref}"),
        ],
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
