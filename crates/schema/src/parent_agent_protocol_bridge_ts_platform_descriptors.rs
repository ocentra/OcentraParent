fn network_linux_nftables_command_kind_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkLinuxNftablesLabCommandStatusKind>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "CreateTable",
            value: NetworkLinuxNftablesLabCommandStatusKind::CreateTable,
        },
        ProtocolLiteralDescriptor {
            key: "CreateChain",
            value: NetworkLinuxNftablesLabCommandStatusKind::CreateChain,
        },
        ProtocolLiteralDescriptor {
            key: "AddRule",
            value: NetworkLinuxNftablesLabCommandStatusKind::AddRule,
        },
        ProtocolLiteralDescriptor {
            key: "VerifyRulePresent",
            value: NetworkLinuxNftablesLabCommandStatusKind::VerifyRulePresent,
        },
        ProtocolLiteralDescriptor {
            key: "DeleteTable",
            value: NetworkLinuxNftablesLabCommandStatusKind::DeleteTable,
        },
        ProtocolLiteralDescriptor {
            key: "VerifyTableRemoved",
            value: NetworkLinuxNftablesLabCommandStatusKind::VerifyTableRemoved,
        },
    ]
}

fn network_windows_firewall_status_state_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkWindowsFirewallLabStatusState>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "ManualRequired",
            value: NetworkWindowsFirewallLabStatusState::ManualRequired,
        },
        ProtocolLiteralDescriptor {
            key: "ExecutedAndRolledBack",
            value: NetworkWindowsFirewallLabStatusState::ExecutedAndRolledBack,
        },
        ProtocolLiteralDescriptor {
            key: "Unavailable",
            value: NetworkWindowsFirewallLabStatusState::Unavailable,
        },
    ]
}

fn network_windows_firewall_command_kind_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkWindowsFirewallLabCommandStatusKind>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "ApplyRule",
            value: NetworkWindowsFirewallLabCommandStatusKind::ApplyRule,
        },
        ProtocolLiteralDescriptor {
            key: "VerifyRulePresent",
            value: NetworkWindowsFirewallLabCommandStatusKind::VerifyRulePresent,
        },
        ProtocolLiteralDescriptor {
            key: "RollbackRule",
            value: NetworkWindowsFirewallLabCommandStatusKind::RollbackRule,
        },
        ProtocolLiteralDescriptor {
            key: "VerifyRuleRemoved",
            value: NetworkWindowsFirewallLabCommandStatusKind::VerifyRuleRemoved,
        },
    ]
}

fn network_windows_wfp_gate_state_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkWindowsWfpGateStatusState>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "ManualRequired",
            value: NetworkWindowsWfpGateStatusState::ManualRequired,
        },
        ProtocolLiteralDescriptor {
            key: "ResearchOnly",
            value: NetworkWindowsWfpGateStatusState::ResearchOnly,
        },
        ProtocolLiteralDescriptor {
            key: "Unavailable",
            value: NetworkWindowsWfpGateStatusState::Unavailable,
        },
        ProtocolLiteralDescriptor {
            key: "LabProofReady",
            value: NetworkWindowsWfpGateStatusState::LabProofReady,
        },
    ]
}

fn network_windows_wfp_capability_state_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkWindowsWfpGateCapabilityStatusState>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "ManualRequired",
            value: NetworkWindowsWfpGateCapabilityStatusState::ManualRequired,
        },
        ProtocolLiteralDescriptor {
            key: "LabReady",
            value: NetworkWindowsWfpGateCapabilityStatusState::LabReady,
        },
        ProtocolLiteralDescriptor {
            key: "Unavailable",
            value: NetworkWindowsWfpGateCapabilityStatusState::Unavailable,
        },
    ]
}

fn network_android_vpn_gate_state_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkAndroidVpnServiceGateStatusState>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "ManualRequired",
            value: NetworkAndroidVpnServiceGateStatusState::ManualRequired,
        },
        ProtocolLiteralDescriptor {
            key: "ResearchOnly",
            value: NetworkAndroidVpnServiceGateStatusState::ResearchOnly,
        },
        ProtocolLiteralDescriptor {
            key: "Unavailable",
            value: NetworkAndroidVpnServiceGateStatusState::Unavailable,
        },
        ProtocolLiteralDescriptor {
            key: "PhysicalDeviceProofReady",
            value: NetworkAndroidVpnServiceGateStatusState::PhysicalDeviceProofReady,
        },
    ]
}

