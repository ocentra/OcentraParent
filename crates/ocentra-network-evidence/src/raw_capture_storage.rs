use serde::{Deserialize, Serialize};

use crate::{
    NetworkLiveCaptureProof, NetworkLiveCaptureProofState, NetworkRetentionReadinessProof,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkRawCaptureArtifactKind {
    Pcap,
    PacketMetadataLog,
    AnalyzerInputSnapshot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkRawCaptureStorageState {
    CustodyReady,
    ManualRequired,
    Unavailable,
    Degraded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkRawCaptureStorageRequiredArtifact {
    LiveCaptureProofReady,
    RawArtifactManifest,
    StorageLocation,
    EncryptionAtRest,
    QuotaRotation,
    RetentionPolicy,
    DeleteExport,
    Custody,
    PrivateTrafficExclusion,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkRawCaptureStorageProofInput {
    pub storage_proof_ref: String,
    pub artifact_kind: NetworkRawCaptureArtifactKind,
    pub live_capture_proof: NetworkLiveCaptureProof,
    pub retention: NetworkRetentionReadinessProof,
    pub raw_artifact_manifest_ref: Option<String>,
    pub storage_location_ref: Option<String>,
    pub raw_artifact_touched: bool,
    pub encryption_at_rest_verified: bool,
    pub quota_rotation_verified: bool,
    pub retention_policy_verified: bool,
    pub delete_export_verified: bool,
    pub custody_chain_verified: bool,
    pub private_traffic_exclusion_verified: bool,
    pub remote_upload_claimed: bool,
    pub live_capture_execution_claimed: bool,
    pub raw_pcap_without_custody_claimed: bool,
    pub exact_url_claimed: bool,
    pub decrypted_payload_claimed: bool,
    pub page_content_claimed: bool,
    pub private_message_claimed: bool,
    pub search_query_claimed: bool,
    pub policy_authority_claimed: bool,
    pub adapter_authority_claimed: bool,
    pub enforcement_command_claimed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkRawCaptureStorageProof {
    pub storage_proof_ref: String,
    pub artifact_kind: NetworkRawCaptureArtifactKind,
    pub storage_state: NetworkRawCaptureStorageState,
    pub missing_required_artifacts: Vec<NetworkRawCaptureStorageRequiredArtifact>,
    pub live_capture_proof_ref: String,
    pub live_capture_state: NetworkLiveCaptureProofState,
    pub raw_artifact_manifest_ref: Option<String>,
    pub storage_location_ref: Option<String>,
    pub retention_refs: Vec<String>,
    pub raw_artifact_touched: bool,
    pub raw_artifact_stored: bool,
    pub encrypted_at_rest: bool,
    pub quota_rotation_governed: bool,
    pub retention_policy_governed: bool,
    pub delete_export_governed: bool,
    pub custody_governed: bool,
    pub private_traffic_exclusion_governed: bool,
    pub remote_upload_enabled: bool,
    pub live_capture_executed: bool,
    pub raw_pcap_without_custody_available: bool,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
    pub page_content_available: bool,
    pub private_message_available: bool,
    pub search_query_available: bool,
    pub policy_authority: bool,
    pub adapter_authority: bool,
    pub enforcement_commands_published: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkRawCaptureStorageProofError {
    EmptyStorageProofRef,
    EmptyLiveCaptureProofRef,
    EmptyRetentionRef,
    EmptyRawArtifactManifestRef,
    EmptyStorageLocationRef,
    LiveCaptureProofExecuted,
    RemoteUploadClaimRejected,
    LiveCaptureExecutionClaimRejected,
    RawPcapWithoutCustodyClaimRejected,
    ExactUrlClaimRejected,
    DecryptedPayloadClaimRejected,
    PageContentClaimRejected,
    PrivateMessageClaimRejected,
    SearchQueryClaimRejected,
    PolicyAuthorityClaimRejected,
    AdapterAuthorityClaimRejected,
    EnforcementCommandClaimRejected,
}

pub fn evaluate_network_raw_capture_storage_proof(
    input: NetworkRawCaptureStorageProofInput,
) -> Result<NetworkRawCaptureStorageProof, NetworkRawCaptureStorageProofError> {
    validate_input(&input)?;

    let missing_required_artifacts = missing_required_artifacts(&input);
    let storage_state = storage_state(&input.live_capture_proof, &missing_required_artifacts);
    let raw_artifact_stored =
        input.raw_artifact_touched && storage_state == NetworkRawCaptureStorageState::CustodyReady;

    Ok(NetworkRawCaptureStorageProof {
        storage_proof_ref: input.storage_proof_ref.trim().to_owned(),
        artifact_kind: input.artifact_kind,
        storage_state,
        missing_required_artifacts,
        live_capture_proof_ref: input.live_capture_proof.capture_proof_ref,
        live_capture_state: input.live_capture_proof.proof_state,
        raw_artifact_manifest_ref: normalized_optional_ref(input.raw_artifact_manifest_ref),
        storage_location_ref: normalized_optional_ref(input.storage_location_ref),
        retention_refs: retention_refs(input.retention),
        raw_artifact_touched: input.raw_artifact_touched,
        raw_artifact_stored,
        encrypted_at_rest: input.encryption_at_rest_verified,
        quota_rotation_governed: input.quota_rotation_verified,
        retention_policy_governed: input.retention_policy_verified,
        delete_export_governed: input.delete_export_verified,
        custody_governed: input.custody_chain_verified,
        private_traffic_exclusion_governed: input.private_traffic_exclusion_verified,
        remote_upload_enabled: false,
        live_capture_executed: false,
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
    input: &NetworkRawCaptureStorageProofInput,
) -> Result<(), NetworkRawCaptureStorageProofError> {
    if input.storage_proof_ref.trim().is_empty() {
        return Err(NetworkRawCaptureStorageProofError::EmptyStorageProofRef);
    }
    if input.live_capture_proof.capture_proof_ref.trim().is_empty() {
        return Err(NetworkRawCaptureStorageProofError::EmptyLiveCaptureProofRef);
    }
    validate_retention_refs(&input.retention)?;
    validate_optional_ref(
        input.raw_artifact_manifest_ref.as_deref(),
        NetworkRawCaptureStorageProofError::EmptyRawArtifactManifestRef,
    )?;
    validate_optional_ref(
        input.storage_location_ref.as_deref(),
        NetworkRawCaptureStorageProofError::EmptyStorageLocationRef,
    )?;
    if input.live_capture_proof.live_capture_executed {
        return Err(NetworkRawCaptureStorageProofError::LiveCaptureProofExecuted);
    }
    validate_claims(input)
}

fn validate_claims(
    input: &NetworkRawCaptureStorageProofInput,
) -> Result<(), NetworkRawCaptureStorageProofError> {
    if input.remote_upload_claimed {
        return Err(NetworkRawCaptureStorageProofError::RemoteUploadClaimRejected);
    }
    if input.live_capture_execution_claimed {
        return Err(NetworkRawCaptureStorageProofError::LiveCaptureExecutionClaimRejected);
    }
    if input.raw_pcap_without_custody_claimed {
        return Err(NetworkRawCaptureStorageProofError::RawPcapWithoutCustodyClaimRejected);
    }
    if input.exact_url_claimed {
        return Err(NetworkRawCaptureStorageProofError::ExactUrlClaimRejected);
    }
    if input.decrypted_payload_claimed {
        return Err(NetworkRawCaptureStorageProofError::DecryptedPayloadClaimRejected);
    }
    if input.page_content_claimed {
        return Err(NetworkRawCaptureStorageProofError::PageContentClaimRejected);
    }
    if input.private_message_claimed {
        return Err(NetworkRawCaptureStorageProofError::PrivateMessageClaimRejected);
    }
    if input.search_query_claimed {
        return Err(NetworkRawCaptureStorageProofError::SearchQueryClaimRejected);
    }
    if input.policy_authority_claimed {
        return Err(NetworkRawCaptureStorageProofError::PolicyAuthorityClaimRejected);
    }
    if input.adapter_authority_claimed {
        return Err(NetworkRawCaptureStorageProofError::AdapterAuthorityClaimRejected);
    }
    if input.enforcement_command_claimed {
        return Err(NetworkRawCaptureStorageProofError::EnforcementCommandClaimRejected);
    }
    Ok(())
}

fn missing_required_artifacts(
    input: &NetworkRawCaptureStorageProofInput,
) -> Vec<NetworkRawCaptureStorageRequiredArtifact> {
    let mut missing = Vec::new();
    push_missing(
        input.live_capture_proof.proof_state == NetworkLiveCaptureProofState::ProofReady,
        NetworkRawCaptureStorageRequiredArtifact::LiveCaptureProofReady,
        &mut missing,
    );
    push_missing(
        !input.raw_artifact_touched || input.raw_artifact_manifest_ref.is_some(),
        NetworkRawCaptureStorageRequiredArtifact::RawArtifactManifest,
        &mut missing,
    );
    push_missing(
        !input.raw_artifact_touched || input.storage_location_ref.is_some(),
        NetworkRawCaptureStorageRequiredArtifact::StorageLocation,
        &mut missing,
    );
    push_missing(
        input.encryption_at_rest_verified,
        NetworkRawCaptureStorageRequiredArtifact::EncryptionAtRest,
        &mut missing,
    );
    push_missing(
        input.quota_rotation_verified,
        NetworkRawCaptureStorageRequiredArtifact::QuotaRotation,
        &mut missing,
    );
    push_missing(
        input.retention_policy_verified,
        NetworkRawCaptureStorageRequiredArtifact::RetentionPolicy,
        &mut missing,
    );
    push_missing(
        input.delete_export_verified,
        NetworkRawCaptureStorageRequiredArtifact::DeleteExport,
        &mut missing,
    );
    push_missing(
        input.custody_chain_verified,
        NetworkRawCaptureStorageRequiredArtifact::Custody,
        &mut missing,
    );
    push_missing(
        input.private_traffic_exclusion_verified,
        NetworkRawCaptureStorageRequiredArtifact::PrivateTrafficExclusion,
        &mut missing,
    );
    missing
}

fn storage_state(
    proof: &NetworkLiveCaptureProof,
    missing: &[NetworkRawCaptureStorageRequiredArtifact],
) -> NetworkRawCaptureStorageState {
    match proof.proof_state {
        NetworkLiveCaptureProofState::Unavailable => NetworkRawCaptureStorageState::Unavailable,
        NetworkLiveCaptureProofState::Degraded => NetworkRawCaptureStorageState::Degraded,
        NetworkLiveCaptureProofState::ManualRequired => {
            NetworkRawCaptureStorageState::ManualRequired
        }
        NetworkLiveCaptureProofState::ProofReady => {
            if missing.is_empty() {
                NetworkRawCaptureStorageState::CustodyReady
            } else {
                NetworkRawCaptureStorageState::ManualRequired
            }
        }
    }
}

fn push_missing(
    present: bool,
    artifact: NetworkRawCaptureStorageRequiredArtifact,
    missing: &mut Vec<NetworkRawCaptureStorageRequiredArtifact>,
) {
    if !present {
        missing.push(artifact);
    }
}

fn validate_retention_refs(
    retention: &NetworkRetentionReadinessProof,
) -> Result<(), NetworkRawCaptureStorageProofError> {
    for value in retention_ref_slice(retention) {
        if value.trim().is_empty() {
            return Err(NetworkRawCaptureStorageProofError::EmptyRetentionRef);
        }
    }
    Ok(())
}

fn validate_optional_ref(
    value: Option<&str>,
    error: NetworkRawCaptureStorageProofError,
) -> Result<(), NetworkRawCaptureStorageProofError> {
    if value.is_some_and(|raw| raw.trim().is_empty()) {
        return Err(error);
    }
    Ok(())
}

fn retention_refs(retention: NetworkRetentionReadinessProof) -> Vec<String> {
    vec![
        retention.encryption_at_rest_ref,
        retention.quota_rotation_ref,
        retention.retention_policy_ref,
        retention.delete_export_ref,
        retention.custody_ref,
        retention.private_family_traffic_exclusion_ref,
    ]
}

fn retention_ref_slice(retention: &NetworkRetentionReadinessProof) -> [&str; 6] {
    [
        retention.encryption_at_rest_ref.as_str(),
        retention.quota_rotation_ref.as_str(),
        retention.retention_policy_ref.as_str(),
        retention.delete_export_ref.as_str(),
        retention.custody_ref.as_str(),
        retention.private_family_traffic_exclusion_ref.as_str(),
    ]
}

fn normalized_optional_ref(value: Option<String>) -> Option<String> {
    value.map(|raw| raw.trim().to_owned())
}
