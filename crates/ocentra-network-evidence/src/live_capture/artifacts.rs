use super::{NetworkLiveCaptureProofInput, NetworkLiveCaptureRequiredArtifact};

pub(super) fn missing_artifacts(
    input: &NetworkLiveCaptureProofInput,
) -> Vec<NetworkLiveCaptureRequiredArtifact> {
    let mut missing = Vec::new();
    require(
        input.driver_available && input.driver_proof_ref.is_some(),
        NetworkLiveCaptureRequiredArtifact::DriverProof,
        &mut missing,
    );
    require(
        input.interface_enumerated && input.interface_ref.is_some(),
        NetworkLiveCaptureRequiredArtifact::InterfaceEnumeration,
        &mut missing,
    );
    require(
        input.permission_granted && input.permission_proof_ref.is_some(),
        NetworkLiveCaptureRequiredArtifact::PermissionProof,
        &mut missing,
    );
    require(
        input.bounded_capture_succeeded && input.bounded_capture_ref.is_some(),
        NetworkLiveCaptureRequiredArtifact::BoundedCaptureProof,
        &mut missing,
    );
    require(
        input.clean_stop_succeeded && input.clean_stop_ref.is_some(),
        NetworkLiveCaptureRequiredArtifact::CleanStopProof,
        &mut missing,
    );
    require(
        input.quota_rotation_ref.is_some(),
        NetworkLiveCaptureRequiredArtifact::QuotaRotationProof,
        &mut missing,
    );
    require(
        input.retention_delete_export_ref.is_some(),
        NetworkLiveCaptureRequiredArtifact::RetentionDeleteExportProof,
        &mut missing,
    );
    require(
        input.custody_ref.is_some(),
        NetworkLiveCaptureRequiredArtifact::CustodyProof,
        &mut missing,
    );
    require(
        input.private_traffic_exclusion_ref.is_some(),
        NetworkLiveCaptureRequiredArtifact::PrivateTrafficExclusionProof,
        &mut missing,
    );
    missing
}

fn require(
    condition: bool,
    artifact: NetworkLiveCaptureRequiredArtifact,
    missing: &mut Vec<NetworkLiveCaptureRequiredArtifact>,
) {
    if !condition {
        missing.push(artifact);
    }
}
