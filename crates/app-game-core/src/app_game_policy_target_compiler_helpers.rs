use super::references::AppGamePolicyEvidenceRef;
use super::types::{
    AppGamePolicyCompileRequest, AppGamePolicyCompilerOutcomeState,
    AppGamePolicyCompilerRejectionReason,
};

#[path = "app_game_policy_target_compiler_evidence_validation.rs"]
mod app_game_policy_target_compiler_evidence_validation;
#[path = "app_game_policy_target_compiler_readiness.rs"]
mod app_game_policy_target_compiler_readiness;

pub(super) fn evaluate_request(
    request: &AppGamePolicyCompileRequest,
) -> (
    AppGamePolicyCompilerOutcomeState,
    AppGamePolicyCompilerRejectionReason,
) {
    if let Some(reason) =
        app_game_policy_target_compiler_evidence_validation::validate_evidence(request)
    {
        return rejected(reason);
    }
    if let Some(reason) =
        app_game_policy_target_compiler_evidence_validation::validate_target(request)
    {
        return rejected(reason);
    }
    if let Some(reason) =
        app_game_policy_target_compiler_readiness::validate_reference_bindings(request)
    {
        return rejected(reason);
    }
    app_game_policy_target_compiler_readiness::evaluate_readiness(request)
}

pub(super) fn collect_evidence_refs(
    request: &AppGamePolicyCompileRequest,
) -> Vec<AppGamePolicyEvidenceRef> {
    let mut refs = Vec::new();
    for evidence in &request.evidence {
        if !refs.contains(&evidence.evidence_ref) {
            refs.push(evidence.evidence_ref.clone());
        }
    }
    refs
}

fn rejected(
    reason: AppGamePolicyCompilerRejectionReason,
) -> (
    AppGamePolicyCompilerOutcomeState,
    AppGamePolicyCompilerRejectionReason,
) {
    (AppGamePolicyCompilerOutcomeState::Rejected, reason)
}
