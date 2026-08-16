#[path = "app_game_policy_target_compiler_helpers.rs"]
mod app_game_policy_target_compiler_helpers;
#[path = "app_game_policy_target_compiler_references.rs"]
pub mod references;
#[path = "app_game_policy_target_compiler_types.rs"]
pub mod types;

use references::{AppGamePolicyAuthorityRef, AppGamePolicyCapabilityRef, AppGamePolicyEvidenceRef};
use types::{
    AppGamePolicyCompilation, AppGamePolicyCompileRequest, AppGamePolicyCompiledDecision,
    AppGamePolicyCompilerContext, AppGamePolicyCompilerOutcomeState,
    AppGamePolicyCompilerRedactionState, AppGamePolicyCompilerTrace,
    AppGamePolicyCompilerTraceBoundary, AppGamePolicyCompilerTraceOwner,
    AppGamePolicyEnforcementHandoffState,
};

pub fn compile_app_game_policy_target(
    request: AppGamePolicyCompileRequest,
    context: AppGamePolicyCompilerContext,
) -> AppGamePolicyCompilation {
    let (outcome_state, rejection_reason) =
        app_game_policy_target_compiler_helpers::evaluate_request(&request);
    let evidence_refs = collect_evidence_refs(&request);
    let capability_refs = collect_capability_refs(&request);
    let authority_refs = collect_authority_refs(&request);
    let no_claim_reason = (outcome_state != AppGamePolicyCompilerOutcomeState::DryRunReady)
        .then_some(rejection_reason);

    AppGamePolicyCompilation {
        trace: AppGamePolicyCompilerTrace {
            run_id: context.audit_ref.clone(),
            correlation_id: request.compile_request_id.clone(),
            owner: AppGamePolicyCompilerTraceOwner::AppGameCore,
            boundary: AppGamePolicyCompilerTraceBoundary::PolicyTargetCompiler,
            result: outcome_state,
            no_claim_reason,
            redaction_state: AppGamePolicyCompilerRedactionState::OpaqueReferencesOnly,
        },
        decision: AppGamePolicyCompiledDecision {
            compiled_decision_id: context.compiled_decision_id,
            rule_refs: vec![request.rule_ref.clone()],
            audit_refs: vec![context.audit_ref],
            request,
            outcome_state,
            rejection_reason,
            dry_run: true,
            enforcement_handoff_state: AppGamePolicyEnforcementHandoffState::Disabled,
            evidence_refs,
            capability_refs,
            authority_refs,
        },
    }
}

fn collect_evidence_refs(request: &AppGamePolicyCompileRequest) -> Vec<AppGamePolicyEvidenceRef> {
    app_game_policy_target_compiler_helpers::collect_evidence_refs(request)
}

fn collect_capability_refs(
    request: &AppGamePolicyCompileRequest,
) -> Vec<AppGamePolicyCapabilityRef> {
    request
        .capability_refs
        .iter()
        .map(|entry| entry.capability_ref.clone())
        .collect()
}

fn collect_authority_refs(request: &AppGamePolicyCompileRequest) -> Vec<AppGamePolicyAuthorityRef> {
    request
        .authority_refs
        .iter()
        .map(|entry| entry.authority_ref.clone())
        .collect()
}

pub fn app_game_policy_target_compiler_rules_typescript() -> String {
    include_str!(
        "../../../packages/schema-domain/src/generated-app-game-policy-target-compiler-rules.ts"
    )
    .to_string()
}
