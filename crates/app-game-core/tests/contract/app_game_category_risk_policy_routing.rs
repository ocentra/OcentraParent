use ocentra_app_game_core::app_game_category_risk_policy_routing::route_app_game_category_risk_candidate;
use ocentra_app_game_core::app_game_category_risk_policy_routing::types::{
    AppGameCategoryProofState, AppGameCategoryRiskAdapterDispatchState,
    AppGameCategoryRiskCandidate, AppGameCategoryRiskCandidateKind,
    AppGameCategoryRiskCandidateSource, AppGameCategoryRiskRouteReason,
    AppGameCategoryRiskRouteRequest, AppGameCategoryRiskRouteState,
};
use ocentra_app_game_core::app_game_policy_target_compiler::compile_app_game_policy_target;
use ocentra_app_game_core::app_game_policy_target_compiler::references::{
    AppGamePolicyAuditRef, AppGamePolicyCapabilityRef, AppGamePolicyCompileRequestId,
    AppGamePolicyCompiledDecisionId, AppGamePolicyDeviceId, AppGamePolicyEvidenceRef,
    AppGamePolicyLocalUserRef, AppGamePolicyRuleRef, AppGamePolicyTargetRef,
};
use ocentra_app_game_core::app_game_policy_target_compiler::types::{
    AppGamePolicyCompilerCapabilityEvidence, AppGamePolicyCompilerCapabilityState,
    AppGamePolicyCompilerContext, AppGamePolicyCompilerOutcomeState,
    AppGamePolicyCompilerRequestedAction, AppGamePolicyEnforcementHandoffState,
    AppGamePolicyTargetKind,
};
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn category_risk_policy_routing_maps_every_supported_candidate_kind() {
    let cases = [
        (
            AppGameCategoryRiskCandidateKind::NativeAppCategory,
            AppGamePolicyTargetKind::AppCategory,
        ),
        (
            AppGameCategoryRiskCandidateKind::AppRisk,
            AppGamePolicyTargetKind::RiskApp,
        ),
        (
            AppGameCategoryRiskCandidateKind::NativeGameCategory,
            AppGamePolicyTargetKind::GameCategory,
        ),
        (
            AppGameCategoryRiskCandidateKind::MultiplayerGameContext,
            AppGamePolicyTargetKind::MultiplayerGame,
        ),
        (
            AppGameCategoryRiskCandidateKind::UserGeneratedContentGameContext,
            AppGamePolicyTargetKind::UgcGame,
        ),
        (
            AppGameCategoryRiskCandidateKind::PurchaseCapableGameContext,
            AppGamePolicyTargetKind::PurchaseCapableGame,
        ),
        (
            AppGameCategoryRiskCandidateKind::MatureGameContext,
            AppGamePolicyTargetKind::MatureGame,
        ),
    ];

    for (candidate_kind, target_kind) in cases {
        let route = route_app_game_category_risk_candidate(&base_request(candidate_kind));
        assert_eq!(
            route.route_state,
            AppGameCategoryRiskRouteState::CompileReady
        );
        assert_eq!(route.target_kind, target_kind);
        assert_eq!(
            route.adapter_dispatch_state,
            AppGameCategoryRiskAdapterDispatchState::NotDispatched
        );
    }
}

#[test]
fn category_risk_policy_routing_rejects_missing_stale_and_manual_category_proof() {
    let mut missing = base_request(AppGameCategoryRiskCandidateKind::NativeAppCategory);
    missing.candidate.category_proof_ref = None;
    assert_route(
        &missing,
        AppGameCategoryRiskRouteState::Rejected,
        AppGameCategoryRiskRouteReason::MissingCategoryProof,
    );

    let mut stale = base_request(AppGameCategoryRiskCandidateKind::NativeGameCategory);
    stale.candidate.category_proof_state = AppGameCategoryProofState::Stale;
    assert_route(
        &stale,
        AppGameCategoryRiskRouteState::Rejected,
        AppGameCategoryRiskRouteReason::StaleCategoryProof,
    );

    let mut manual = base_request(AppGameCategoryRiskCandidateKind::AppRisk);
    manual.candidate.category_proof_state = AppGameCategoryProofState::ManualRequired;
    assert_route(
        &manual,
        AppGameCategoryRiskRouteState::ManualRequired,
        AppGameCategoryRiskRouteReason::CandidateRequiresManualReview,
    );
}

