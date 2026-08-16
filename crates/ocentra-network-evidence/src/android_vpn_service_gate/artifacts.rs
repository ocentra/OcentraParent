use super::{NetworkAndroidVpnServiceArtifactRefs, NetworkAndroidVpnServiceRequiredArtifact};

pub(super) fn missing_required_artifacts(
    artifacts: &NetworkAndroidVpnServiceArtifactRefs,
    device_owner_required: bool,
) -> Vec<NetworkAndroidVpnServiceRequiredArtifact> {
    let mut missing = Vec::new();
    push_missing(
        &mut missing,
        artifacts.vpn_service_declaration_ref.as_ref(),
        NetworkAndroidVpnServiceRequiredArtifact::VpnServiceDeclaration,
    );
    push_missing(
        &mut missing,
        artifacts.user_consent_proof_ref.as_ref(),
        NetworkAndroidVpnServiceRequiredArtifact::UserConsentProof,
    );
    push_missing(
        &mut missing,
        artifacts.physical_device_proof_ref.as_ref(),
        NetworkAndroidVpnServiceRequiredArtifact::PhysicalDeviceProof,
    );
    push_missing(
        &mut missing,
        artifacts.package_identity_proof_ref.as_ref(),
        NetworkAndroidVpnServiceRequiredArtifact::PackageIdentityProof,
    );
    push_missing(
        &mut missing,
        artifacts.virtual_interface_proof_ref.as_ref(),
        NetworkAndroidVpnServiceRequiredArtifact::VirtualInterfaceProof,
    );
    push_missing(
        &mut missing,
        artifacts.traffic_observation_proof_ref.as_ref(),
        NetworkAndroidVpnServiceRequiredArtifact::TrafficObservationProof,
    );
    push_missing(
        &mut missing,
        artifacts.rollback_plan_ref.as_ref(),
        NetworkAndroidVpnServiceRequiredArtifact::RollbackPlan,
    );
    push_missing(
        &mut missing,
        artifacts.audit_event_ref.as_ref(),
        NetworkAndroidVpnServiceRequiredArtifact::AuditEvent,
    );
    if device_owner_required {
        push_missing(
            &mut missing,
            artifacts.device_owner_proof_ref.as_ref(),
            NetworkAndroidVpnServiceRequiredArtifact::DeviceOwnerProof,
        );
    }
    missing
}

fn push_missing(
    missing: &mut Vec<NetworkAndroidVpnServiceRequiredArtifact>,
    value: Option<&String>,
    artifact: NetworkAndroidVpnServiceRequiredArtifact,
) {
    if value.is_none() {
        missing.push(artifact);
    }
}
