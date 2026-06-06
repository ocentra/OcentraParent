use crate::{
    NetworkAndroidVpnServiceGateState, NetworkAndroidVpnServiceRequiredArtifact,
    NetworkAppleNetworkExtensionGateState, NetworkAppleNetworkExtensionPlatform,
    NetworkAppleNetworkExtensionRequiredArtifact, NetworkLinuxAdapterGateState,
    NetworkLinuxAdapterKind, NetworkLinuxAdapterRequiredArtifact, NetworkPlatformClaimState,
    NetworkPlatformClaimTarget, NetworkWindowsFirewallProofState,
    NetworkWindowsFirewallRequiredArtifact, NetworkWindowsWfpGateState,
    NetworkWindowsWfpRequiredArtifact,
};

pub(crate) fn windows_firewall_state(
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

pub(crate) fn windows_wfp_state(state: NetworkWindowsWfpGateState) -> NetworkPlatformClaimState {
    match state {
        NetworkWindowsWfpGateState::LabProofReady => NetworkPlatformClaimState::Ready,
        NetworkWindowsWfpGateState::ResearchOnly => NetworkPlatformClaimState::ResearchOnly,
        NetworkWindowsWfpGateState::ManualRequired => NetworkPlatformClaimState::ManualRequired,
        NetworkWindowsWfpGateState::Unavailable => NetworkPlatformClaimState::Unavailable,
    }
}

pub(crate) fn android_vpn_state(
    state: NetworkAndroidVpnServiceGateState,
) -> NetworkPlatformClaimState {
    match state {
        NetworkAndroidVpnServiceGateState::PhysicalDeviceProofReady => {
            NetworkPlatformClaimState::Ready
        }
        NetworkAndroidVpnServiceGateState::ResearchOnly => NetworkPlatformClaimState::ResearchOnly,
        NetworkAndroidVpnServiceGateState::ManualRequired => {
            NetworkPlatformClaimState::ManualRequired
        }
        NetworkAndroidVpnServiceGateState::Unavailable => NetworkPlatformClaimState::Unavailable,
    }
}

pub(crate) fn apple_state(
    state: NetworkAppleNetworkExtensionGateState,
) -> NetworkPlatformClaimState {
    match state {
        NetworkAppleNetworkExtensionGateState::AppleEntitlementProofReady => {
            NetworkPlatformClaimState::Ready
        }
        NetworkAppleNetworkExtensionGateState::ResearchOnly => {
            NetworkPlatformClaimState::ResearchOnly
        }
        NetworkAppleNetworkExtensionGateState::ManualRequired => {
            NetworkPlatformClaimState::ManualRequired
        }
        NetworkAppleNetworkExtensionGateState::Unavailable => {
            NetworkPlatformClaimState::Unavailable
        }
    }
}

pub(crate) fn linux_state(state: NetworkLinuxAdapterGateState) -> NetworkPlatformClaimState {
    match state {
        NetworkLinuxAdapterGateState::DistroProofReady => NetworkPlatformClaimState::Ready,
        NetworkLinuxAdapterGateState::ResearchOnly => NetworkPlatformClaimState::ResearchOnly,
        NetworkLinuxAdapterGateState::ManualRequired => NetworkPlatformClaimState::ManualRequired,
        NetworkLinuxAdapterGateState::Unavailable => NetworkPlatformClaimState::Unavailable,
    }
}

pub(crate) fn apple_target(
    platform: NetworkAppleNetworkExtensionPlatform,
) -> NetworkPlatformClaimTarget {
    match platform {
        NetworkAppleNetworkExtensionPlatform::MacOs => {
            NetworkPlatformClaimTarget::AppleNetworkExtensionMacOs
        }
        NetworkAppleNetworkExtensionPlatform::Ios => {
            NetworkPlatformClaimTarget::AppleNetworkExtensionIos
        }
    }
}

pub(crate) fn linux_target(kind: NetworkLinuxAdapterKind) -> NetworkPlatformClaimTarget {
    match kind {
        NetworkLinuxAdapterKind::Nftables => NetworkPlatformClaimTarget::LinuxNftables,
        NetworkLinuxAdapterKind::Ebpf => NetworkPlatformClaimTarget::LinuxEbpf,
        NetworkLinuxAdapterKind::Tun => NetworkPlatformClaimTarget::LinuxTun,
    }
}

pub(crate) fn compact_refs(values: Vec<Option<String>>) -> Vec<String> {
    let mut refs = Vec::new();
    for value in values.into_iter().flatten() {
        if !value.is_empty() && !refs.contains(&value) {
            refs.push(value);
        }
    }
    refs
}

pub(crate) fn windows_firewall_artifact_label(
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

pub(crate) fn windows_wfp_artifact_label(
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

pub(crate) fn android_artifact_label(
    artifact: NetworkAndroidVpnServiceRequiredArtifact,
) -> &'static str {
    match artifact {
        NetworkAndroidVpnServiceRequiredArtifact::VpnServiceDeclaration => {
            "android-vpn.vpn-service-declaration"
        }
        NetworkAndroidVpnServiceRequiredArtifact::UserConsentProof => "android-vpn.user-consent",
        NetworkAndroidVpnServiceRequiredArtifact::PhysicalDeviceProof => {
            "android-vpn.physical-device"
        }
        NetworkAndroidVpnServiceRequiredArtifact::PackageIdentityProof => {
            "android-vpn.package-identity"
        }
        NetworkAndroidVpnServiceRequiredArtifact::VirtualInterfaceProof => {
            "android-vpn.virtual-interface"
        }
        NetworkAndroidVpnServiceRequiredArtifact::TrafficObservationProof => {
            "android-vpn.traffic-observation"
        }
        NetworkAndroidVpnServiceRequiredArtifact::RollbackPlan => "android-vpn.rollback-plan",
        NetworkAndroidVpnServiceRequiredArtifact::AuditEvent => "android-vpn.audit-event",
        NetworkAndroidVpnServiceRequiredArtifact::DeviceOwnerProof => "android-vpn.device-owner",
    }
}

pub(crate) fn apple_artifact_label(
    artifact: NetworkAppleNetworkExtensionRequiredArtifact,
) -> &'static str {
    match artifact {
        NetworkAppleNetworkExtensionRequiredArtifact::DeveloperTeamProof => {
            "apple-network-extension.developer-team"
        }
        NetworkAppleNetworkExtensionRequiredArtifact::EntitlementApprovalProof => {
            "apple-network-extension.entitlement-approval"
        }
        NetworkAppleNetworkExtensionRequiredArtifact::ProvisioningProfileProof => {
            "apple-network-extension.provisioning-profile"
        }
        NetworkAppleNetworkExtensionRequiredArtifact::SigningProof => {
            "apple-network-extension.signing"
        }
        NetworkAppleNetworkExtensionRequiredArtifact::DeviceOrTestFlightProof => {
            "apple-network-extension.device-or-testflight"
        }
        NetworkAppleNetworkExtensionRequiredArtifact::NetworkExtensionDeclaration => {
            "apple-network-extension.declaration"
        }
        NetworkAppleNetworkExtensionRequiredArtifact::ExtensionConfigurationProof => {
            "apple-network-extension.configuration"
        }
        NetworkAppleNetworkExtensionRequiredArtifact::RollbackPlan => {
            "apple-network-extension.rollback-plan"
        }
        NetworkAppleNetworkExtensionRequiredArtifact::AuditEvent => {
            "apple-network-extension.audit-event"
        }
        NetworkAppleNetworkExtensionRequiredArtifact::SupervisionOrMdmProof => {
            "apple-network-extension.supervision-mdm"
        }
    }
}

pub(crate) fn linux_artifact_label(artifact: NetworkLinuxAdapterRequiredArtifact) -> &'static str {
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
