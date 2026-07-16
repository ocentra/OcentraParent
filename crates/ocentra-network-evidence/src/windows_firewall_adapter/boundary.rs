use crate::{NetworkEvidenceGrade, NetworkEvidencePolicyAction, NetworkEvidencePolicyMode};

use super::{
    NetworkWindowsFirewallAdapterProofError, NetworkWindowsFirewallAdapterProofInput,
    NetworkWindowsFirewallArtifactRefs, NetworkWindowsFirewallBoundaryReason,
    NetworkWindowsFirewallCapabilityState, NetworkWindowsFirewallProofState,
    NetworkWindowsFirewallRequiredArtifact,
};

pub(super) fn reject_unsupported_claims(
    input: &NetworkWindowsFirewallAdapterProofInput,
) -> Result<(), NetworkWindowsFirewallAdapterProofError> {
    [
        (
            input.exact_url_claimed,
            NetworkWindowsFirewallAdapterProofError::ExactUrlClaimRejected,
        ),
        (
            input.decrypted_payload_claimed,
            NetworkWindowsFirewallAdapterProofError::DecryptedPayloadClaimRejected,
        ),
        (
            input.page_content_claimed,
            NetworkWindowsFirewallAdapterProofError::PageContentClaimRejected,
        ),
        (
            input.host_firewall_mutation_claimed,
            NetworkWindowsFirewallAdapterProofError::HostFirewallMutationClaimRejected,
        ),
        (
            input.netsh_command_invoked,
            NetworkWindowsFirewallAdapterProofError::NetshCommandInvocationRejected,
        ),
        (
            input.powershell_command_invoked,
            NetworkWindowsFirewallAdapterProofError::PowershellCommandInvocationRejected,
        ),
    ]
    .into_iter()
    .find_map(|(claimed, error)| claimed.then_some(error))
    .map_or(Ok(()), Err)
}

pub(super) fn boundary_reasons(
    input: &NetworkWindowsFirewallAdapterProofInput,
    has_required_artifacts: bool,
) -> Vec<NetworkWindowsFirewallBoundaryReason> {
    let capability_reason = match input.capability_state {
        NetworkWindowsFirewallCapabilityState::ManualRequired => {
            Some(NetworkWindowsFirewallBoundaryReason::CapabilityManualRequired)
        }
        NetworkWindowsFirewallCapabilityState::Unavailable => {
            Some(NetworkWindowsFirewallBoundaryReason::CapabilityUnavailable)
        }
        NetworkWindowsFirewallCapabilityState::Supported => None,
    };
    let policy_not_approved = input.policy_mapping.mode != NetworkEvidencePolicyMode::DryRun
        || input.policy_mapping.mapped_action != NetworkEvidencePolicyAction::Block;

    [
        input
            .dry_run
            .then_some(NetworkWindowsFirewallBoundaryReason::DryRunRequested),
        capability_reason,
        (input.policy_mapping.evidence_grade != NetworkEvidenceGrade::A)
            .then_some(NetworkWindowsFirewallBoundaryReason::EvidenceGradeBelowApplyThreshold),
        policy_not_approved
            .then_some(NetworkWindowsFirewallBoundaryReason::PolicyNotFirewallApproved),
        (!has_required_artifacts)
            .then_some(NetworkWindowsFirewallBoundaryReason::MissingRequiredArtifact),
    ]
    .into_iter()
    .flatten()
    .collect()
}

pub(super) fn proof_state(
    dry_run: bool,
    capability_state: NetworkWindowsFirewallCapabilityState,
    boundary_reasons: &[NetworkWindowsFirewallBoundaryReason],
) -> NetworkWindowsFirewallProofState {
    if dry_run {
        return NetworkWindowsFirewallProofState::DryRun;
    }
    if capability_state == NetworkWindowsFirewallCapabilityState::Unavailable {
        return NetworkWindowsFirewallProofState::Unavailable;
    }
    if boundary_reasons.is_empty() {
        NetworkWindowsFirewallProofState::ApplyReady
    } else {
        NetworkWindowsFirewallProofState::ManualRequired
    }
}

pub(super) fn missing_required_artifacts(
    artifacts: &NetworkWindowsFirewallArtifactRefs,
) -> Vec<NetworkWindowsFirewallRequiredArtifact> {
    [
        (
            artifacts.adapter_authorization_ref.as_ref(),
            NetworkWindowsFirewallRequiredArtifact::AdapterAuthorization,
        ),
        (
            artifacts.adapter_capability_proof_ref.as_ref(),
            NetworkWindowsFirewallRequiredArtifact::CapabilityProof,
        ),
        (
            artifacts.apply_artifact_ref.as_ref(),
            NetworkWindowsFirewallRequiredArtifact::ApplyArtifact,
        ),
        (
            artifacts.result_artifact_ref.as_ref(),
            NetworkWindowsFirewallRequiredArtifact::ResultArtifact,
        ),
        (
            artifacts.rollback_artifact_ref.as_ref(),
            NetworkWindowsFirewallRequiredArtifact::RollbackArtifact,
        ),
        (
            artifacts.audit_event_ref.as_ref(),
            NetworkWindowsFirewallRequiredArtifact::AuditEvent,
        ),
    ]
    .into_iter()
    .filter_map(|(value, artifact)| value.is_none().then_some(artifact))
    .collect()
}
