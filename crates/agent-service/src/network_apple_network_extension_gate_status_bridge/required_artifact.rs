use ocentra_network_evidence::apple_network_extension_gate::NetworkAppleNetworkExtensionRequiredArtifact;
use ocentra_parent_agent_protocol::network_apple_network_extension_gate_status::NetworkAppleNetworkExtensionGateRequiredArtifact as ProtocolNetworkAppleNetworkExtensionGateRequiredArtifact;

pub(super) fn required_artifact(
    artifact: &NetworkAppleNetworkExtensionRequiredArtifact,
) -> ProtocolNetworkAppleNetworkExtensionGateRequiredArtifact {
    match artifact {
        NetworkAppleNetworkExtensionRequiredArtifact::DeveloperTeamProof => {
            ProtocolNetworkAppleNetworkExtensionGateRequiredArtifact::DeveloperTeamProof
        }
        NetworkAppleNetworkExtensionRequiredArtifact::EntitlementApprovalProof => {
            ProtocolNetworkAppleNetworkExtensionGateRequiredArtifact::EntitlementApprovalProof
        }
        NetworkAppleNetworkExtensionRequiredArtifact::ProvisioningProfileProof => {
            ProtocolNetworkAppleNetworkExtensionGateRequiredArtifact::ProvisioningProfileProof
        }
        NetworkAppleNetworkExtensionRequiredArtifact::SigningProof => {
            ProtocolNetworkAppleNetworkExtensionGateRequiredArtifact::SigningProof
        }
        NetworkAppleNetworkExtensionRequiredArtifact::DeviceOrTestFlightProof => {
            ProtocolNetworkAppleNetworkExtensionGateRequiredArtifact::DeviceOrTestflightProof
        }
        NetworkAppleNetworkExtensionRequiredArtifact::NetworkExtensionDeclaration => {
            ProtocolNetworkAppleNetworkExtensionGateRequiredArtifact::NetworkExtensionDeclaration
        }
        NetworkAppleNetworkExtensionRequiredArtifact::ExtensionConfigurationProof => {
            ProtocolNetworkAppleNetworkExtensionGateRequiredArtifact::ExtensionConfigurationProof
        }
        NetworkAppleNetworkExtensionRequiredArtifact::RollbackPlan => {
            ProtocolNetworkAppleNetworkExtensionGateRequiredArtifact::RollbackPlan
        }
        NetworkAppleNetworkExtensionRequiredArtifact::AuditEvent => {
            ProtocolNetworkAppleNetworkExtensionGateRequiredArtifact::AuditEvent
        }
        NetworkAppleNetworkExtensionRequiredArtifact::SupervisionOrMdmProof => {
            ProtocolNetworkAppleNetworkExtensionGateRequiredArtifact::SupervisionOrMdmProof
        }
    }
}
