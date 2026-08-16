use super::app_game_policy_target_compiler_evidence_validation::has_proof;
use crate::app_game_policy_target_compiler::references::AppGamePolicyEvidenceRef;
use crate::app_game_policy_target_compiler::types::{
    AppGamePolicyCompileRequest, AppGamePolicyCompilerAuthorityState,
    AppGamePolicyCompilerCapabilityState, AppGamePolicyCompilerOutcomeState,
    AppGamePolicyCompilerProofKind, AppGamePolicyCompilerRejectionReason,
    AppGamePolicyCompilerRequestedAction,
};

pub(super) fn validate_reference_bindings(
    request: &AppGamePolicyCompileRequest,
) -> Option<AppGamePolicyCompilerRejectionReason> {
    if request.capability_refs.is_empty() {
        return Some(AppGamePolicyCompilerRejectionReason::MissingCapabilityProof);
    }
    if request
        .capability_refs
        .iter()
        .any(|entry| !refs_bind_to_request(request, &entry.evidence_refs))
    {
        return Some(AppGamePolicyCompilerRejectionReason::UnboundCapabilityEvidence);
    }
    request
        .authority_refs
        .iter()
        .any(|entry| !refs_bind_to_request(request, &entry.evidence_refs))
        .then_some(AppGamePolicyCompilerRejectionReason::UnboundAuthorityEvidence)
}

pub(super) fn evaluate_readiness(
    request: &AppGamePolicyCompileRequest,
) -> (
    AppGamePolicyCompilerOutcomeState,
    AppGamePolicyCompilerRejectionReason,
) {
    if request.requested_action == AppGamePolicyCompilerRequestedAction::ManualRequired {
        return manual(AppGamePolicyCompilerRejectionReason::RequestedManualRequired);
    }

    let capability_ready = has_proof(request, AppGamePolicyCompilerProofKind::CapabilityProof)
        && request
            .capability_refs
            .iter()
            .all(|entry| entry.capability_state == AppGamePolicyCompilerCapabilityState::Supported);
    let authority_ready = has_proof(request, AppGamePolicyCompilerProofKind::AuthorityProof)
        && request
            .authority_refs
            .iter()
            .any(|entry| entry.authority_state == AppGamePolicyCompilerAuthorityState::Proved);
    let approval_ready = has_proof(request, AppGamePolicyCompilerProofKind::ApprovalProof);

    if request.requested_action.is_hard_action()
        && (!capability_ready || !authority_ready || !approval_ready)
    {
        return manual(hard_action_reason(request.requested_action));
    }
    if !capability_ready {
        return manual(AppGamePolicyCompilerRejectionReason::MissingCapabilityProof);
    }
    if request.requested_action.requires_authority() && !authority_ready {
        return manual(AppGamePolicyCompilerRejectionReason::MissingAuthorityProof);
    }
    (
        AppGamePolicyCompilerOutcomeState::DryRunReady,
        AppGamePolicyCompilerRejectionReason::None,
    )
}

fn hard_action_reason(
    action: AppGamePolicyCompilerRequestedAction,
) -> AppGamePolicyCompilerRejectionReason {
    if action == AppGamePolicyCompilerRequestedAction::BlockLaunch {
        AppGamePolicyCompilerRejectionReason::BlockLaunchManualRequired
    } else {
        AppGamePolicyCompilerRejectionReason::HardActionManualRequired
    }
}

fn refs_bind_to_request(
    request: &AppGamePolicyCompileRequest,
    refs: &[AppGamePolicyEvidenceRef],
) -> bool {
    !refs.is_empty()
        && refs.iter().all(|reference| {
            request
                .evidence
                .iter()
                .any(|evidence| evidence.evidence_ref == *reference)
        })
}

fn manual(
    reason: AppGamePolicyCompilerRejectionReason,
) -> (
    AppGamePolicyCompilerOutcomeState,
    AppGamePolicyCompilerRejectionReason,
) {
    (AppGamePolicyCompilerOutcomeState::ManualRequired, reason)
}
