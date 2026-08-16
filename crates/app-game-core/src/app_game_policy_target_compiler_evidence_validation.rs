use crate::app_game_policy_target_compiler::types::{
    AppGamePolicyCompileRequest, AppGamePolicyCompilerEvidenceState,
    AppGamePolicyCompilerProofKind, AppGamePolicyCompilerRejectionReason,
};

pub(super) fn validate_evidence(
    request: &AppGamePolicyCompileRequest,
) -> Option<AppGamePolicyCompilerRejectionReason> {
    if request.evidence.is_empty() {
        return Some(AppGamePolicyCompilerRejectionReason::MissingEvidence);
    }
    if request.evidence.iter().any(|evidence| {
        evidence.evidence_state == AppGamePolicyCompilerEvidenceState::WrongDevice
            || evidence.device_id != request.device_id
    }) {
        return Some(AppGamePolicyCompilerRejectionReason::WrongDevice);
    }
    if request.evidence.iter().any(|evidence| {
        evidence.evidence_state == AppGamePolicyCompilerEvidenceState::WrongLocalUser
            || evidence.local_user_ref != request.local_user_ref
    }) {
        return Some(AppGamePolicyCompilerRejectionReason::WrongLocalUser);
    }
    request
        .evidence
        .iter()
        .any(|evidence| evidence.evidence_state == AppGamePolicyCompilerEvidenceState::Stale)
        .then_some(AppGamePolicyCompilerRejectionReason::StaleEvidence)
}

pub(super) fn validate_target(
    request: &AppGamePolicyCompileRequest,
) -> Option<AppGamePolicyCompilerRejectionReason> {
    if request.target.target_ref.is_none()
        && !request.target.target_kind.permits_missing_target_ref()
    {
        return Some(AppGamePolicyCompilerRejectionReason::MissingTargetReference);
    }
    if request.target.target_kind.requires_identity_proof()
        && !has_proof(request, AppGamePolicyCompilerProofKind::IdentityProof)
    {
        return Some(AppGamePolicyCompilerRejectionReason::MissingIdentityProof);
    }
    if request.target.target_kind.requires_unknown_state_proof()
        && !has_proof(request, AppGamePolicyCompilerProofKind::UnknownStateProof)
    {
        return Some(AppGamePolicyCompilerRejectionReason::MissingUnknownStateProof);
    }
    if request.target.target_kind.requires_category_proof()
        && !has_proof(request, AppGamePolicyCompilerProofKind::CategoryProof)
    {
        return Some(AppGamePolicyCompilerRejectionReason::MissingCategoryProof);
    }
    (request.schedule_ref.is_some()
        && !has_proof(request, AppGamePolicyCompilerProofKind::ScheduleProof))
    .then_some(AppGamePolicyCompilerRejectionReason::MissingScheduleProof)
}

pub(super) fn has_proof(
    request: &AppGamePolicyCompileRequest,
    proof_kind: AppGamePolicyCompilerProofKind,
) -> bool {
    request
        .evidence
        .iter()
        .any(|evidence| evidence.proof_kind == proof_kind)
}
