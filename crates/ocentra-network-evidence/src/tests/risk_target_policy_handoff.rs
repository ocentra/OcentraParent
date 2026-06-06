use crate::{
    map_network_risk_target_to_policy_handoff, CategoryFreshnessState, CategoryMatchKind,
    CategorySourceCustody, DomainCategoryLookup, NetworkCategory, NetworkEvidenceGrade,
    NetworkEvidencePolicyAction, NetworkEvidencePolicyMode, NetworkRiskTargetPolicyHandoffError,
    NetworkRiskTargetPolicyHandoffInput, NetworkRiskTargetPolicyHandoffState,
};

#[test]
fn risk_target_policy_handoff_routes_signed_video_block_to_parent_review() {
    let handoff = map_network_risk_target_to_policy_handoff(input(
        fresh_video_lookup(),
        NetworkEvidencePolicyAction::Block,
    ))
    .expect("fresh high-confidence video target should map to parent review");

    assert_eq!(handoff.risk_target_ref, "risk-target-video");
    assert_eq!(handoff.normalized_domain, "watch.video.example.test");
    assert_eq!(
        handoff.matched_domain,
        Some("video.example.test".to_owned())
    );
    assert_eq!(handoff.category, NetworkCategory::Video);
    assert_eq!(handoff.evidence_grade, NetworkEvidenceGrade::B);
    assert_eq!(
        handoff.handoff_state,
        NetworkRiskTargetPolicyHandoffState::ParentReviewRequired
    );
    assert!(handoff.parent_review_required);
    assert_eq!(
        handoff.policy_mapping.mode,
        NetworkEvidencePolicyMode::ParentReview
    );
    assert_eq!(
        handoff.policy_mapping.mapped_action,
        NetworkEvidencePolicyAction::AskParent
    );
    assert_eq!(
        handoff.evidence_refs,
        vec!["evidence-domain-category-video".to_owned()]
    );
    assert!(!handoff.exact_url_available);
    assert!(!handoff.decrypted_payload_available);
    assert!(!handoff.live_adapter_mutation_executed);
    assert!(!handoff.broad_platform_support);
    assert_eq!(handoff.enforcement_commands_published, 0);
}

#[test]
fn risk_target_policy_handoff_allows_monitor_dry_run_without_adapter_authority() {
    let handoff = map_network_risk_target_to_policy_handoff(input(
        fresh_video_lookup(),
        NetworkEvidencePolicyAction::Monitor,
    ))
    .expect("fresh high-confidence monitor target should map to dry-run handoff");

    assert_eq!(
        handoff.handoff_state,
        NetworkRiskTargetPolicyHandoffState::PolicyDryRun
    );
    assert_eq!(
        handoff.policy_mapping.mode,
        NetworkEvidencePolicyMode::DryRun
    );
    assert_eq!(
        handoff.policy_mapping.mapped_action,
        NetworkEvidencePolicyAction::Monitor
    );
    assert!(!handoff.parent_review_required);
    assert!(!handoff.policy_mapping.adapter_action_authorized);
    assert!(!handoff.policy_mapping.enforcement_command_authorized);
}

#[test]
fn risk_target_policy_handoff_keeps_unknown_category_observe_only() {
    let handoff = map_network_risk_target_to_policy_handoff(input(
        unknown_lookup(),
        NetworkEvidencePolicyAction::Block,
    ))
    .expect("unknown category should remain observe-only");

    assert_eq!(handoff.category, NetworkCategory::Unknown);
    assert_eq!(handoff.evidence_grade, NetworkEvidenceGrade::D);
    assert_eq!(
        handoff.handoff_state,
        NetworkRiskTargetPolicyHandoffState::ObserveOnly
    );
    assert_eq!(
        handoff.policy_mapping.mode,
        NetworkEvidencePolicyMode::ObserveOnly
    );
    assert_eq!(
        handoff.policy_mapping.mapped_action,
        NetworkEvidencePolicyAction::None
    );
    assert_eq!(handoff.enforcement_commands_published, 0);
}

