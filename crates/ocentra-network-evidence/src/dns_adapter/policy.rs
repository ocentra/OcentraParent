use super::{
    NetworkDnsAdapterBoundaryReason, NetworkDnsAdapterCapabilityState, NetworkDnsAdapterProofInput,
    NetworkDnsAdapterProofState,
};
use crate::{NetworkEvidenceGrade, NetworkEvidencePolicyAction, NetworkEvidencePolicyMode};

pub(super) fn boundary_reasons(
    input: &NetworkDnsAdapterProofInput,
    has_required_artifacts: bool,
) -> Vec<NetworkDnsAdapterBoundaryReason> {
    let mut reasons = Vec::new();
    if input.dry_run {
        reasons.push(NetworkDnsAdapterBoundaryReason::DryRunRequested);
    }
    match input.capability_state {
        NetworkDnsAdapterCapabilityState::ManualRequired => {
            reasons.push(NetworkDnsAdapterBoundaryReason::CapabilityManualRequired);
        }
        NetworkDnsAdapterCapabilityState::Unavailable => {
            reasons.push(NetworkDnsAdapterBoundaryReason::CapabilityUnavailable);
        }
        NetworkDnsAdapterCapabilityState::Supported => {}
    }
    if input.policy_mapping.evidence_grade != NetworkEvidenceGrade::A {
        reasons.push(NetworkDnsAdapterBoundaryReason::EvidenceGradeBelowApplyThreshold);
    }
    if input.policy_mapping.mode != NetworkEvidencePolicyMode::DryRun
        || input.policy_mapping.mapped_action != NetworkEvidencePolicyAction::Block
    {
        reasons.push(NetworkDnsAdapterBoundaryReason::PolicyNotAdapterApproved);
    }
    if !has_required_artifacts {
        reasons.push(NetworkDnsAdapterBoundaryReason::MissingRequiredArtifact);
    }
    reasons
}

pub(super) fn proof_state(
    dry_run: bool,
    capability_state: NetworkDnsAdapterCapabilityState,
    boundary_reasons: &[NetworkDnsAdapterBoundaryReason],
) -> NetworkDnsAdapterProofState {
    if dry_run {
        return NetworkDnsAdapterProofState::DryRun;
    }
    if capability_state == NetworkDnsAdapterCapabilityState::Unavailable {
        return NetworkDnsAdapterProofState::Unavailable;
    }
    if boundary_reasons.is_empty() {
        NetworkDnsAdapterProofState::ApplyReady
    } else {
        NetworkDnsAdapterProofState::ManualRequired
    }
}