#[test]
fn category_risk_policy_routing_requires_supporting_evidence_and_valid_confidence() {
    let mut missing_evidence = base_request(AppGameCategoryRiskCandidateKind::AppRisk);
    missing_evidence.candidate.supporting_evidence_refs.clear();
    assert_route(
        &missing_evidence,
        AppGameCategoryRiskRouteState::Rejected,
        AppGameCategoryRiskRouteReason::MissingSupportingEvidence,
    );

    let mut invalid_confidence = base_request(AppGameCategoryRiskCandidateKind::NativeAppCategory);
    invalid_confidence.candidate.confidence_permille = 1_001;
    assert_route(
        &invalid_confidence,
        AppGameCategoryRiskRouteState::Rejected,
        AppGameCategoryRiskRouteReason::InvalidConfidence,
    );

    let mut missing_target = base_request(AppGameCategoryRiskCandidateKind::NativeGameCategory);
    missing_target.target_ref = None;
    assert_route(
        &missing_target,
        AppGameCategoryRiskRouteState::Rejected,
        AppGameCategoryRiskRouteReason::MissingTargetReference,
    );
}

#[test]
fn category_risk_policy_routing_requires_a_bound_local_ai_digest() {
    let mut missing = base_request(AppGameCategoryRiskCandidateKind::AppRisk);
    missing.candidate.candidate_source = AppGameCategoryRiskCandidateSource::LocalAi;
    missing.candidate.ai_digest_ref = None;
    assert_route(
        &missing,
        AppGameCategoryRiskRouteState::Rejected,
        AppGameCategoryRiskRouteReason::MissingAiDigest,
    );

    let mut unbound = missing;
    unbound.candidate.ai_digest_ref = Some(evidence_ref("evidence-ai-digest"));
    assert_route(
        &unbound,
        AppGameCategoryRiskRouteState::Rejected,
        AppGameCategoryRiskRouteReason::UnboundAiDigest,
    );

    let digest_ref = evidence_ref("evidence-ai-digest");
    unbound
        .candidate
        .supporting_evidence_refs
        .push(digest_ref.clone());
    let route = route_app_game_category_risk_candidate(&unbound);
    assert_eq!(
        route.route_state,
        AppGameCategoryRiskRouteState::CompileReady
    );
    assert_eq!(
        route.supporting_evidence_refs,
        vec![
            evidence_ref("evidence-category-proof"),
            evidence_ref("evidence-supporting-1"),
            digest_ref,
        ]
    );
}

#[test]
fn category_risk_policy_routing_turns_hard_actions_into_manual_compiler_inputs() {
    let mut request = base_request(AppGameCategoryRiskCandidateKind::AppRisk);
    request.candidate.requested_action = AppGamePolicyCompilerRequestedAction::BlockLaunch;
    let route = route_app_game_category_risk_candidate(&request);

    assert_eq!(
        route.route_state,
        AppGameCategoryRiskRouteState::ManualRequired
    );
    assert_eq!(
        route.route_reason,
        AppGameCategoryRiskRouteReason::HardActionRequiresManualReview
    );
    let compiler_request = route
        .compiler_request
        .expect_value("manual compiler request");
    assert_eq!(
        compiler_request.requested_action,
        AppGamePolicyCompilerRequestedAction::ManualRequired
    );

    let compilation = compile_app_game_policy_target(compiler_request, compiler_context());
    assert_eq!(
        compilation.decision.outcome_state,
        AppGamePolicyCompilerOutcomeState::ManualRequired
    );
    assert_eq!(
        compilation.decision.enforcement_handoff_state,
        AppGamePolicyEnforcementHandoffState::Disabled
    );
}

