use crate::NetworkAndroidVpnServiceRequiredArtifact;

pub(super) fn android_artifact_label(
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
