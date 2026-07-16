use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::risk_budget::*;

mod fixtures;

use self::fixtures::{
    benign_audit_report, default_policy, high_risk_audit_report, low_risk_signal, prior_event,
    risk_signal, threshold_input, AuditFixtureCase, PriorEventRefCase, SignalRefCase,
};

#[test]
fn risk_budget_threshold_maps_profile_prior_events_and_adapter_proof_to_block() {
    let evaluation = evaluate_network_risk_budget_threshold(threshold_input(
        vec![risk_signal(
            SignalRefCase::Block,
            high_risk_audit_report(AuditFixtureCase::Block),
            NetworkRiskBudgetEvidenceTier::AdapterProofReady,
            80,
            0,
        )],
        vec![prior_event(PriorEventRefCase::Block, 20, true, true)],
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
            SignalRefCase::Manual,
            high_risk_audit_report(AuditFixtureCase::Manual),
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
                SignalRefCase::Safe,
                benign_audit_report(AuditFixtureCase::Safe),
                NetworkRiskBudgetEvidenceTier::AiAuditWithCitations,
                50,
                0,
            )
        }],
        vec![prior_event(PriorEventRefCase::Safe, 20, true, true)],
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
                    SignalRefCase::MissingProof,
                    benign_audit_report(AuditFixtureCase::MissingProof),
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
                SignalRefCase::Signature,
                high_risk_audit_report(AuditFixtureCase::Signature),
                NetworkRiskBudgetEvidenceTier::AdapterProofReady,
                90,
                0,
            )
        }],
        vec![prior_event(
            PriorEventRefCase::OutsideWindow,
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

    let mut audit_report = high_risk_audit_report(AuditFixtureCase::Unsupported);
    audit_report.raw_pcap_available = true;
    assert_eq!(
        evaluate_network_risk_budget_threshold(threshold_input(
            vec![risk_signal(
                SignalRefCase::Unsupported,
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
