use ocentra_policy_control_core::policy_contract_helpers::action::{
    compare_policy_action_strictness, select_stricter_policy_action, PolicyContractAction,
};
use ocentra_policy_control_core::policy_contract_helpers::app_game::{
    app_game_category_risk_policy_route_action_matches_candidate,
    app_game_category_risk_policy_route_keeps_soft_boundary,
    app_game_category_risk_policy_route_local_ai_requires_digest,
    app_game_category_risk_policy_route_manual_review_requires_manual_state,
    app_game_category_risk_policy_route_target_matches_family,
    app_game_category_risk_policy_route_uses_category_proof,
    AppGameCategoryRiskPolicyCandidateAction, AppGameCategoryRiskPolicyRouteFamily,
    AppGameCategoryRiskPolicyRouteSourceKind, AppGameCategoryRiskPolicyRoutingState,
};
use ocentra_policy_control_core::policy_contract_helpers::authority::{
    resolve_policy_authority, validate_policy_approval_resolution, AppGameCategoryRiskPolicyRoute,
    PolicyContractApprovalKind, PolicyContractApprovalOrigin, PolicyContractApprovalRequest,
    PolicyContractApprovalResolution, PolicyContractApprovalState, PolicyContractAuthorityRequest,
    PolicyContractAuthoritySource, PolicyContractAuthorityState, PolicyContractOverrideGrant,
    PolicyContractOverrideState, PolicyContractOverrideType,
};
use ocentra_policy_control_core::policy_contract_helpers::preview::{
    resolve_policy_preview_budget_boundary_state, validate_policy_preview, PolicyContractDecision,
    PolicyContractDecisionHandoffState, PolicyContractPreview,
    PolicyContractPreviewBudgetBoundaryState, PolicyContractPreviewConfirmationState,
};
use ocentra_policy_control_core::policy_contract_helpers::schedule::{
    validate_policy_schedule_boundary, PolicyContractScheduleBoundary,
    PolicyContractScheduleBoundaryState, PolicyContractScheduleClockSource,
    PolicyContractScheduleDstBoundary, PolicyContractScheduleDstResolution,
    PolicyContractScheduleDstTransition, PolicyContractScheduleExpiry,
    PolicyContractScheduleOfflineRecoveryState, PolicyContractScheduleOfflineRecoveryStatus,
    PolicyContractScheduleTimeBudgetStatus,
};
use ocentra_policy_control_core::policy_contract_helpers::screen_ai::{
    screen_ai_stricter_parent_rule_input_is_ready, screen_ai_stricter_parent_rule_proof_is_honest,
    PolicyContractScreenAiStricterParentRuleInput, PolicyContractScreenAiStricterParentRuleProof,
};

fn sample_boundary() -> PolicyContractScheduleBoundary {
    PolicyContractScheduleBoundary {
        evaluated_at: "2026-06-29T14:00:00Z".to_string(),
        local_time: "10:00".to_string(),
        state: PolicyContractScheduleBoundaryState::WithinWindow,
        dst_boundary: None,
        clock_skew: None,
        exception: None,
        expiry: None,
        time_budget: Some(sample_time_budget()),
    }
}

fn sample_time_budget() -> PolicyContractScheduleTimeBudgetStatus {
    PolicyContractScheduleTimeBudgetStatus {
        budget_window_minutes: 60,
        used_minutes: 10,
        remaining_minutes: 50,
        carryover_minutes: 0,
        grace_period_minutes: 5,
        reset_at: "2026-06-29T15:00:00Z".to_string(),
        clock_source: PolicyContractScheduleClockSource::TrustedService,
        offline_recovery: PolicyContractScheduleOfflineRecoveryStatus {
            state: PolicyContractScheduleOfflineRecoveryState::NotNeeded,
            recovered_at: None,
            recovered_offline_minutes: 0,
        },
        bonus_time_minutes: None,
        bonus_time_remaining_minutes: None,
        bonus_time_expires_at: None,
    }
}

fn sample_decision(action: PolicyContractAction) -> PolicyContractDecision {
    PolicyContractDecision {
        action,
        dry_run: true,
        enforcement_handoff_state: PolicyContractDecisionHandoffState::Disabled,
        local_ai_result_id: Some("local-ai-001".to_string()),
        evidence_reference_count: 1,
        rule_ids: vec!["rule-local-ai".to_string()],
    }
}

