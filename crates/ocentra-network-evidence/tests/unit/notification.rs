use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::dns::types::*;
use ocentra_network_evidence::notification::*;
use ocentra_network_evidence::policy::*;

#[test]
fn notification_candidate_maps_grade_a_policy_to_urgent_candidate_only() {
    let candidate = map_network_parent_notification_candidate(&candidate_input(policy_mapping(
        NetworkEvidenceGrade::A,
        NetworkEvidencePolicyAction::Block,
    )))
    .expect_value("grade A block dry-run should map to urgent parent candidate");

    assert_eq!(
        candidate.notification_candidate_ref,
        "network-notification-1"
    );
    assert_eq!(candidate.policy_decision_ref, "policy-decision-network-1");
    assert_eq!(candidate.parent_rule_ref, "parent-rule-network-1");
    assert_eq!(candidate.evidence_refs, vec!["network-evidence-1"]);
    assert_eq!(
        candidate.local_ai_result_ref,
        Some("local-ai-result-ref-1".to_owned())
    );
    assert_eq!(candidate.policy_mode, NetworkEvidencePolicyMode::DryRun);
    assert_eq!(candidate.policy_action, NetworkEvidencePolicyAction::Block);
    assert_eq!(
        candidate.severity,
        NetworkParentNotificationSeverity::Urgent
    );
    assert_eq!(
        candidate.delivery_state,
        NetworkParentNotificationDeliveryState::CandidateOnly
    );
    assert!(!candidate.provider_delivery_authorized);
    assert!(!candidate.sensitive_payload_available);
    assert!(!candidate.adapter_action_authorized);
    assert!(!candidate.enforcement_command_authorized);
}

#[test]
fn notification_candidate_maps_parent_review_and_observe_only_states() {
    let review = map_network_parent_notification_candidate(&candidate_input(policy_mapping(
        NetworkEvidenceGrade::B,
        NetworkEvidencePolicyAction::Block,
    )))
    .expect_value("grade B block should map to parent review candidate");
    assert_eq!(review.policy_mode, NetworkEvidencePolicyMode::ParentReview);
    assert_eq!(review.policy_action, NetworkEvidencePolicyAction::AskParent);
    assert_eq!(review.severity, NetworkParentNotificationSeverity::Review);
    assert!(!review.provider_delivery_authorized);

    let observe = map_network_parent_notification_candidate(&candidate_input(policy_mapping(
        NetworkEvidenceGrade::D,
        NetworkEvidencePolicyAction::Block,
    )))
    .expect_value("grade D should map to observe-only notification candidate");
    assert_eq!(observe.policy_mode, NetworkEvidencePolicyMode::ObserveOnly);
    assert_eq!(observe.policy_action, NetworkEvidencePolicyAction::None);
    assert_eq!(observe.severity, NetworkParentNotificationSeverity::Info);
}

#[test]
fn notification_candidate_rejects_provider_delivery_or_sensitive_payload_claims() {
    assert_eq!(
        map_network_parent_notification_candidate(&NetworkParentNotificationCandidateInput {
            provider_delivery_available: true,
            ..candidate_input(policy_mapping(
                NetworkEvidenceGrade::A,
                NetworkEvidencePolicyAction::Monitor
            ))
        }),
        Err(NetworkParentNotificationCandidateError::ProviderDeliveryClaimRejected)
    );
    assert_eq!(
        map_network_parent_notification_candidate(&NetworkParentNotificationCandidateInput {
            sensitive_payload_available: true,
            ..candidate_input(policy_mapping(
                NetworkEvidenceGrade::A,
                NetworkEvidencePolicyAction::Monitor
            ))
        }),
        Err(NetworkParentNotificationCandidateError::SensitivePayloadRejected)
    );
}

#[test]
fn notification_candidate_rejects_adapter_or_enforcement_authority() {
    let mut adapter = policy_mapping(
        NetworkEvidenceGrade::A,
        NetworkEvidencePolicyAction::Monitor,
    );
    adapter.adapter_action_authorized = true;
    assert_eq!(
        map_network_parent_notification_candidate(&candidate_input(adapter)),
        Err(NetworkParentNotificationCandidateError::AdapterAuthorityRejected)
    );

    let mut enforcement = policy_mapping(
        NetworkEvidenceGrade::A,
        NetworkEvidencePolicyAction::Monitor,
    );
    enforcement.enforcement_command_authorized = true;
    assert_eq!(
        map_network_parent_notification_candidate(&candidate_input(enforcement)),
        Err(NetworkParentNotificationCandidateError::EnforcementCommandRejected)
    );
}

#[test]
fn notification_candidate_rejects_empty_refs() {
    assert_eq!(
        map_network_parent_notification_candidate(&NetworkParentNotificationCandidateInput {
            notification_candidate_ref: " ".to_owned(),
            ..candidate_input(policy_mapping(
                NetworkEvidenceGrade::A,
                NetworkEvidencePolicyAction::Monitor
            ))
        }),
        Err(NetworkParentNotificationCandidateError::EmptyNotificationCandidateRef)
    );

    let mut missing_policy = policy_mapping(
        NetworkEvidenceGrade::A,
        NetworkEvidencePolicyAction::Monitor,
    );
    missing_policy.policy_decision_ref = " ".to_owned();
    assert_eq!(
        map_network_parent_notification_candidate(&candidate_input(missing_policy)),
        Err(NetworkParentNotificationCandidateError::EmptyPolicyDecisionRef)
    );

    let mut missing_rule = policy_mapping(
        NetworkEvidenceGrade::A,
        NetworkEvidencePolicyAction::Monitor,
    );
    missing_rule.parent_rule_ref = " ".to_owned();
    assert_eq!(
        map_network_parent_notification_candidate(&candidate_input(missing_rule)),
        Err(NetworkParentNotificationCandidateError::EmptyParentRuleRef)
    );

    let mut missing_evidence = policy_mapping(
        NetworkEvidenceGrade::A,
        NetworkEvidencePolicyAction::Monitor,
    );
    missing_evidence.evidence_refs = vec![" ".to_owned()];
    assert_eq!(
        map_network_parent_notification_candidate(&candidate_input(missing_evidence)),
        Err(NetworkParentNotificationCandidateError::EmptyEvidenceRef)
    );
}

fn candidate_input(
    mapping: NetworkEvidencePolicyMapping,
) -> NetworkParentNotificationCandidateInput {
    NetworkParentNotificationCandidateInput {
        notification_candidate_ref: " network-notification-1 ".to_owned(),
        mapping,
        provider_delivery_available: false,
        sensitive_payload_available: false,
    }
}

fn policy_mapping(
    evidence_grade: NetworkEvidenceGrade,
    requested_action: NetworkEvidencePolicyAction,
) -> NetworkEvidencePolicyMapping {
    map_network_evidence_grade_to_policy(NetworkEvidencePolicyMappingInput {
        policy_decision_ref: "policy-decision-network-1".to_owned(),
        parent_rule_ref: "parent-rule-network-1".to_owned(),
        evidence_refs: vec!["network-evidence-1".to_owned()],
        local_ai_result_ref: Some("local-ai-result-ref-1".to_owned()),
        evidence_grade,
        requested_action,
        adapter_capability_proof_ref: None,
    })
    .expect_value("policy mapping test input should be valid")
}
