use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::ai_audit::*;
use ocentra_network_evidence::ai_detection::*;
use ocentra_network_evidence::risk_budget::*;

#[test]
fn risk_budget_threshold_maps_profile_prior_events_and_adapter_proof_to_block() {
    let evaluation = evaluate_network_risk_budget_threshold(threshold_input(
        vec![risk_signal(
            "network-risk-signal-1",
            high_risk_audit_report("network-ai-audit-row48-block", "detect-risk-block"),
            NetworkRiskBudgetEvidenceTier::AdapterProofReady,
            80,
            0,
        )],
        vec![prior_event("prior-network-risk-1", 20, true, true)],
        default_policy(),
        NetworkRiskBudgetAdapterProofState::Ready,
    ))
    .expect_value("adapter-ready high score should map to a block recommendation");

    assert_eq!(
        evaluation.risk_budget_state,
        NetworkRiskBudgetState::BlockThreshold
    );
    assert_eq!(
        evaluation.intervention_state,
        NetworkInterventionState::Block
    );
    assert_eq!(evaluation.total_risk_points, 120);
    assert_eq!(evaluation.age_profile_points, 20);
    assert_eq!(evaluation.active_signal_points, 80);
    assert_eq!(evaluation.prior_event_points, 20);
    assert_eq!(evaluation.triggered_threshold_points, 100);
    assert_eq!(evaluation.cited_signal_refs, vec!["network-risk-signal-1"]);
    assert_eq!(
        evaluation.cited_audit_refs,
        vec!["network-ai-audit-row48-block"]
    );
    assert_eq!(
        evaluation.cited_evidence_refs,
        vec!["evidence-detect-risk-block"]
    );
    assert_eq!(
        evaluation.cited_prior_event_refs,
        vec!["prior-network-risk-1"]
    );
    assert!(evaluation.advisory_only);
    assert!(!evaluation.policy_authority);
    assert!(!evaluation.adapter_authority);
    assert_eq!(evaluation.enforcement_commands_published, 0);
}

#[test]
fn risk_budget_threshold_requires_manual_review_without_adapter_proof() {
    let evaluation = evaluate_network_risk_budget_threshold(threshold_input(
        vec![risk_signal(
            "network-risk-signal-manual",
            high_risk_audit_report("network-ai-audit-row48-manual", "detect-risk-manual"),
            NetworkRiskBudgetEvidenceTier::AdapterProofReady,
            90,
            0,
        )],
        vec![],
        default_policy(),
        NetworkRiskBudgetAdapterProofState::Missing,
    ))
    .expect_value("missing adapter proof should stay manual-required");

    assert_eq!(
        evaluation.risk_budget_state,
        NetworkRiskBudgetState::BlockThreshold
    );
    assert_eq!(
        evaluation.intervention_state,
        NetworkInterventionState::ManualRequired
    );
    assert_eq!(
        evaluation.adapter_proof_state,
        NetworkRiskBudgetAdapterProofState::Missing
    );
    assert_eq!(evaluation.enforcement_commands_published, 0);
}

#[test]
fn risk_budget_threshold_applies_safe_behavior_credit_only_with_policy_proof() {
    let evaluation = evaluate_network_risk_budget_threshold(threshold_input(
        vec![NetworkRiskBudgetSignal {
            safe_behavior_credit_points: 25,
            known_safe: true,
            expected_activity: true,
            ..risk_signal(
                "network-risk-signal-safe",
                benign_audit_report("network-ai-audit-row48-safe", "detect-safe"),
                NetworkRiskBudgetEvidenceTier::AiAuditWithCitations,
                50,
                0,
            )
        }],
        vec![prior_event("prior-network-risk-safe", 20, true, true)],
        default_policy(),
        NetworkRiskBudgetAdapterProofState::NotNeeded,
    ))
    .expect_value("safe behavior credit with policy proof should reduce pressure");

    assert_eq!(evaluation.total_risk_points, 15);
    assert_eq!(evaluation.safe_behavior_credit_applied_points, 25);
    assert_eq!(
        evaluation.risk_budget_state,
        NetworkRiskBudgetState::WithinBudget
    );
    assert_eq!(
        evaluation.intervention_state,
        NetworkInterventionState::Ignore
    );

    assert_eq!(
        evaluate_network_risk_budget_threshold(threshold_input(
            vec![NetworkRiskBudgetSignal {
                safe_behavior_credit_points: 25,
                known_safe: true,
                expected_activity: true,
                ..risk_signal(
                    "network-risk-signal-missing-proof",
                    benign_audit_report("network-ai-audit-row48-missing", "detect-safe-missing"),
                    NetworkRiskBudgetEvidenceTier::AiAuditWithCitations,
                    50,
                    0,
                )
            }],
            vec![],
            NetworkRiskBudgetHouseholdPolicy {
                safe_behavior_credit_expiry_ref: None,
                ..default_policy()
            },
            NetworkRiskBudgetAdapterProofState::NotNeeded,
        )),
        Err(NetworkRiskBudgetThresholdError::SafeBehaviorCreditRequiresPolicyProof)
    );
}

