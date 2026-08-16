use crate::NetworkAppleNetworkExtensionRequiredArtifact;

pub(super) fn apple_artifact_label(
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
