use crate::live_capture::{NetworkLiveCaptureProof, NetworkLiveCaptureProofState};

mod types;

pub use types::{
    NetworkRawCaptureStorageError, NetworkRawCaptureStorageInput, NetworkRawCaptureStorageProof,
    NetworkRawCaptureStorageRequiredArtifact, NetworkRawCaptureStorageState,
};

pub fn plan_network_raw_capture_storage(
    input: NetworkRawCaptureStorageInput,
) -> Result<NetworkRawCaptureStorageProof, NetworkRawCaptureStorageError> {
    validate_input(&input)?;

    let missing_artifacts = missing_artifacts(&input);
    let storage_state = storage_state(&input.live_capture_proof, &missing_artifacts);
    let raw_artifact_storage_authorized =
        storage_state == NetworkRawCaptureStorageState::CustodyReady;

    Ok(NetworkRawCaptureStorageProof {
        storage_proof_ref: input.storage_proof_ref,
        live_capture_proof_ref: input.live_capture_proof.capture_proof_ref,
        live_capture_state: input.live_capture_proof.proof_state,
        storage_state,
        missing_artifacts,
        raw_artifact_manifest_ref: input.raw_artifact_manifest_ref,
        storage_location_ref: input.storage_location_ref,
        encryption_at_rest_ref: input.encryption_at_rest_ref,
        quota_rotation_ref: input.quota_rotation_ref,
        retention_policy_ref: input.retention_policy_ref,
        delete_export_ref: input.delete_export_ref,
        custody_chain_ref: input.custody_chain_ref,
        private_traffic_exclusion_ref: input.private_traffic_exclusion_ref,
        raw_artifact_storage_authorized,
        live_capture_executed: false,
        remote_upload_enabled: false,
        raw_pcap_without_custody_available: false,
        exact_url_available: false,
        decrypted_payload_available: false,
        page_content_available: false,
        private_message_available: false,
        search_query_available: false,
        policy_authority: false,
        adapter_authority: false,
        enforcement_commands_published: 0,
    })
}

fn validate_input(
    input: &NetworkRawCaptureStorageInput,
) -> Result<(), NetworkRawCaptureStorageError> {
    if input.storage_proof_ref.trim().is_empty() {
        return Err(NetworkRawCaptureStorageError::EmptyStorageProofRef);
    }
    for artifact_ref in artifact_refs(input).into_iter().flatten() {
        if artifact_ref.trim().is_empty() {
            return Err(NetworkRawCaptureStorageError::EmptyArtifactRef);
        }
    }
    validate_claims(input)
}

fn validate_claims(
    input: &NetworkRawCaptureStorageInput,
) -> Result<(), NetworkRawCaptureStorageError> {
    if input.live_capture_execution_claimed {
        return Err(NetworkRawCaptureStorageError::LiveCaptureExecutionClaimRejected);
    }
    if input.remote_upload_claimed {
        return Err(NetworkRawCaptureStorageError::RemoteUploadClaimRejected);
    }
    if input.raw_pcap_without_custody_claimed {
        return Err(NetworkRawCaptureStorageError::RawPcapWithoutCustodyClaimRejected);
    }
    if input.exact_url_claimed {
        return Err(NetworkRawCaptureStorageError::ExactUrlClaimRejected);
    }
    if input.decrypted_payload_claimed {
        return Err(NetworkRawCaptureStorageError::DecryptedPayloadClaimRejected);
    }
    if input.page_content_claimed {
        return Err(NetworkRawCaptureStorageError::PageContentClaimRejected);
    }
    if input.private_message_claimed {
        return Err(NetworkRawCaptureStorageError::PrivateMessageClaimRejected);
    }
    if input.search_query_claimed {
        return Err(NetworkRawCaptureStorageError::SearchQueryClaimRejected);
    }
    if input.policy_authority_claimed {
        return Err(NetworkRawCaptureStorageError::PolicyAuthorityClaimRejected);
    }
    if input.adapter_authority_claimed {
        return Err(NetworkRawCaptureStorageError::AdapterAuthorityClaimRejected);
    }
    if input.enforcement_command_claimed {
        return Err(NetworkRawCaptureStorageError::EnforcementCommandClaimRejected);
    }
    Ok(())
}

fn storage_state(
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

fn missing_artifacts(
    input: &NetworkRawCaptureStorageInput,
) -> Vec<NetworkRawCaptureStorageRequiredArtifact> {
    let mut missing = Vec::new();
    require(
        input.live_capture_proof.proof_state == NetworkLiveCaptureProofState::ProofReady,
        NetworkRawCaptureStorageRequiredArtifact::LiveCaptureProof,
        &mut missing,
    );
    require_artifact(
        input.raw_artifact_manifest_available,
        input.raw_artifact_manifest_ref.as_deref(),
        NetworkRawCaptureStorageRequiredArtifact::RawArtifactManifest,
        &mut missing,
    );
    require_artifact(
        input.storage_location_available,
        input.storage_location_ref.as_deref(),
        NetworkRawCaptureStorageRequiredArtifact::StorageLocation,
        &mut missing,
    );
    require_artifact(
        input.encryption_at_rest_verified,
        input.encryption_at_rest_ref.as_deref(),
        NetworkRawCaptureStorageRequiredArtifact::EncryptionAtRest,
        &mut missing,
    );
    require_artifact(
        input.quota_rotation_verified,
        input.quota_rotation_ref.as_deref(),
        NetworkRawCaptureStorageRequiredArtifact::QuotaRotation,
        &mut missing,
    );
    require_artifact(
        input.retention_policy_verified,
        input.retention_policy_ref.as_deref(),
        NetworkRawCaptureStorageRequiredArtifact::RetentionPolicy,
        &mut missing,
    );
    require_artifact(
        input.delete_export_verified,
        input.delete_export_ref.as_deref(),
        NetworkRawCaptureStorageRequiredArtifact::DeleteExport,
        &mut missing,
    );
    require_artifact(
        input.custody_chain_verified,
        input.custody_chain_ref.as_deref(),
        NetworkRawCaptureStorageRequiredArtifact::CustodyChain,
        &mut missing,
    );
    require_artifact(
        input.private_traffic_exclusion_verified,
        input.private_traffic_exclusion_ref.as_deref(),
        NetworkRawCaptureStorageRequiredArtifact::PrivateTrafficExclusion,
        &mut missing,
    );
    missing
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

fn artifact_refs(input: &NetworkRawCaptureStorageInput) -> [Option<&str>; 8] {
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
