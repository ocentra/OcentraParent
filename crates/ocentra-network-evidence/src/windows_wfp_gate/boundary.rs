use crate::{NetworkEvidenceGrade, NetworkEvidencePolicyAction, NetworkEvidencePolicyMode};

use super::{
    NetworkWindowsWfpArtifactRefs, NetworkWindowsWfpGateBoundaryReason,
    NetworkWindowsWfpGateCapabilityState, NetworkWindowsWfpGateError, NetworkWindowsWfpGateInput,
    NetworkWindowsWfpGateState, NetworkWindowsWfpRequiredArtifact,
};

pub(super) fn reject_unsupported_claims(
    input: &NetworkWindowsWfpGateInput,
) -> Result<(), NetworkWindowsWfpGateError> {
    [
        (
            input.exact_url_claimed,
            NetworkWindowsWfpGateError::ExactUrlClaimRejected,
        ),
        (
            input.decrypted_payload_claimed,
            NetworkWindowsWfpGateError::DecryptedPayloadClaimRejected,
        ),
        (
            input.page_content_claimed,
            NetworkWindowsWfpGateError::PageContentClaimRejected,
        ),
        (
            input.live_driver_install_claimed,
            NetworkWindowsWfpGateError::LiveDriverInstallClaimRejected,
        ),
        (
            input.callout_registration_claimed,
            NetworkWindowsWfpGateError::CalloutRegistrationClaimRejected,
        ),
        (
            input.packet_block_claimed,
            NetworkWindowsWfpGateError::PacketBlockClaimRejected,
        ),
        (
            input.kernel_payload_inspection_claimed,
            NetworkWindowsWfpGateError::KernelPayloadInspectionClaimRejected,
        ),
        (
            input.command_invocation_claimed,
            NetworkWindowsWfpGateError::CommandInvocationRejected,
        ),
    ]
    .into_iter()
    .find_map(|(claimed, error)| claimed.then_some(error))
    .map_or(Ok(()), Err)
}

pub(super) fn boundary_reasons(
    input: &NetworkWindowsWfpGateInput,
    has_required_artifacts: bool,
) -> Vec<NetworkWindowsWfpGateBoundaryReason> {
    let capability_reason = match input.capability_state {
        NetworkWindowsWfpGateCapabilityState::ManualRequired => {
            Some(NetworkWindowsWfpGateBoundaryReason::CapabilityManualRequired)
        }
        NetworkWindowsWfpGateCapabilityState::Unavailable => {
            Some(NetworkWindowsWfpGateBoundaryReason::CapabilityUnavailable)
        }
        NetworkWindowsWfpGateCapabilityState::LabReady => None,
    };
    let policy_not_approved = input.policy_mapping.mode != NetworkEvidencePolicyMode::DryRun
        || input.policy_mapping.mapped_action != NetworkEvidencePolicyAction::Block;

    [
        input
            .research_only
            .then_some(NetworkWindowsWfpGateBoundaryReason::ResearchOnlyRequested),
        capability_reason,
        (input.policy_mapping.evidence_grade != NetworkEvidenceGrade::A)
            .then_some(NetworkWindowsWfpGateBoundaryReason::EvidenceGradeBelowProofThreshold),
        policy_not_approved.then_some(NetworkWindowsWfpGateBoundaryReason::PolicyNotWfpApproved),
        (!has_required_artifacts)
            .then_some(NetworkWindowsWfpGateBoundaryReason::MissingRequiredArtifact),
    ]
    .into_iter()
    .flatten()
    .collect()
}

pub(super) fn gate_state(
    research_only: bool,
    capability_state: NetworkWindowsWfpGateCapabilityState,
    boundary_reasons: &[NetworkWindowsWfpGateBoundaryReason],
) -> NetworkWindowsWfpGateState {
    if research_only {
        return NetworkWindowsWfpGateState::ResearchOnly;
    }
    if capability_state == NetworkWindowsWfpGateCapabilityState::Unavailable {
        return NetworkWindowsWfpGateState::Unavailable;
    }
    if boundary_reasons.is_empty() {
        NetworkWindowsWfpGateState::LabProofReady
    } else {
        NetworkWindowsWfpGateState::ManualRequired
    }
}

pub(super) fn missing_required_artifacts(
    artifacts: &NetworkWindowsWfpArtifactRefs,
) -> Vec<NetworkWindowsWfpRequiredArtifact> {
    [
        (
            artifacts.administrator_permission_proof_ref.as_ref(),
            NetworkWindowsWfpRequiredArtifact::AdministratorPermissionProof,
        ),
        (
            artifacts.driver_signing_proof_ref.as_ref(),
            NetworkWindowsWfpRequiredArtifact::DriverSigningProof,
        ),
        (
            artifacts.driver_package_proof_ref.as_ref(),
            NetworkWindowsWfpRequiredArtifact::DriverPackageProof,
        ),
        (
            artifacts.provider_registration_plan_ref.as_ref(),
            NetworkWindowsWfpRequiredArtifact::ProviderRegistrationPlan,
        ),
        (
            artifacts.layer_capability_matrix_ref.as_ref(),
            NetworkWindowsWfpRequiredArtifact::LayerCapabilityMatrix,
        ),
        (
            artifacts.rollback_plan_ref.as_ref(),
            NetworkWindowsWfpRequiredArtifact::RollbackPlan,
        ),
        (
            artifacts.lab_result_artifact_ref.as_ref(),
            NetworkWindowsWfpRequiredArtifact::LabResultArtifact,
        ),
        (
            artifacts.audit_event_ref.as_ref(),
            NetworkWindowsWfpRequiredArtifact::AuditEvent,
        ),
    ]
    .into_iter()
    .filter_map(|(value, artifact)| value.is_none().then_some(artifact))
    .collect()
}