#[test]
fn risk_budget_threshold_keeps_signature_only_hits_manual_required() {
    let evaluation = evaluate_network_risk_budget_threshold(threshold_input(
        vec![NetworkRiskBudgetSignal {
            signature_only: true,
            ..risk_signal(
                "network-risk-signal-signature",
                high_risk_audit_report("network-ai-audit-row48-signature", "detect-signature-only"),
                NetworkRiskBudgetEvidenceTier::AdapterProofReady,
                90,
                0,
            )
        }],
        vec![prior_event(
            "prior-network-risk-outside-window",
            70,
            false,
            true,
        )],
        default_policy(),
        NetworkRiskBudgetAdapterProofState::Ready,
    ))
    .expect_value("signature-only hit should not auto-map to control");

    assert_eq!(evaluation.prior_event_points, 0);
    assert!(evaluation.cited_prior_event_refs.is_empty());
    assert_eq!(
        evaluation.risk_budget_state,
        NetworkRiskBudgetState::BlockThreshold
    );
    assert_eq!(
        evaluation.intervention_state,
        NetworkInterventionState::ManualRequired
    );
    assert_eq!(evaluation.enforcement_commands_published, 0);
}

#[test]
fn risk_budget_threshold_rejects_unsupported_content_authority_and_grants() {
    assert_eq!(
        evaluate_network_risk_budget_threshold(NetworkRiskBudgetThresholdInput {
            exact_url_claimed: true,
            ..threshold_input(
                vec![low_risk_signal()],
                vec![],
                default_policy(),
                NetworkRiskBudgetAdapterProofState::NotNeeded,
            )
        }),
        Err(NetworkRiskBudgetThresholdError::ExactUrlClaimRejected)
    );
    assert_eq!(
        evaluate_network_risk_budget_threshold(NetworkRiskBudgetThresholdInput {
            extra_privilege_grant_claimed: true,
            ..threshold_input(
                vec![low_risk_signal()],
                vec![],
                default_policy(),
                NetworkRiskBudgetAdapterProofState::NotNeeded,
            )
        }),
        Err(NetworkRiskBudgetThresholdError::ExtraPrivilegeGrantRejected)
    );

    let mut audit_report =
        high_risk_audit_report("network-ai-audit-row48-unsupported", "detect-unsupported");
    audit_report.raw_pcap_available = true;
    assert_eq!(
        evaluate_network_risk_budget_threshold(threshold_input(
            vec![risk_signal(
                "network-risk-signal-unsupported",
                audit_report,
                NetworkRiskBudgetEvidenceTier::AiAuditWithCitations,
                40,
                0,
            )],
            vec![],
            default_policy(),
            NetworkRiskBudgetAdapterProofState::NotNeeded,
        )),
        Err(NetworkRiskBudgetThresholdError::AuditReportUnsupportedClaim)
    );
}

fn threshold_input(
    signals: Vec<NetworkRiskBudgetSignal>,
    prior_events: Vec<NetworkRiskBudgetPriorEvent>,
    household_policy: NetworkRiskBudgetHouseholdPolicy,
    adapter_proof_state: NetworkRiskBudgetAdapterProofState,
) -> NetworkRiskBudgetThresholdInput {
    NetworkRiskBudgetThresholdInput {
        evaluation_ref: "network-risk-budget-row48".to_owned(),
        child_profile_ref: "child-profile-middle-school".to_owned(),
        risk_budget_ref: "household-network-risk-budget".to_owned(),
        cascade_ref: "network-cascade-row48".to_owned(),
        age_band: NetworkRiskBudgetAgeBand::UnderTwelve,
        profile_risk_weight_points: 5,
        thresholds: NetworkRiskBudgetThresholds {
            monitor_points: 20,
            ask_parent_points: 40,
            warn_child_points: 60,
            limit_points: 80,
            block_points: 100,
            max_points: 120,
        },
        household_policy,
        signals,
        prior_events,
        adapter_proof_state,
        raw_pcap_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        exact_url_claimed: false,
        private_message_claimed: false,
        search_query_claimed: false,
        policy_authority_claimed: false,
        adapter_authority_claimed: false,
        enforcement_command_claimed: false,
        extra_privilege_grant_claimed: false,
        allowance_grant_claimed: false,
        time_grant_claimed: false,
    }
}

