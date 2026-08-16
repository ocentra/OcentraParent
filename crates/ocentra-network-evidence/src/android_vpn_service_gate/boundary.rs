use crate::{NetworkEvidenceGrade, NetworkEvidencePolicyAction, NetworkEvidencePolicyMode};

use super::{
    NetworkAndroidVpnServiceCapabilityState, NetworkAndroidVpnServiceGateBoundaryReason,
    NetworkAndroidVpnServiceGateInput, NetworkAndroidVpnServiceGateState,
};

pub(super) fn boundary_reasons(
    input: &NetworkAndroidVpnServiceGateInput,
    has_required_artifacts: bool,
) -> Vec<NetworkAndroidVpnServiceGateBoundaryReason> {
    let mut reasons = Vec::new();
    if input.research_only {
        reasons.push(NetworkAndroidVpnServiceGateBoundaryReason::ResearchOnlyRequested);
    }
    match input.capability_state {
        NetworkAndroidVpnServiceCapabilityState::ManualRequired => {
            reasons.push(NetworkAndroidVpnServiceGateBoundaryReason::CapabilityManualRequired);
        }
        NetworkAndroidVpnServiceCapabilityState::Unavailable => {
            reasons.push(NetworkAndroidVpnServiceGateBoundaryReason::CapabilityUnavailable);
        }
        NetworkAndroidVpnServiceCapabilityState::PhysicalDeviceReady => {}
    }
    if input.policy_mapping.evidence_grade != NetworkEvidenceGrade::A {
        reasons.push(NetworkAndroidVpnServiceGateBoundaryReason::EvidenceGradeBelowProofThreshold);
    }
    if input.policy_mapping.mode != NetworkEvidencePolicyMode::DryRun
        || input.policy_mapping.mapped_action != NetworkEvidencePolicyAction::Block
    {
        reasons.push(NetworkAndroidVpnServiceGateBoundaryReason::PolicyNotVpnServiceApproved);
    }
    if !has_required_artifacts {
        reasons.push(NetworkAndroidVpnServiceGateBoundaryReason::MissingRequiredArtifact);
    }
    reasons
}

pub(super) fn gate_state(
    research_only: bool,
    capability_state: NetworkAndroidVpnServiceCapabilityState,
    boundary_reasons: &[NetworkAndroidVpnServiceGateBoundaryReason],
) -> NetworkAndroidVpnServiceGateState {
    if research_only {
        return NetworkAndroidVpnServiceGateState::ResearchOnly;
    }
    if capability_state == NetworkAndroidVpnServiceCapabilityState::Unavailable {
        return NetworkAndroidVpnServiceGateState::Unavailable;
    }
    if boundary_reasons.is_empty() {
        NetworkAndroidVpnServiceGateState::PhysicalDeviceProofReady
    } else {
        NetworkAndroidVpnServiceGateState::ManualRequired
    }
}
