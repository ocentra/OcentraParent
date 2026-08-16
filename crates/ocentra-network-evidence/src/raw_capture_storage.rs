use crate::live_capture::{NetworkLiveCaptureProof, NetworkLiveCaptureProofState};

mod requirements;
pub mod types;
mod validation;

use self::types::{
    NetworkRawCaptureStorageError, NetworkRawCaptureStorageInput, NetworkRawCaptureStorageProof,
    NetworkRawCaptureStorageRequiredArtifact, NetworkRawCaptureStorageState,
};
use self::{
    requirements::{missing_artifacts, storage_state},
    validation::validate_input,
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