#[test]
fn stricter_policy_action_prefers_higher_rank() {
    assert!(
        compare_policy_action_strictness(PolicyContractAction::Block, PolicyContractAction::Warn,)
            > 0
    );
    assert_eq!(
        select_stricter_policy_action(
            PolicyContractAction::AskParent,
            PolicyContractAction::TimeLimit,
        ),
        PolicyContractAction::TimeLimit
    );
}

#[test]
fn preview_budget_boundary_and_schedule_validation_follow_rust_owned_rules() {
    let mut boundary = sample_boundary();
    boundary.state = PolicyContractScheduleBoundaryState::DstOverlap;
    boundary.dst_boundary = Some(PolicyContractScheduleDstBoundary {
        transition: PolicyContractScheduleDstTransition::FallBack,
        local_time: "01:30".to_string(),
        offset_before_minutes: -240,
        offset_after_minutes: -300,
        resolution: PolicyContractScheduleDstResolution::ManualRequired,
    });
    boundary.time_budget = Some(PolicyContractScheduleTimeBudgetStatus {
        bonus_time_minutes: Some(30),
        bonus_time_remaining_minutes: Some(10),
        bonus_time_expires_at: Some("2026-06-29T14:30:00Z".to_string()),
        ..sample_time_budget()
    });

    assert_eq!(validate_policy_schedule_boundary(&boundary), Ok(()));
    assert_eq!(
        resolve_policy_preview_budget_boundary_state(Some(&boundary)),
        PolicyContractPreviewBudgetBoundaryState::ManualRequired
    );

    let mut invalid_boundary = sample_boundary();
    invalid_boundary.expiry = Some(PolicyContractScheduleExpiry {
        expires_at: "2026-06-29T13:00:00Z".to_string(),
        expired_at: "2026-06-29T13:00:00Z".to_string(),
    });
    assert_eq!(
        validate_policy_schedule_boundary(&invalid_boundary),
        Err(
            ocentra_policy_control_core::policy_contract_helpers::PolicyContractValidationError(
                "non-expired schedule boundaries cannot be evaluated after expiry",
            )
        )
    );
}

#[test]
fn authority_and_approval_lifecycle_move_to_rust_owner() {
    let decision = resolve_policy_authority(&PolicyContractAuthorityRequest {
        source: PolicyContractAuthoritySource::LocalAiResult,
        decision: sample_decision(PolicyContractAction::Warn),
    });
    assert_eq!(decision.state, PolicyContractAuthorityState::DryRun);

    let resolution = PolicyContractApprovalResolution {
        approval: PolicyContractApprovalRequest {
            origin: PolicyContractApprovalOrigin::ChildRequest,
            kind: PolicyContractApprovalKind::BonusTime,
            child_profile_id: "child-001".to_string(),
            requested_at: "2026-06-29T13:00:00Z".to_string(),
            expires_at: "2026-06-29T15:00:00Z".to_string(),
            requested_bonus_time_minutes: Some(20),
            schedule_boundary: Some(sample_boundary()),
        },
        state: PolicyContractApprovalState::Approved,
        evaluated_at: "2026-06-29T14:00:00Z".to_string(),
        reviewed_by_actor_id: Some("parent-001".to_string()),
        reviewed_at: Some("2026-06-29T13:30:00Z".to_string()),
        audit_reference_id: Some("audit-001".to_string()),
        override_grant: Some(PolicyContractOverrideGrant {
            override_type: PolicyContractOverrideType::BonusTime,
            state: PolicyContractOverrideState::Active,
            action: PolicyContractAction::Allow,
            effective_from: "2026-06-29T13:30:00Z".to_string(),
            effective_until: "2026-06-29T14:30:00Z".to_string(),
            bonus_time_minutes: Some(20),
        }),
        replay_of_approval_id: None,
    };

    assert_eq!(validate_policy_approval_resolution(&resolution), Ok(()));

    let mut invalid_resolution = resolution;
    invalid_resolution.reviewed_by_actor_id = Some("child-001".to_string());
    assert_eq!(
        validate_policy_approval_resolution(&invalid_resolution),
        Err(
            ocentra_policy_control_core::policy_contract_helpers::PolicyContractValidationError(
                "child requests cannot self-approve or self-modify",
            )
        )
    );
}

