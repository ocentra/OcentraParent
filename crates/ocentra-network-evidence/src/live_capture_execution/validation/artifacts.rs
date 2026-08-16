use super::{NetworkLiveCaptureExecutionInput, NetworkLiveCaptureExecutionRequiredArtifact};
use crate::live_capture::NetworkLiveCaptureProofState;

pub(super) fn missing_artifacts(
    input: &NetworkLiveCaptureExecutionInput,
) -> Vec<NetworkLiveCaptureExecutionRequiredArtifact> {
    let mut missing = Vec::new();
    require(
        input.live_capture_proof.proof_state == NetworkLiveCaptureProofState::ProofReady,
        NetworkLiveCaptureExecutionRequiredArtifact::ProofReadyLiveCapture,
        &mut missing,
    );
    require_execution_artifacts(input, &mut missing);
    require_custody_artifacts(input, &mut missing);
    missing
}

fn require_execution_artifacts(
    input: &NetworkLiveCaptureExecutionInput,
    missing: &mut Vec<NetworkLiveCaptureExecutionRequiredArtifact>,
) {
    require_artifact(
        input.driver_invoked,
        input.driver_invocation_ref.as_deref(),
        NetworkLiveCaptureExecutionRequiredArtifact::DriverInvocation,
        missing,
    );
    require_artifact(
        input.live_capture_executed,
        input.interface_observation_ref.as_deref(),
        NetworkLiveCaptureExecutionRequiredArtifact::InterfaceObservation,
        missing,
    );
    require_artifact(
        input.live_capture_executed,
        input.permission_ref.as_deref(),
        NetworkLiveCaptureExecutionRequiredArtifact::Permission,
        missing,
    );
    require_artifact(
        input.live_capture_executed,
        input.bounded_window_ref.as_deref(),
        NetworkLiveCaptureExecutionRequiredArtifact::BoundedWindow,
        missing,
    );
    require_artifact(
        input.live_capture_executed,
        input.clean_stop_ref.as_deref(),
        NetworkLiveCaptureExecutionRequiredArtifact::CleanStop,
        missing,
    );
}

fn require_custody_artifacts(
    input: &NetworkLiveCaptureExecutionInput,
    missing: &mut Vec<NetworkLiveCaptureExecutionRequiredArtifact>,
) {
    require_artifact(
        input.live_capture_executed,
        input.custody_ref.as_deref(),
        NetworkLiveCaptureExecutionRequiredArtifact::Custody,
        missing,
    );
    require_artifact(
        input.live_capture_executed,
        input.retention_delete_export_ref.as_deref(),
        NetworkLiveCaptureExecutionRequiredArtifact::RetentionDeleteExport,
        missing,
    );
    require_artifact(
        input.live_capture_executed,
        input.metadata_only_sanitization_ref.as_deref(),
        NetworkLiveCaptureExecutionRequiredArtifact::MetadataOnlySanitization,
        missing,
    );
    require_artifact(
        input.live_capture_executed,
        input.private_traffic_exclusion_ref.as_deref(),
        NetworkLiveCaptureExecutionRequiredArtifact::PrivateTrafficExclusion,
        missing,
    );
}

fn require_artifact(
    condition: bool,
    artifact_ref: Option<&str>,
    artifact: NetworkLiveCaptureExecutionRequiredArtifact,
    missing: &mut Vec<NetworkLiveCaptureExecutionRequiredArtifact>,
) {
    require(condition && artifact_ref.is_some(), artifact, missing);
}

fn require(
    condition: bool,
    artifact: NetworkLiveCaptureExecutionRequiredArtifact,
    missing: &mut Vec<NetworkLiveCaptureExecutionRequiredArtifact>,
) {
    if !condition {
        missing.push(artifact);
    }
}
