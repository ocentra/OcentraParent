use ocentra_network_evidence::android_vpn_service_gate::NetworkAndroidVpnServiceGateBoundaryReason;
use ocentra_parent_agent_protocol::network_android_vpn_service_gate_status::NetworkAndroidVpnServiceGateBoundaryReason as ProtocolNetworkAndroidVpnServiceGateBoundaryReason;

pub(super) fn boundary_reason(
    reason: &NetworkAndroidVpnServiceGateBoundaryReason,
) -> ProtocolNetworkAndroidVpnServiceGateBoundaryReason {
    match reason {
        NetworkAndroidVpnServiceGateBoundaryReason::ResearchOnlyRequested => {
            ProtocolNetworkAndroidVpnServiceGateBoundaryReason::ResearchOnlyRequested
        }
        NetworkAndroidVpnServiceGateBoundaryReason::CapabilityManualRequired => {
            ProtocolNetworkAndroidVpnServiceGateBoundaryReason::CapabilityManualRequired
        }
        NetworkAndroidVpnServiceGateBoundaryReason::CapabilityUnavailable => {
            ProtocolNetworkAndroidVpnServiceGateBoundaryReason::CapabilityUnavailable
        }
        NetworkAndroidVpnServiceGateBoundaryReason::EvidenceGradeBelowProofThreshold => {
            ProtocolNetworkAndroidVpnServiceGateBoundaryReason::EvidenceGradeBelowProofThreshold
        }
        NetworkAndroidVpnServiceGateBoundaryReason::PolicyNotVpnServiceApproved => {
            ProtocolNetworkAndroidVpnServiceGateBoundaryReason::PolicyNotVpnServiceApproved
        }
        NetworkAndroidVpnServiceGateBoundaryReason::MissingRequiredArtifact => {
            ProtocolNetworkAndroidVpnServiceGateBoundaryReason::MissingRequiredArtifact
        }
    }
}
