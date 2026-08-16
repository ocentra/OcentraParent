use super::{
    NetworkLiveCaptureProof, NetworkLiveCaptureProofState, NetworkRawCaptureStorageInput,
    NetworkRawCaptureStorageRequiredArtifact, NetworkRawCaptureStorageState,
};

pub(super) fn storage_state(
    live_capture_proof: &NetworkLiveCaptureProof,
    missing_artifacts: &[NetworkRawCaptureStorageRequiredArtifact],
) -> NetworkRawCaptureStorageState {
    match live_capture_proof.proof_state {
        NetworkLiveCaptureProofState::Unavailable => NetworkRawCaptureStorageState::Unavailable,
        NetworkLiveCaptureProofState::Degraded => NetworkRawCaptureStorageState::Degraded,
        NetworkLiveCaptureProofState::ProofReady if missing_artifacts.is_empty() => {
            NetworkRawCaptureStorageState::CustodyReady
        }
        NetworkLiveCaptureProofState::ProofReady | NetworkLiveCaptureProofState::ManualRequired => {
            NetworkRawCaptureStorageState::ManualRequired
        }
    }
}

pub(super) fn missing_artifacts(
    input: &NetworkRawCaptureStorageInput,
) -> Vec<NetworkRawCaptureStorageRequiredArtifact> {
    let mut missing = Vec::new();
    require(
        input.live_capture_proof.proof_state == NetworkLiveCaptureProofState::ProofReady,
        NetworkRawCaptureStorageRequiredArtifact::LiveCaptureProof,
        &mut missing,
    );
    [
        (
            input.raw_artifact_manifest_available,
            input.raw_artifact_manifest_ref.as_deref(),
            NetworkRawCaptureStorageRequiredArtifact::RawArtifactManifest,
        ),
        (
            input.storage_location_available,
            input.storage_location_ref.as_deref(),
            NetworkRawCaptureStorageRequiredArtifact::StorageLocation,
        ),
        (
            input.encryption_at_rest_verified,
            input.encryption_at_rest_ref.as_deref(),
            NetworkRawCaptureStorageRequiredArtifact::EncryptionAtRest,
        ),
        (
            input.quota_rotation_verified,
            input.quota_rotation_ref.as_deref(),
            NetworkRawCaptureStorageRequiredArtifact::QuotaRotation,
        ),
        (
            input.retention_policy_verified,
            input.retention_policy_ref.as_deref(),
            NetworkRawCaptureStorageRequiredArtifact::RetentionPolicy,
        ),
        (
            input.delete_export_verified,
            input.delete_export_ref.as_deref(),
            NetworkRawCaptureStorageRequiredArtifact::DeleteExport,
        ),
        (
            input.custody_chain_verified,
            input.custody_chain_ref.as_deref(),
            NetworkRawCaptureStorageRequiredArtifact::CustodyChain,
        ),
        (
            input.private_traffic_exclusion_verified,
            input.private_traffic_exclusion_ref.as_deref(),
            NetworkRawCaptureStorageRequiredArtifact::PrivateTrafficExclusion,
        ),
    ]
    .into_iter()
    .for_each(|(condition, artifact_ref, artifact)| {
        require_artifact(condition, artifact_ref, artifact, &mut missing);
    });
    missing
}

pub(super) fn artifact_refs(input: &NetworkRawCaptureStorageInput) -> [Option<&str>; 8] {
    [
        input.raw_artifact_manifest_ref.as_deref(),
        input.storage_location_ref.as_deref(),
        input.encryption_at_rest_ref.as_deref(),
        input.quota_rotation_ref.as_deref(),
        input.retention_policy_ref.as_deref(),
        input.delete_export_ref.as_deref(),
        input.custody_chain_ref.as_deref(),
        input.private_traffic_exclusion_ref.as_deref(),
    ]
}

fn require_artifact(
    condition: bool,
    artifact_ref: Option<&str>,
    artifact: NetworkRawCaptureStorageRequiredArtifact,
    missing: &mut Vec<NetworkRawCaptureStorageRequiredArtifact>,
) {
    require(condition && artifact_ref.is_some(), artifact, missing);
}

fn require(
    condition: bool,
    artifact: NetworkRawCaptureStorageRequiredArtifact,
    missing: &mut Vec<NetworkRawCaptureStorageRequiredArtifact>,
) {
    if !condition {
        missing.push(artifact);
    }
}