fn network_android_vpn_capability_state_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkAndroidVpnServiceGateCapabilityStatusState>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "PhysicalDeviceReady",
            value: NetworkAndroidVpnServiceGateCapabilityStatusState::PhysicalDeviceReady,
        },
        ProtocolLiteralDescriptor {
            key: "ManualRequired",
            value: NetworkAndroidVpnServiceGateCapabilityStatusState::ManualRequired,
        },
        ProtocolLiteralDescriptor {
            key: "Unavailable",
            value: NetworkAndroidVpnServiceGateCapabilityStatusState::Unavailable,
        },
    ]
}

fn network_android_vpn_required_artifact_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkAndroidVpnServiceGateRequiredArtifact>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "VpnServiceDeclaration",
            value: NetworkAndroidVpnServiceGateRequiredArtifact::VpnServiceDeclaration,
        },
        ProtocolLiteralDescriptor {
            key: "UserConsentProof",
            value: NetworkAndroidVpnServiceGateRequiredArtifact::UserConsentProof,
        },
        ProtocolLiteralDescriptor {
            key: "PhysicalDeviceProof",
            value: NetworkAndroidVpnServiceGateRequiredArtifact::PhysicalDeviceProof,
        },
        ProtocolLiteralDescriptor {
            key: "PackageIdentityProof",
            value: NetworkAndroidVpnServiceGateRequiredArtifact::PackageIdentityProof,
        },
        ProtocolLiteralDescriptor {
            key: "VirtualInterfaceProof",
            value: NetworkAndroidVpnServiceGateRequiredArtifact::VirtualInterfaceProof,
        },
        ProtocolLiteralDescriptor {
            key: "TrafficObservationProof",
            value: NetworkAndroidVpnServiceGateRequiredArtifact::TrafficObservationProof,
        },
        ProtocolLiteralDescriptor {
            key: "RollbackPlan",
            value: NetworkAndroidVpnServiceGateRequiredArtifact::RollbackPlan,
        },
        ProtocolLiteralDescriptor {
            key: "AuditEvent",
            value: NetworkAndroidVpnServiceGateRequiredArtifact::AuditEvent,
        },
        ProtocolLiteralDescriptor {
            key: "DeviceOwnerProof",
            value: NetworkAndroidVpnServiceGateRequiredArtifact::DeviceOwnerProof,
        },
    ]
}

fn network_android_vpn_boundary_reason_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkAndroidVpnServiceGateBoundaryReason>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "ResearchOnlyRequested",
            value: NetworkAndroidVpnServiceGateBoundaryReason::ResearchOnlyRequested,
        },
        ProtocolLiteralDescriptor {
            key: "CapabilityManualRequired",
            value: NetworkAndroidVpnServiceGateBoundaryReason::CapabilityManualRequired,
        },
        ProtocolLiteralDescriptor {
            key: "CapabilityUnavailable",
            value: NetworkAndroidVpnServiceGateBoundaryReason::CapabilityUnavailable,
        },
        ProtocolLiteralDescriptor {
            key: "EvidenceGradeBelowProofThreshold",
            value: NetworkAndroidVpnServiceGateBoundaryReason::EvidenceGradeBelowProofThreshold,
        },
        ProtocolLiteralDescriptor {
            key: "PolicyNotVpnServiceApproved",
            value: NetworkAndroidVpnServiceGateBoundaryReason::PolicyNotVpnServiceApproved,
        },
        ProtocolLiteralDescriptor {
            key: "MissingRequiredArtifact",
            value: NetworkAndroidVpnServiceGateBoundaryReason::MissingRequiredArtifact,
        },
    ]
}

fn network_apple_network_extension_platform_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkAppleNetworkExtensionPlatformStatus>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "MacOs",
            value: NetworkAppleNetworkExtensionPlatformStatus::MacOs,
        },
        ProtocolLiteralDescriptor {
            key: "Ios",
            value: NetworkAppleNetworkExtensionPlatformStatus::Ios,
        },
    ]
}

