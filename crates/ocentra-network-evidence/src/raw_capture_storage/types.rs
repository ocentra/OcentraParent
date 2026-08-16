use serde::{Deserialize, Serialize};

use crate::live_capture::{NetworkLiveCaptureProof, NetworkLiveCaptureProofState};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkRawCaptureStorageState {
    CustodyReady,
    ManualRequired,
    Unavailable,
    Degraded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkRawCaptureStorageRequiredArtifact {
    LiveCaptureProof,
    RawArtifactManifest,
    StorageLocation,
    EncryptionAtRest,
    QuotaRotation,
    RetentionPolicy,
    DeleteExport,
    CustodyChain,
    PrivateTrafficExclusion,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkRawCaptureStorageInput {
    pub storage_proof_ref: String,
    pub live_capture_proof: NetworkLiveCaptureProof,
    pub raw_artifact_manifest_ref: Option<String>,
    pub storage_location_ref: Option<String>,
    pub encryption_at_rest_ref: Option<String>,
    pub quota_rotation_ref: Option<String>,
    pub retention_policy_ref: Option<String>,
    pub delete_export_ref: Option<String>,
    pub custody_chain_ref: Option<String>,
    pub private_traffic_exclusion_ref: Option<String>,
    pub raw_artifact_manifest_available: bool,
    pub storage_location_available: bool,
    pub encryption_at_rest_verified: bool,
    pub quota_rotation_verified: bool,
    pub retention_policy_verified: bool,
    pub delete_export_verified: bool,
    pub custody_chain_verified: bool,
    pub private_traffic_exclusion_verified: bool,
    pub live_capture_execution_claimed: bool,
    pub remote_upload_claimed: bool,
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
    pub live_capture_proof_ref: String,
    pub live_capture_state: NetworkLiveCaptureProofState,
    pub storage_state: NetworkRawCaptureStorageState,
    pub missing_artifacts: Vec<NetworkRawCaptureStorageRequiredArtifact>,
    pub raw_artifact_manifest_ref: Option<String>,
    pub storage_location_ref: Option<String>,
    pub encryption_at_rest_ref: Option<String>,
    pub quota_rotation_ref: Option<String>,
    pub retention_policy_ref: Option<String>,
    pub delete_export_ref: Option<String>,
    pub custody_chain_ref: Option<String>,
    pub private_traffic_exclusion_ref: Option<String>,
    pub raw_artifact_storage_authorized: bool,
    pub live_capture_executed: bool,
    pub remote_upload_enabled: bool,
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
pub enum NetworkRawCaptureStorageError {
    EmptyStorageProofRef,
    EmptyArtifactRef,
    LiveCaptureExecutionClaimRejected,
    RemoteUploadClaimRejected,
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
