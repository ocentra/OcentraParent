use crate::{
    NetworkLinuxAdapterGateState, NetworkLinuxAdapterKind, NetworkLinuxAdapterRequiredArtifact,
    NetworkPlatformClaimState, NetworkPlatformClaimTarget,
};

pub(super) fn linux_state(state: NetworkLinuxAdapterGateState) -> NetworkPlatformClaimState {
    match state {
        NetworkLinuxAdapterGateState::DistroProofReady => NetworkPlatformClaimState::Ready,
        NetworkLinuxAdapterGateState::ResearchOnly => NetworkPlatformClaimState::ResearchOnly,
        NetworkLinuxAdapterGateState::ManualRequired => NetworkPlatformClaimState::ManualRequired,
        NetworkLinuxAdapterGateState::Unavailable => NetworkPlatformClaimState::Unavailable,
    }
}

pub(super) fn linux_target(kind: NetworkLinuxAdapterKind) -> NetworkPlatformClaimTarget {
    match kind {
        NetworkLinuxAdapterKind::Nftables => NetworkPlatformClaimTarget::LinuxNftables,
        NetworkLinuxAdapterKind::Ebpf => NetworkPlatformClaimTarget::LinuxEbpf,
        NetworkLinuxAdapterKind::Tun => NetworkPlatformClaimTarget::LinuxTun,
    }
}

pub(super) fn linux_artifact_label(artifact: NetworkLinuxAdapterRequiredArtifact) -> &'static str {
    match artifact {
        NetworkLinuxAdapterRequiredArtifact::DistroKernelProof => "linux-adapter.distro-kernel",
        NetworkLinuxAdapterRequiredArtifact::PermissionProof => "linux-adapter.permission",
        NetworkLinuxAdapterRequiredArtifact::AdapterApiCapabilityProof => {
            "linux-adapter.api-capability"
        }
        NetworkLinuxAdapterRequiredArtifact::AdapterPlanProof => "linux-adapter.plan",
        NetworkLinuxAdapterRequiredArtifact::ServiceManagerScopeProof => {
            "linux-adapter.service-manager"
        }
        NetworkLinuxAdapterRequiredArtifact::RollbackPlan => "linux-adapter.rollback-plan",
        NetworkLinuxAdapterRequiredArtifact::LabResultArtifact => "linux-adapter.lab-result",
        NetworkLinuxAdapterRequiredArtifact::AuditEvent => "linux-adapter.audit-event",
    }
}
