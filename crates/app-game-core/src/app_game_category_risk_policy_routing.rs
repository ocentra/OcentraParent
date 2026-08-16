#[path = "app_game_category_risk_policy_routing_manual.rs"]
mod app_game_category_risk_policy_routing_manual;
#[path = "app_game_category_risk_policy_routing_validation.rs"]
mod app_game_category_risk_policy_routing_validation;
#[path = "app_game_category_risk_policy_routing_types.rs"]
pub mod types;

use crate::app_game_policy_target_compiler::compile_app_game_policy_target;
use crate::app_game_policy_target_compiler::references::AppGamePolicyEvidenceRef;
use crate::app_game_policy_target_compiler::types::{
    AppGamePolicyCompilation, AppGamePolicyCompilerContext,
};
use crate::app_game_policy_target_compiler::types::{
    AppGamePolicyCompileRequest, AppGamePolicyCompilerEvidence, AppGamePolicyCompilerEvidenceState,
    AppGamePolicyCompilerProofKind, AppGamePolicyCompilerRequestedAction,
    AppGamePolicyCompilerTarget,
};
use types::{
    AppGameCategoryRiskAdapterDispatchState, AppGameCategoryRiskRoute,
    AppGameCategoryRiskRouteReason, AppGameCategoryRiskRouteRequest, AppGameCategoryRiskRouteState,
};

pub fn route_app_game_category_risk_candidate(
    request: &AppGameCategoryRiskRouteRequest,
) -> AppGameCategoryRiskRoute {
    let target_kind = request.candidate.candidate_kind.target_kind();
    if let Some(route) = app_game_category_risk_policy_routing_validation::route_failure(request) {
        return route;
    }

    let manual_reason = app_game_category_risk_policy_routing_manual::manual_reason(request);
    let requested_action = manual_reason
        .map(|_| AppGamePolicyCompilerRequestedAction::ManualRequired)
        .unwrap_or(request.candidate.requested_action);
    let compiler_request = build_compiler_request(request, requested_action);

    AppGameCategoryRiskRoute {
        route_state: manual_reason
            .map(|_| AppGameCategoryRiskRouteState::ManualRequired)
            .unwrap_or(AppGameCategoryRiskRouteState::CompileReady),
        route_reason: manual_reason.unwrap_or(AppGameCategoryRiskRouteReason::None),
        target_kind,
        compiler_request: Some(compiler_request),
        adapter_dispatch_state: AppGameCategoryRiskAdapterDispatchState::NotDispatched,
        supporting_evidence_refs: collect_evidence_refs(request),
        capability_refs: request
            .capability_refs
            .iter()
            .map(|entry| entry.capability_ref.clone())
            .collect(),
        authority_refs: request
            .authority_refs
            .iter()
            .map(|entry| entry.authority_ref.clone())
            .collect(),
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AppGameCategoryRiskCompilation {
    pub route: AppGameCategoryRiskRoute,
    pub compilation: Option<AppGamePolicyCompilation>,
}

pub fn compile_app_game_category_risk_candidate(
    request: &AppGameCategoryRiskRouteRequest,
    context: AppGamePolicyCompilerContext,
) -> AppGameCategoryRiskCompilation {
    let route = route_app_game_category_risk_candidate(request);
    let compilation = route
        .compiler_request
        .clone()
        .map(|compiler_request| compile_app_game_policy_target(compiler_request, context));

    AppGameCategoryRiskCompilation { route, compilation }
}

fn build_compiler_request(
    request: &AppGameCategoryRiskRouteRequest,
    requested_action: AppGamePolicyCompilerRequestedAction,
) -> AppGamePolicyCompileRequest {
    AppGamePolicyCompileRequest {
        compile_request_id: request.compile_request_id.clone(),
        rule_ref: request.rule_ref.clone(),
        device_id: request.device_id.clone(),
        local_user_ref: request.local_user_ref.clone(),
        target: AppGamePolicyCompilerTarget {
            target_kind: request.candidate.candidate_kind.target_kind(),
            target_ref: request.target_ref.clone(),
        },
        requested_action,
        schedule_ref: request.schedule_ref.clone(),
        evidence: collect_evidence(request),
        capability_refs: request.capability_refs.clone(),
        authority_refs: request.authority_refs.clone(),
    }
}

fn collect_evidence(
    request: &AppGameCategoryRiskRouteRequest,
) -> Vec<AppGamePolicyCompilerEvidence> {
    let mut evidence: Vec<AppGamePolicyCompilerEvidence> = collect_evidence_refs(request)
        .into_iter()
        .map(|evidence_ref| {
            evidence_entry(
                request,
                evidence_ref,
                AppGamePolicyCompilerProofKind::CategoryProof,
            )
        })
        .collect();
    for capability in &request.capability_refs {
        evidence.extend(
            capability
                .evidence_refs
                .iter()
                .cloned()
                .map(|evidence_ref| {
                    evidence_entry(
                        request,
                        evidence_ref,
                        AppGamePolicyCompilerProofKind::CapabilityProof,
                    )
                }),
        );
    }
    for authority in &request.authority_refs {
        evidence.extend(authority.evidence_refs.iter().cloned().map(|evidence_ref| {
            evidence_entry(
                request,
                evidence_ref,
                AppGamePolicyCompilerProofKind::AuthorityProof,
            )
        }));
    }
    evidence
}

fn evidence_entry(
    request: &AppGameCategoryRiskRouteRequest,
    evidence_ref: AppGamePolicyEvidenceRef,
    proof_kind: AppGamePolicyCompilerProofKind,
) -> AppGamePolicyCompilerEvidence {
    AppGamePolicyCompilerEvidence {
        evidence_ref,
        proof_kind,
        evidence_state: AppGamePolicyCompilerEvidenceState::Active,
        device_id: request.device_id.clone(),
        local_user_ref: request.local_user_ref.clone(),
    }
}

fn collect_evidence_refs(
    request: &AppGameCategoryRiskRouteRequest,
) -> Vec<AppGamePolicyEvidenceRef> {
    let mut evidence_refs = Vec::new();
    if let Some(category_proof_ref) = request.candidate.category_proof_ref.clone() {
        evidence_refs.push(category_proof_ref);
    }
    for evidence_ref in &request.candidate.supporting_evidence_refs {
        if !evidence_refs.contains(evidence_ref) {
            evidence_refs.push(evidence_ref.clone());
        }
    }
    evidence_refs
}

pub(super) fn terminal_route(
    request: &AppGameCategoryRiskRouteRequest,
    route_state: AppGameCategoryRiskRouteState,
    route_reason: AppGameCategoryRiskRouteReason,
) -> AppGameCategoryRiskRoute {
    AppGameCategoryRiskRoute {
        route_state,
        route_reason,
        target_kind: request.candidate.candidate_kind.target_kind(),
        compiler_request: None,
        adapter_dispatch_state: AppGameCategoryRiskAdapterDispatchState::NotDispatched,
        supporting_evidence_refs: collect_evidence_refs(request),
        capability_refs: Vec::new(),
        authority_refs: Vec::new(),
    }
}
