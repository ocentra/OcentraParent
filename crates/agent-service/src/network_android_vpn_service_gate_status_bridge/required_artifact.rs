use ocentra_network_evidence::android_vpn_service_gate::NetworkAndroidVpnServiceRequiredArtifact;
use ocentra_parent_agent_protocol::network_android_vpn_service_gate_status::NetworkAndroidVpnServiceGateRequiredArtifact as ProtocolNetworkAndroidVpnServiceGateRequiredArtifact;

pub(super) fn required_artifact(
    artifact: &NetworkAndroidVpnServiceRequiredArtifact,
) -> ProtocolNetworkAndroidVpnServiceGateRequiredArtifact {
    match artifact {
        NetworkAndroidVpnServiceRequiredArtifact::VpnServiceDeclaration => {
            ProtocolNetworkAndroidVpnServiceGateRequiredArtifact::VpnServiceDeclaration
        }
        NetworkAndroidVpnServiceRequiredArtifact::UserConsentProof => {
            ProtocolNetworkAndroidVpnServiceGateRequiredArtifact::UserConsentProof
        }
        NetworkAndroidVpnServiceRequiredArtifact::PhysicalDeviceProof => {
            ProtocolNetworkAndroidVpnServiceGateRequiredArtifact::PhysicalDeviceProof
        }
        NetworkAndroidVpnServiceRequiredArtifact::PackageIdentityProof => {
            ProtocolNetworkAndroidVpnServiceGateRequiredArtifact::PackageIdentityProof
        }
        NetworkAndroidVpnServiceRequiredArtifact::VirtualInterfaceProof => {
            ProtocolNetworkAndroidVpnServiceGateRequiredArtifact::VirtualInterfaceProof
        }
        NetworkAndroidVpnServiceRequiredArtifact::TrafficObservationProof => {
            ProtocolNetworkAndroidVpnServiceGateRequiredArtifact::TrafficObservationProof
        }
        NetworkAndroidVpnServiceRequiredArtifact::RollbackPlan => {
            ProtocolNetworkAndroidVpnServiceGateRequiredArtifact::RollbackPlan
        }
        NetworkAndroidVpnServiceRequiredArtifact::AuditEvent => {
            ProtocolNetworkAndroidVpnServiceGateRequiredArtifact::AuditEvent
        }
        NetworkAndroidVpnServiceRequiredArtifact::DeviceOwnerProof => {
            ProtocolNetworkAndroidVpnServiceGateRequiredArtifact::DeviceOwnerProof
        }
    }
}