#[test]
fn category_risk_policy_routing_produces_dry_run_compiler_input_without_adapter_claims() {
    let route = route_app_game_category_risk_candidate(&base_request(
        AppGameCategoryRiskCandidateKind::MultiplayerGameContext,
    ));
    let compiler_request = route.compiler_request.expect_value("compile-ready request");
    let compilation = compile_app_game_policy_target(compiler_request, compiler_context());

    assert_eq!(
        compilation.decision.outcome_state,
        AppGamePolicyCompilerOutcomeState::DryRunReady
    );
    assert!(compilation.decision.dry_run);
    assert_eq!(
        compilation.decision.enforcement_handoff_state,
        AppGamePolicyEnforcementHandoffState::Disabled
    );
    assert_eq!(
        route.adapter_dispatch_state,
        AppGameCategoryRiskAdapterDispatchState::NotDispatched
    );
    assert_eq!(
        compilation.decision.evidence_refs,
        route.supporting_evidence_refs
    );
}

fn assert_route(
    request: &AppGameCategoryRiskRouteRequest,
    state: AppGameCategoryRiskRouteState,
    reason: AppGameCategoryRiskRouteReason,
) {
    let route = route_app_game_category_risk_candidate(request);
    assert_eq!(route.route_state, state);
    assert_eq!(route.route_reason, reason);
    assert!(route.compiler_request.is_none());
    assert_eq!(
        route.adapter_dispatch_state,
        AppGameCategoryRiskAdapterDispatchState::NotDispatched
    );
}

fn base_request(
    candidate_kind: AppGameCategoryRiskCandidateKind,
) -> AppGameCategoryRiskRouteRequest {
    AppGameCategoryRiskRouteRequest {
        compile_request_id: AppGamePolicyCompileRequestId::parse("compile-category-risk-1")
            .expect_value("compile id"),
        rule_ref: AppGamePolicyRuleRef::parse("rule-category-risk-1").expect_value("rule ref"),
        device_id: AppGamePolicyDeviceId::parse("device-category-risk-1").expect_value("device id"),
        local_user_ref: AppGamePolicyLocalUserRef::parse("user-category-risk-1")
            .expect_value("user ref"),
        target_ref: Some(
            AppGamePolicyTargetRef::parse("target-category-risk-1").expect_value("target ref"),
        ),
        schedule_ref: None,
        candidate: AppGameCategoryRiskCandidate {
            candidate_kind,
            candidate_source: AppGameCategoryRiskCandidateSource::NativeInventory,
            confidence_permille: 900,
            category_proof_state: AppGameCategoryProofState::Active,
            category_proof_ref: Some(evidence_ref("evidence-category-proof")),
            supporting_evidence_refs: vec![evidence_ref("evidence-supporting-1")],
            ai_digest_ref: None,
            requested_action: AppGamePolicyCompilerRequestedAction::Warn,
        },
        capability_refs: vec![AppGamePolicyCompilerCapabilityEvidence {
            capability_ref: AppGamePolicyCapabilityRef::parse("capability-category-risk-1")
                .expect_value("capability ref"),
            capability_state: AppGamePolicyCompilerCapabilityState::Supported,
            evidence_refs: vec![evidence_ref("evidence-supporting-1")],
        }],
        authority_refs: Vec::new(),
    }
}

fn compiler_context() -> AppGamePolicyCompilerContext {
    AppGamePolicyCompilerContext {
        compiled_decision_id: AppGamePolicyCompiledDecisionId::parse("decision-category-risk-1")
            .expect_value("decision id"),
        audit_ref: AppGamePolicyAuditRef::parse("audit-category-risk-1").expect_value("audit ref"),
    }
}

fn evidence_ref(value: &str) -> AppGamePolicyEvidenceRef {
    AppGamePolicyEvidenceRef::parse(value).expect_value("evidence ref")
}
