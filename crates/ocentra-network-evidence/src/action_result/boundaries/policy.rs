use super::*;

pub(super) fn push_policy_reasons(
    input: &NetworkActionResultInput,
    has_required_artifacts: bool,
    reasons: &mut Vec<NetworkActionResultBoundaryReason>,
) {
    if input.policy_mapping.evidence_grade != NetworkEvidenceGrade::A {
        reasons.push(NetworkActionResultBoundaryReason::EvidenceGradeBelowApplyThreshold);
    }
    if input.policy_mapping.mode != NetworkEvidencePolicyMode::DryRun
        || input.policy_mapping.mapped_action != NetworkEvidencePolicyAction::Block
    {
        reasons.push(NetworkActionResultBoundaryReason::PolicyNotAdapterApproved);
    }
    if input.requested_action == NetworkActionResultRequestedAction::TerminateProcess
        && !matches!(
            input.target_kind,
            NetworkActionResultTargetKind::Process | NetworkActionResultTargetKind::App
        )
    {
        reasons.push(NetworkActionResultBoundaryReason::TerminateTargetNotProcessOrApp);
    }
    if !has_required_artifacts {
        reasons.push(NetworkActionResultBoundaryReason::MissingRequiredArtifact);
    }
}