#[test]
fn route_and_screen_ai_helpers_are_rust_owned() {
    let route = AppGameCategoryRiskPolicyRoute {
        route_family: AppGameCategoryRiskPolicyRouteFamily::GameContext,
        source_kind: AppGameCategoryRiskPolicyRouteSourceKind::LocalAi,
        target_kind: "multiplayer-game".to_string(),
        candidate_action: AppGameCategoryRiskPolicyCandidateAction::ManualReview,
        requested_action: "manual-required".to_string(),
        policy_action: PolicyContractAction::AskParent,
        routing_state: AppGameCategoryRiskPolicyRoutingState::ManualRequired,
        category_proof_kind: "category-proof".to_string(),
        category_proof_evidence_state: "active".to_string(),
        supporting_evidence_count: 1,
        has_ai_digest_ref: true,
    };
    assert!(app_game_category_risk_policy_route_target_matches_family(
        &route
    ));
    assert!(app_game_category_risk_policy_route_uses_category_proof(
        &route
    ));
    assert!(app_game_category_risk_policy_route_action_matches_candidate(&route));
    assert!(app_game_category_risk_policy_route_manual_review_requires_manual_state(&route));
    assert!(app_game_category_risk_policy_route_keeps_soft_boundary(
        &route
    ));
    assert!(app_game_category_risk_policy_route_local_ai_requires_digest(&route));

    let input = PolicyContractScreenAiStricterParentRuleInput {
        source_decision: sample_decision(PolicyContractAction::Warn),
        stricter_parent_rule_enabled: true,
        stricter_parent_rule_action: PolicyContractAction::Block,
        expected_final_action: PolicyContractAction::Block,
    };
    assert!(screen_ai_stricter_parent_rule_input_is_ready(&input));

    let proof = PolicyContractScreenAiStricterParentRuleProof {
        final_action: PolicyContractAction::Block,
        stricter_parent_rule_action: PolicyContractAction::Block,
        final_decision: PolicyContractDecision {
            action: PolicyContractAction::Block,
            dry_run: true,
            enforcement_handoff_state: PolicyContractDecisionHandoffState::Disabled,
            local_ai_result_id: Some("local-ai-001".to_string()),
            evidence_reference_count: 1,
            rule_ids: vec!["rule-parent-block".to_string(), "rule-local-ai".to_string()],
        },
        source_decision: sample_decision(PolicyContractAction::Warn),
        stricter_parent_rule_id: "rule-parent-block".to_string(),
        all_claim_boundaries_false: true,
    };
    assert!(screen_ai_stricter_parent_rule_proof_is_honest(&proof));
}

#[test]
fn rust_owned_policy_contract_helpers_sidecar_carries_generated_literal_tables() {
    let sidecar = include_str!("../../src/policy_contract_helpers_ts.contracts.txt");

    for expected in [
        "GeneratedPolicyTargetTypeValues",
        "GeneratedPermissionRequestStateValues",
        "GeneratedPolicyScheduleOfflineRecoveryValues",
        "GeneratedPolicyPreviewOriginValues",
        "GeneratedPolicyCompilerDomainValues",
        "GeneratedPolicyCompilerRuleStatusValues",
        "GeneratedPolicyCompilerCapabilityStateValues",
        "GeneratedPolicyCompilerSourceStatusValues",
        "GeneratedPolicyCompilerTargetKindValues",
        "GeneratedPolicyCompilerNoClaimLabelValues",
    ] {
        assert!(
            sidecar.contains(expected),
            "expected Rust-owned policy contract helpers sidecar to contain {expected}"
        );
    }
}

#[test]
fn preview_validation_keeps_confirmation_and_dry_run_rules_rust_owned() {
    let preview = PolicyContractPreview {
        confirmation_state: PolicyContractPreviewConfirmationState::Confirmed,
        confirmed_by_present: true,
        confirmed_at: Some("2026-06-29T14:00:00Z".to_string()),
        decision: sample_decision(PolicyContractAction::Warn),
    };
    assert_eq!(validate_policy_preview(&preview), Ok(()));

    let mut invalid_preview = preview;
    invalid_preview.decision.dry_run = false;
    assert_eq!(
        validate_policy_preview(&invalid_preview),
        Err(
            ocentra_policy_control_core::policy_contract_helpers::PolicyContractValidationError(
                "preview decisions must remain dry-run",
            )
        )
    );
}
