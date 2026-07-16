use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::dns::types::*;
use ocentra_network_evidence::policy::*;

#[test]
fn policy_mapping_allows_grade_a_dry_run_without_adapter_authority() {
    let mapping = map_network_evidence_grade_to_policy(input(
        NetworkEvidenceGrade::A,
        NetworkEvidencePolicyAction::Block,
    ))
    .expect_value("grade A evidence should map to a dry-run policy handoff");

    assert_eq!(mapping.mode, NetworkEvidencePolicyMode::DryRun);
    assert_eq!(mapping.mapped_action, NetworkEvidencePolicyAction::Block);
    assert_eq!(mapping.policy_decision_ref, "policy-decision-network-1");
    assert_eq!(mapping.parent_rule_ref, "parent-rule-network-1");
    assert_eq!(
        mapping.evidence_refs,
        vec![
            "network-evidence-1".to_owned(),
            "local-ai-result-ref-1".to_owned()
        ]
    );
    assert_eq!(
        mapping.local_ai_result_ref,
        Some("local-ai-result-ref-1".to_owned())
    );
    assert_eq!(
        mapping.adapter_capability_proof_ref,
        Some("adapter-proof-ref-1".to_owned())
    );
    assert!(!mapping.adapter_action_authorized);
    assert!(!mapping.enforcement_command_authorized);
}

#[test]
fn policy_mapping_routes_grade_b_block_requests_to_parent_review() {
    let mapping = map_network_evidence_grade_to_policy(input(
        NetworkEvidenceGrade::B,
        NetworkEvidencePolicyAction::Block,
    ))
    .expect_value("grade B block request should map to parent review");

    assert_eq!(mapping.mode, NetworkEvidencePolicyMode::ParentReview);
    assert_eq!(mapping.requested_action, NetworkEvidencePolicyAction::Block);
    assert_eq!(
        mapping.mapped_action,
        NetworkEvidencePolicyAction::AskParent
    );
    assert!(!mapping.adapter_action_authorized);
    assert!(!mapping.enforcement_command_authorized);
}

#[test]
fn policy_mapping_allows_grade_b_monitor_dry_run() {
    let mapping = map_network_evidence_grade_to_policy(input(
        NetworkEvidenceGrade::B,
        NetworkEvidencePolicyAction::Monitor,
    ))
    .expect_value("grade B monitor request should stay dry-run");

    assert_eq!(mapping.mode, NetworkEvidencePolicyMode::DryRun);
    assert_eq!(mapping.mapped_action, NetworkEvidencePolicyAction::Monitor);
    assert!(!mapping.adapter_action_authorized);
}

#[test]
fn policy_mapping_keeps_grade_c_and_d_non_enforcing() {
    let weak = map_network_evidence_grade_to_policy(input(
        NetworkEvidenceGrade::C,
        NetworkEvidencePolicyAction::Limit,
    ))
    .expect_value("grade C evidence should require parent review");
    assert_eq!(weak.mode, NetworkEvidencePolicyMode::ParentReview);
    assert_eq!(weak.mapped_action, NetworkEvidencePolicyAction::AskParent);
    assert!(!weak.enforcement_command_authorized);

    let unusable = map_network_evidence_grade_to_policy(input(
        NetworkEvidenceGrade::D,
        NetworkEvidencePolicyAction::Block,
    ))
    .expect_value("grade D evidence should stay observe-only");
    assert_eq!(unusable.mode, NetworkEvidencePolicyMode::ObserveOnly);
    assert_eq!(unusable.mapped_action, NetworkEvidencePolicyAction::None);
    assert!(!unusable.adapter_action_authorized);
    assert!(!unusable.enforcement_command_authorized);
}

#[test]
fn policy_mapping_rejects_missing_policy_rule_or_evidence_refs() {
    assert_eq!(
        map_network_evidence_grade_to_policy(NetworkEvidencePolicyMappingInput {
            policy_decision_ref: " ".to_owned(),
            ..input(
                NetworkEvidenceGrade::A,
                NetworkEvidencePolicyAction::Monitor
            )
        }),
        Err(NetworkEvidencePolicyMappingError::EmptyPolicyDecisionRef)
    );
    assert_eq!(
        map_network_evidence_grade_to_policy(NetworkEvidencePolicyMappingInput {
            parent_rule_ref: " ".to_owned(),
            ..input(
                NetworkEvidenceGrade::A,
                NetworkEvidencePolicyAction::Monitor
            )
        }),
        Err(NetworkEvidencePolicyMappingError::EmptyParentRuleRef)
    );
    assert_eq!(
        map_network_evidence_grade_to_policy(NetworkEvidencePolicyMappingInput {
            evidence_refs: vec![" ".to_owned()],
            ..input(
                NetworkEvidenceGrade::A,
                NetworkEvidencePolicyAction::Monitor
            )
        }),
        Err(NetworkEvidencePolicyMappingError::EmptyEvidenceRef)
    );
}

#[test]
fn policy_mapping_rejects_empty_optional_refs_when_present() {
    assert_eq!(
        map_network_evidence_grade_to_policy(NetworkEvidencePolicyMappingInput {
            local_ai_result_ref: Some(" ".to_owned()),
            ..input(
                NetworkEvidenceGrade::A,
                NetworkEvidencePolicyAction::Monitor
            )
        }),
        Err(NetworkEvidencePolicyMappingError::EmptyLocalAiResultRef)
    );
    assert_eq!(
        map_network_evidence_grade_to_policy(NetworkEvidencePolicyMappingInput {
            adapter_capability_proof_ref: Some(" ".to_owned()),
            ..input(
                NetworkEvidenceGrade::A,
                NetworkEvidencePolicyAction::Monitor
            )
        }),
        Err(NetworkEvidencePolicyMappingError::EmptyAdapterCapabilityProofRef)
    );
}

fn input(
    evidence_grade: NetworkEvidenceGrade,
    requested_action: NetworkEvidencePolicyAction,
) -> NetworkEvidencePolicyMappingInput {
    NetworkEvidencePolicyMappingInput {
        policy_decision_ref: " policy-decision-network-1 ".to_owned(),
        parent_rule_ref: "parent-rule-network-1".to_owned(),
        evidence_refs: vec![
            " network-evidence-1 ".to_owned(),
            "local-ai-result-ref-1".to_owned(),
            "network-evidence-1".to_owned(),
        ],
        local_ai_result_ref: Some("local-ai-result-ref-1".to_owned()),
        evidence_grade,
        requested_action,
        adapter_capability_proof_ref: Some("adapter-proof-ref-1".to_owned()),
    }
}
