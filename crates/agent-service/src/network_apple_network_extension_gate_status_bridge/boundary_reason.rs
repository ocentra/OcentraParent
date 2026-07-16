use ocentra_network_evidence::apple_network_extension_gate::NetworkAppleNetworkExtensionGateBoundaryReason;
use ocentra_parent_agent_protocol::network_apple_network_extension_gate_status::NetworkAppleNetworkExtensionGateBoundaryReason as ProtocolNetworkAppleNetworkExtensionGateBoundaryReason;

pub(super) fn boundary_reason(
    reason: &NetworkAppleNetworkExtensionGateBoundaryReason,
) -> ProtocolNetworkAppleNetworkExtensionGateBoundaryReason {
    match reason {
        NetworkAppleNetworkExtensionGateBoundaryReason::ResearchOnlyRequested => {
            ProtocolNetworkAppleNetworkExtensionGateBoundaryReason::ResearchOnlyRequested
        }
        NetworkAppleNetworkExtensionGateBoundaryReason::CapabilityManualRequired => {
            ProtocolNetworkAppleNetworkExtensionGateBoundaryReason::CapabilityManualRequired
        }
        NetworkAppleNetworkExtensionGateBoundaryReason::CapabilityUnavailable => {
            ProtocolNetworkAppleNetworkExtensionGateBoundaryReason::CapabilityUnavailable
        }
        NetworkAppleNetworkExtensionGateBoundaryReason::EvidenceGradeBelowProofThreshold => {
            ProtocolNetworkAppleNetworkExtensionGateBoundaryReason::EvidenceGradeBelowProofThreshold
        }
        NetworkAppleNetworkExtensionGateBoundaryReason::PolicyNotNetworkExtensionApproved => {
            ProtocolNetworkAppleNetworkExtensionGateBoundaryReason::PolicyNotNetworkExtensionApproved
        }
        NetworkAppleNetworkExtensionGateBoundaryReason::MissingRequiredArtifact => {
            ProtocolNetworkAppleNetworkExtensionGateBoundaryReason::MissingRequiredArtifact
        }
    }
}
