use crate::{NetworkEvidenceGrade, NetworkEvidencePolicyAction, NetworkEvidencePolicyMode};

use super::{
    NetworkAppleNetworkExtensionCapabilityState, NetworkAppleNetworkExtensionGateBoundaryReason,
    NetworkAppleNetworkExtensionGateInput, NetworkAppleNetworkExtensionGateState,
};

pub(super) fn boundary_reasons(
    input: &NetworkAppleNetworkExtensionGateInput,
    has_required_artifacts: bool,
) -> Vec<NetworkAppleNetworkExtensionGateBoundaryReason> {
    let mut reasons = Vec::new();
    if input.research_only {
        reasons.push(NetworkAppleNetworkExtensionGateBoundaryReason::ResearchOnlyRequested);
    }
    match input.capability_state {
        NetworkAppleNetworkExtensionCapabilityState::ManualRequired => {
            reasons.push(NetworkAppleNetworkExtensionGateBoundaryReason::CapabilityManualRequired);
        }
        NetworkAppleNetworkExtensionCapabilityState::Unavailable => {
            reasons.push(NetworkAppleNetworkExtensionGateBoundaryReason::CapabilityUnavailable);
        }
        NetworkAppleNetworkExtensionCapabilityState::AppleDeviceReady => {}
    }
    if input.policy_mapping.evidence_grade != NetworkEvidenceGrade::A {
        reasons
            .push(NetworkAppleNetworkExtensionGateBoundaryReason::EvidenceGradeBelowProofThreshold);
    }
    if input.policy_mapping.mode != NetworkEvidencePolicyMode::DryRun
        || input.policy_mapping.mapped_action != NetworkEvidencePolicyAction::Block
    {
        reasons.push(
            NetworkAppleNetworkExtensionGateBoundaryReason::PolicyNotNetworkExtensionApproved,
        );
    }
    if !has_required_artifacts {
        reasons.push(NetworkAppleNetworkExtensionGateBoundaryReason::MissingRequiredArtifact);
    }
    reasons
}

pub(super) fn gate_state(
    research_only: bool,
    capability_state: NetworkAppleNetworkExtensionCapabilityState,
    boundary_reasons: &[NetworkAppleNetworkExtensionGateBoundaryReason],
) -> NetworkAppleNetworkExtensionGateState {
    if research_only {
        return NetworkAppleNetworkExtensionGateState::ResearchOnly;
    }
    if capability_state == NetworkAppleNetworkExtensionCapabilityState::Unavailable {
        return NetworkAppleNetworkExtensionGateState::Unavailable;
    }
    if boundary_reasons.is_empty() {
        NetworkAppleNetworkExtensionGateState::AppleEntitlementProofReady
    } else {
        NetworkAppleNetworkExtensionGateState::ManualRequired
    }
}