#[test]
fn risk_target_policy_handoff_rejects_network_only_content_and_authority_claims() {
    assert_eq!(
        map_network_risk_target_to_policy_handoff(NetworkRiskTargetPolicyHandoffInput {
            exact_url_claimed: true,
            ..input(fresh_video_lookup(), NetworkEvidencePolicyAction::Monitor)
        }),
        Err(NetworkRiskTargetPolicyHandoffError::ExactUrlClaimRejected)
    );
    assert_eq!(
        map_network_risk_target_to_policy_handoff(NetworkRiskTargetPolicyHandoffInput {
            decrypted_payload_claimed: true,
            ..input(fresh_video_lookup(), NetworkEvidencePolicyAction::Monitor)
        }),
        Err(NetworkRiskTargetPolicyHandoffError::DecryptedPayloadClaimRejected)
    );
    assert_eq!(
        map_network_risk_target_to_policy_handoff(NetworkRiskTargetPolicyHandoffInput {
            live_adapter_mutation_claimed: true,
            ..input(fresh_video_lookup(), NetworkEvidencePolicyAction::Monitor)
        }),
        Err(NetworkRiskTargetPolicyHandoffError::LiveAdapterMutationClaimRejected)
    );
    assert_eq!(
        map_network_risk_target_to_policy_handoff(NetworkRiskTargetPolicyHandoffInput {
            enforcement_command_claimed: true,
            ..input(fresh_video_lookup(), NetworkEvidencePolicyAction::Monitor)
        }),
        Err(NetworkRiskTargetPolicyHandoffError::EnforcementCommandClaimRejected)
    );
    assert_eq!(
        map_network_risk_target_to_policy_handoff(NetworkRiskTargetPolicyHandoffInput {
            broad_platform_support_claimed: true,
            ..input(fresh_video_lookup(), NetworkEvidencePolicyAction::Monitor)
        }),
        Err(NetworkRiskTargetPolicyHandoffError::BroadPlatformSupportClaimRejected)
    );
}

#[test]
fn risk_target_policy_handoff_rejects_category_lookup_content_upgrade() {
    let result = map_network_risk_target_to_policy_handoff(input(
        DomainCategoryLookup {
            exact_url_available: true,
            ..fresh_video_lookup()
        },
        NetworkEvidencePolicyAction::Monitor,
    ));

    assert_eq!(
        result,
        Err(NetworkRiskTargetPolicyHandoffError::CategoryLookupExactUrlRejected)
    );
}

fn input(
    category_lookup: DomainCategoryLookup,
    requested_action: NetworkEvidencePolicyAction,
) -> NetworkRiskTargetPolicyHandoffInput {
    NetworkRiskTargetPolicyHandoffInput {
        risk_target_ref: " risk-target-video ".to_owned(),
        category_lookup,
        requested_action,
        policy_decision_ref: "policy-decision-risk-target".to_owned(),
        parent_rule_ref: "parent-rule-risk-target".to_owned(),
        evidence_refs: vec![
            " evidence-domain-category-video ".to_owned(),
            "evidence-domain-category-video".to_owned(),
        ],
        local_ai_result_ref: Some("local-ai-risk-target-review".to_owned()),
        adapter_capability_proof_ref: Some("adapter-capability-risk-target".to_owned()),
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        live_adapter_mutation_claimed: false,
        enforcement_command_claimed: false,
        broad_platform_support_claimed: false,
    }
}

fn fresh_video_lookup() -> DomainCategoryLookup {
    DomainCategoryLookup {
        normalized_domain: "watch.video.example.test".to_owned(),
        matched_domain: Some("video.example.test".to_owned()),
        match_kind: CategoryMatchKind::RegistrableDomain,
        category: NetworkCategory::Video,
        source_id: Some("signed-category-source".to_owned()),
        source_custody: Some(CategorySourceCustody::SignedLocalSnapshot),
        freshness: CategoryFreshnessState::Fresh {
            age_seconds: 60,
            max_age_seconds: 3_600,
        },
        confidence_percent: Some(96),
        evidence_grade: NetworkEvidenceGrade::C,
        exact_url_available: false,
        decrypted_payload_available: false,
    }
}

fn unknown_lookup() -> DomainCategoryLookup {
    DomainCategoryLookup {
        normalized_domain: "unknown.example.test".to_owned(),
        matched_domain: None,
        match_kind: CategoryMatchKind::NoMatch,
        category: NetworkCategory::Unknown,
        source_id: None,
        source_custody: None,
        freshness: CategoryFreshnessState::Unknown,
        confidence_percent: None,
        evidence_grade: NetworkEvidenceGrade::D,
        exact_url_available: false,
        decrypted_payload_available: false,
    }
}