fn network_apple_network_extension_capability_state_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkAppleNetworkExtensionGateCapabilityStatusState>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "AppleDeviceReady",
            value: NetworkAppleNetworkExtensionGateCapabilityStatusState::AppleDeviceReady,
        },
        ProtocolLiteralDescriptor {
            key: "ManualRequired",
            value: NetworkAppleNetworkExtensionGateCapabilityStatusState::ManualRequired,
        },
        ProtocolLiteralDescriptor {
            key: "Unavailable",
            value: NetworkAppleNetworkExtensionGateCapabilityStatusState::Unavailable,
        },
    ]
}

fn network_apple_network_extension_gate_state_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkAppleNetworkExtensionGateStatusState>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "ResearchOnly",
            value: NetworkAppleNetworkExtensionGateStatusState::ResearchOnly,
        },
        ProtocolLiteralDescriptor {
            key: "ManualRequired",
            value: NetworkAppleNetworkExtensionGateStatusState::ManualRequired,
        },
        ProtocolLiteralDescriptor {
            key: "Unavailable",
            value: NetworkAppleNetworkExtensionGateStatusState::Unavailable,
        },
        ProtocolLiteralDescriptor {
            key: "AppleEntitlementProofReady",
            value: NetworkAppleNetworkExtensionGateStatusState::AppleEntitlementProofReady,
        },
    ]
}

fn network_apple_network_extension_required_artifact_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkAppleNetworkExtensionGateRequiredArtifact>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "DeveloperTeamProof",
            value: NetworkAppleNetworkExtensionGateRequiredArtifact::DeveloperTeamProof,
        },
        ProtocolLiteralDescriptor {
            key: "EntitlementApprovalProof",
            value: NetworkAppleNetworkExtensionGateRequiredArtifact::EntitlementApprovalProof,
        },
        ProtocolLiteralDescriptor {
            key: "ProvisioningProfileProof",
            value: NetworkAppleNetworkExtensionGateRequiredArtifact::ProvisioningProfileProof,
        },
        ProtocolLiteralDescriptor {
            key: "SigningProof",
            value: NetworkAppleNetworkExtensionGateRequiredArtifact::SigningProof,
        },
        ProtocolLiteralDescriptor {
            key: "DeviceOrTestflightProof",
            value: NetworkAppleNetworkExtensionGateRequiredArtifact::DeviceOrTestflightProof,
        },
        ProtocolLiteralDescriptor {
            key: "NetworkExtensionDeclaration",
            value: NetworkAppleNetworkExtensionGateRequiredArtifact::NetworkExtensionDeclaration,
        },
        ProtocolLiteralDescriptor {
            key: "ExtensionConfigurationProof",
            value: NetworkAppleNetworkExtensionGateRequiredArtifact::ExtensionConfigurationProof,
        },
        ProtocolLiteralDescriptor {
            key: "RollbackPlan",
            value: NetworkAppleNetworkExtensionGateRequiredArtifact::RollbackPlan,
        },
        ProtocolLiteralDescriptor {
            key: "AuditEvent",
            value: NetworkAppleNetworkExtensionGateRequiredArtifact::AuditEvent,
        },
        ProtocolLiteralDescriptor {
            key: "SupervisionOrMdmProof",
            value: NetworkAppleNetworkExtensionGateRequiredArtifact::SupervisionOrMdmProof,
        },
    ]
}

fn network_apple_network_extension_boundary_reason_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkAppleNetworkExtensionGateBoundaryReason>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "ResearchOnlyRequested",
            value: NetworkAppleNetworkExtensionGateBoundaryReason::ResearchOnlyRequested,
        },
        ProtocolLiteralDescriptor {
            key: "CapabilityManualRequired",
            value: NetworkAppleNetworkExtensionGateBoundaryReason::CapabilityManualRequired,
        },
        ProtocolLiteralDescriptor {
            key: "CapabilityUnavailable",
            value: NetworkAppleNetworkExtensionGateBoundaryReason::CapabilityUnavailable,
        },
        ProtocolLiteralDescriptor {
            key: "EvidenceGradeBelowProofThreshold",
            value: NetworkAppleNetworkExtensionGateBoundaryReason::EvidenceGradeBelowProofThreshold,
        },
        ProtocolLiteralDescriptor {
            key: "PolicyNotNetworkExtensionApproved",
            value:
                NetworkAppleNetworkExtensionGateBoundaryReason::PolicyNotNetworkExtensionApproved,
        },
        ProtocolLiteralDescriptor {
            key: "MissingRequiredArtifact",
            value: NetworkAppleNetworkExtensionGateBoundaryReason::MissingRequiredArtifact,
        },
    ]
}