fn default_policy() -> NetworkRiskBudgetHouseholdPolicy {
    NetworkRiskBudgetHouseholdPolicy {
        household_policy_ref: "household-policy-network-risk".to_owned(),
        parent_rule_refs: vec!["parent-rule-network-review".to_owned()],
        child_warning_allowed: true,
        limit_policy_allowed: true,
        block_policy_allowed: true,
        strict_block_policy_enabled: true,
        safe_behavior_credit_cap_points: 30,
        safe_behavior_credit_expiry_ref: Some("safe-credit-expiry-row48".to_owned()),
        safe_behavior_audit_reason_ref: Some("safe-credit-audit-reason-row48".to_owned()),
        safe_behavior_ui_explanation_ref: Some("safe-credit-ui-explanation-row48".to_owned()),
    }
}

fn risk_signal<S>(
    signal_ref: S,
    audit_report: NetworkAiAuditReport,
    evidence_tier: NetworkRiskBudgetEvidenceTier,
    base_risk_points: u16,
    safe_behavior_credit_points: u16,
) -> NetworkRiskBudgetSignal
where
    S: Into<String>,
{
    NetworkRiskBudgetSignal {
        signal_ref: signal_ref.into(),
        audit_report,
        evidence_tier,
        base_risk_points,
        safe_behavior_credit_points,
        known_safe: false,
        expected_activity: false,
        signature_only: false,
    }
}

fn low_risk_signal() -> NetworkRiskBudgetSignal {
    risk_signal(
        "network-risk-signal-low",
        benign_audit_report("network-ai-audit-row48-low", "detect-low"),
        NetworkRiskBudgetEvidenceTier::AiAuditWithCitations,
        10,
        0,
    )
}

fn prior_event<S>(
    event_ref: S,
    risk_points: u16,
    within_window: bool,
    same_household_rule: bool,
) -> NetworkRiskBudgetPriorEvent
where
    S: Into<String>,
{
    NetworkRiskBudgetPriorEvent {
        event_ref: event_ref.into(),
        risk_points,
        within_window,
        same_household_rule,
    }
}

fn high_risk_audit_report(audit_report_ref: &str, detection_ref: &str) -> NetworkAiAuditReport {
    audit_report(
        audit_report_ref,
        detection_case(
            detection_ref,
            NetworkAiDetectionLabel::SignatureThreat,
            NetworkAiDetectionLabel::SignatureThreat,
            9_100,
            8_900,
            NetworkAiDetectionRiskLevel::Critical,
            vec!["analyzer-alert-critical"],
        ),
    )
}

fn benign_audit_report(audit_report_ref: &str, detection_ref: &str) -> NetworkAiAuditReport {
    audit_report(
        audit_report_ref,
        detection_case(
            detection_ref,
            NetworkAiDetectionLabel::BenignExpected,
            NetworkAiDetectionLabel::BenignExpected,
            8_200,
            8_100,
            NetworkAiDetectionRiskLevel::Low,
            vec![],
        ),
    )
}

fn audit_report(
    audit_report_ref: &str,
    detection_case: NetworkAiDetectionFixtureCase,
) -> NetworkAiAuditReport {
    build_network_ai_audit_report(&NetworkAiAuditReportInput {
        audit_report_ref: audit_report_ref.to_owned(),
        narrative_template_ref: "network-ai-audit-template-row48".to_owned(),
        model_version_ref: "local-ai-model-version-fixture-row48".to_owned(),
        policy_context_ref: "network-policy-context-row48".to_owned(),
        detection_results: detection_results(vec![detection_case]),
        parent_rule_refs: vec!["parent-rule-network-review".to_owned()],
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
    })
    .expect_value("AI audit report should build from detection fixture")
}

fn detection_results(cases: Vec<NetworkAiDetectionFixtureCase>) -> Vec<NetworkAiDetectionResult> {
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
