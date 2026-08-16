use super::{NetworkLiveCaptureProofError, NetworkLiveCaptureProofInput};

mod claims;

pub(super) fn validate_input(
    input: &NetworkLiveCaptureProofInput,
) -> Result<(), NetworkLiveCaptureProofError> {
    if input.capture_proof_ref.trim().is_empty() {
        return Err(NetworkLiveCaptureProofError::EmptyCaptureProofRef);
    }
    for artifact_ref in artifact_refs(input).into_iter().flatten() {
        if artifact_ref.trim().is_empty() {
            return Err(NetworkLiveCaptureProofError::EmptyArtifactRef);
        }
    }
    claims::validate_input(input)
}

fn artifact_refs(input: &NetworkLiveCaptureProofInput) -> [Option<&str>; 9] {
    [
        input.interface_ref.as_deref(),
        input.driver_proof_ref.as_deref(),
        input.permission_proof_ref.as_deref(),
        input.bounded_capture_ref.as_deref(),
        input.clean_stop_ref.as_deref(),
        input.quota_rotation_ref.as_deref(),
        input.retention_delete_export_ref.as_deref(),
        input.custody_ref.as_deref(),
        input.private_traffic_exclusion_ref.as_deref(),
    ]
}
