use crate::{NetworkEvidenceGrade, NetworkEvidencePolicyAction, NetworkEvidencePolicyMode};

use super::{NetworkLinuxAdapterGateBoundaryReason, NetworkLinuxAdapterGateInput};

pub(super) fn boundary_reasons(
    input: &NetworkLinuxAdapterGateInput,
    has_required_artifacts: bool,
) -> Vec<NetworkLinuxAdapterGateBoundaryReason> {
    let mut reasons = Vec::new();
    if input.research_only {
        reasons.push(NetworkLinuxAdapterGateBoundaryReason::ResearchOnlyRequested);
    }
    match input.capability_state {
        super::NetworkLinuxAdapterCapabilityState::ManualRequired => {
            reasons.push(NetworkLinuxAdapterGateBoundaryReason::CapabilityManualRequired);
        }
        super::NetworkLinuxAdapterCapabilityState::Unavailable => {
            reasons.push(NetworkLinuxAdapterGateBoundaryReason::CapabilityUnavailable);
        }
        super::NetworkLinuxAdapterCapabilityState::DistroReady => {}
    }
    if input.policy_mapping.evidence_grade != NetworkEvidenceGrade::A {
        reasons.push(NetworkLinuxAdapterGateBoundaryReason::EvidenceGradeBelowProofThreshold);
    }
    if input.policy_mapping.mode != NetworkEvidencePolicyMode::DryRun
        || input.policy_mapping.mapped_action != NetworkEvidencePolicyAction::Block
    {
        reasons.push(NetworkLinuxAdapterGateBoundaryReason::PolicyNotLinuxAdapterApproved);
    }
    if !has_required_artifacts {
        reasons.push(NetworkLinuxAdapterGateBoundaryReason::MissingRequiredArtifact);
    }
    reasons
}
