use crate::{
    NetworkPlatformClaimState, NetworkWindowsWfpGateState, NetworkWindowsWfpRequiredArtifact,
};

pub(super) fn windows_wfp_state(state: NetworkWindowsWfpGateState) -> NetworkPlatformClaimState {
    match state {
        NetworkWindowsWfpGateState::LabProofReady => NetworkPlatformClaimState::Ready,
        NetworkWindowsWfpGateState::ResearchOnly => NetworkPlatformClaimState::ResearchOnly,
        NetworkWindowsWfpGateState::ManualRequired => NetworkPlatformClaimState::ManualRequired,
        NetworkWindowsWfpGateState::Unavailable => NetworkPlatformClaimState::Unavailable,
    }
}

pub(super) fn windows_wfp_artifact_label(
    artifact: NetworkWindowsWfpRequiredArtifact,
) -> &'static str {
    match artifact {
        NetworkWindowsWfpRequiredArtifact::AdministratorPermissionProof => {
            "windows-wfp.administrator-permission"
        }
        NetworkWindowsWfpRequiredArtifact::DriverSigningProof => "windows-wfp.driver-signing",
        NetworkWindowsWfpRequiredArtifact::DriverPackageProof => "windows-wfp.driver-package",
        NetworkWindowsWfpRequiredArtifact::ProviderRegistrationPlan => {
            "windows-wfp.provider-registration"
        }
        NetworkWindowsWfpRequiredArtifact::LayerCapabilityMatrix => "windows-wfp.layer-capability",
        NetworkWindowsWfpRequiredArtifact::RollbackPlan => "windows-wfp.rollback-plan",
        NetworkWindowsWfpRequiredArtifact::LabResultArtifact => "windows-wfp.lab-result",
        NetworkWindowsWfpRequiredArtifact::AuditEvent => "windows-wfp.audit-event",
    }
}
