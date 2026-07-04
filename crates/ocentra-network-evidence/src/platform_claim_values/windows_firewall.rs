use crate::{
    NetworkPlatformClaimState, NetworkWindowsFirewallProofState,
    NetworkWindowsFirewallRequiredArtifact,
};

pub(super) fn windows_firewall_state(
    state: NetworkWindowsFirewallProofState,
) -> NetworkPlatformClaimState {
    match state {
        NetworkWindowsFirewallProofState::ApplyReady => NetworkPlatformClaimState::Ready,
        NetworkWindowsFirewallProofState::DryRun => NetworkPlatformClaimState::DryRun,
        NetworkWindowsFirewallProofState::ManualRequired => {
            NetworkPlatformClaimState::ManualRequired
        }
        NetworkWindowsFirewallProofState::Unavailable => NetworkPlatformClaimState::Unavailable,
    }
}

pub(super) fn windows_firewall_artifact_label(
    artifact: NetworkWindowsFirewallRequiredArtifact,
) -> &'static str {
    match artifact {
        NetworkWindowsFirewallRequiredArtifact::AdapterAuthorization => {
            "windows-firewall.adapter-authorization"
        }
        NetworkWindowsFirewallRequiredArtifact::CapabilityProof => {
            "windows-firewall.capability-proof"
        }
        NetworkWindowsFirewallRequiredArtifact::ApplyArtifact => "windows-firewall.apply-artifact",
        NetworkWindowsFirewallRequiredArtifact::ResultArtifact => {
            "windows-firewall.result-artifact"
        }
        NetworkWindowsFirewallRequiredArtifact::RollbackArtifact => {
            "windows-firewall.rollback-artifact"
        }
        NetworkWindowsFirewallRequiredArtifact::AuditEvent => "windows-firewall.audit-event",
    }
}
