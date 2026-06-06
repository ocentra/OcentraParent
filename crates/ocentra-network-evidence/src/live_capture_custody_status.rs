use crate::{
    NetworkLiveCaptureProof, NetworkLiveCaptureProofState, NetworkLiveCaptureRequiredArtifact,
    NetworkRawCaptureStorageProof, NetworkRawCaptureStorageRequiredArtifact,
    NetworkRawCaptureStorageState,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkLiveCaptureCustodyStatusState {
    CustodyReady,
    ManualRequired,
    Unavailable,
    Degraded,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkLiveCaptureCustodyStatusMissingArtifact {
    LiveCapture(NetworkLiveCaptureRequiredArtifact),
    RawCaptureStorage(NetworkRawCaptureStorageRequiredArtifact),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkLiveCaptureCustodyStatusInput {
    pub status_ref: String,
    pub live_capture_proof: NetworkLiveCaptureProof,
    pub raw_capture_storage_proof: NetworkRawCaptureStorageProof,
    pub live_capture_execution_claimed: bool,
    pub raw_artifact_creation_claimed: bool,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkLiveCaptureCustodyStatus {
    pub status_ref: String,
    pub live_capture_proof_ref: String,
    pub raw_capture_storage_proof_ref: String,
    pub state: NetworkLiveCaptureCustodyStatusState,
    pub live_capture_state: NetworkLiveCaptureProofState,
    pub raw_capture_storage_state: NetworkRawCaptureStorageState,
    pub missing_artifacts: Vec<NetworkLiveCaptureCustodyStatusMissingArtifact>,
    pub capture_ready: bool,
    pub raw_artifact_storage_authorized: bool,
    pub driver_invoked: bool,
    pub live_capture_executed: bool,
    pub raw_artifact_created: bool,
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
pub enum NetworkLiveCaptureCustodyStatusError {
    EmptyStatusRef,
    MismatchedLiveCaptureProofRef,
    LiveCaptureExecutionClaimRejected,
    RawArtifactCreationClaimRejected,
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

pub fn materialize_network_live_capture_custody_status(
    input: NetworkLiveCaptureCustodyStatusInput,
) -> Result<NetworkLiveCaptureCustodyStatus, NetworkLiveCaptureCustodyStatusError> {
    validate_input(&input)?;

    let missing_artifacts = missing_artifacts(&input);
    let state = status_state(&input);

    Ok(NetworkLiveCaptureCustodyStatus {
        status_ref: input.status_ref,
        live_capture_proof_ref: input.live_capture_proof.capture_proof_ref,
        raw_capture_storage_proof_ref: input.raw_capture_storage_proof.storage_proof_ref,
        state,
        live_capture_state: input.live_capture_proof.proof_state,
        raw_capture_storage_state: input.raw_capture_storage_proof.storage_state,
        missing_artifacts,
        capture_ready: input.live_capture_proof.capture_ready,
        raw_artifact_storage_authorized: input
            .raw_capture_storage_proof
            .raw_artifact_storage_authorized,
        driver_invoked: false,
        live_capture_executed: false,
        raw_artifact_created: false,
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
    input: &NetworkLiveCaptureCustodyStatusInput,
) -> Result<(), NetworkLiveCaptureCustodyStatusError> {
    if input.status_ref.trim().is_empty() {
        return Err(NetworkLiveCaptureCustodyStatusError::EmptyStatusRef);
    }
    if input.live_capture_proof.capture_proof_ref
        != input.raw_capture_storage_proof.live_capture_proof_ref
    {
        return Err(NetworkLiveCaptureCustodyStatusError::MismatchedLiveCaptureProofRef);
    }
    validate_claims(input)
}

fn validate_claims(
    input: &NetworkLiveCaptureCustodyStatusInput,
) -> Result<(), NetworkLiveCaptureCustodyStatusError> {
    if input.live_capture_execution_claimed {
        return Err(NetworkLiveCaptureCustodyStatusError::LiveCaptureExecutionClaimRejected);
    }
    if input.raw_artifact_creation_claimed {
        return Err(NetworkLiveCaptureCustodyStatusError::RawArtifactCreationClaimRejected);
    }
    if input.remote_upload_claimed {
        return Err(NetworkLiveCaptureCustodyStatusError::RemoteUploadClaimRejected);
    }
    if input.raw_pcap_without_custody_claimed {
        return Err(NetworkLiveCaptureCustodyStatusError::RawPcapWithoutCustodyClaimRejected);
    }
    if input.exact_url_claimed {
        return Err(NetworkLiveCaptureCustodyStatusError::ExactUrlClaimRejected);
    }
    if input.decrypted_payload_claimed {
        return Err(NetworkLiveCaptureCustodyStatusError::DecryptedPayloadClaimRejected);
    }
    if input.page_content_claimed {
        return Err(NetworkLiveCaptureCustodyStatusError::PageContentClaimRejected);
    }
    if input.private_message_claimed {
        return Err(NetworkLiveCaptureCustodyStatusError::PrivateMessageClaimRejected);
    }
    if input.search_query_claimed {
        return Err(NetworkLiveCaptureCustodyStatusError::SearchQueryClaimRejected);
    }
    if input.policy_authority_claimed {
        return Err(NetworkLiveCaptureCustodyStatusError::PolicyAuthorityClaimRejected);
    }
    if input.adapter_authority_claimed {
        return Err(NetworkLiveCaptureCustodyStatusError::AdapterAuthorityClaimRejected);
    }
    if input.enforcement_command_claimed {
        return Err(NetworkLiveCaptureCustodyStatusError::EnforcementCommandClaimRejected);
    }
    Ok(())
}

fn status_state(
    input: &NetworkLiveCaptureCustodyStatusInput,
) -> NetworkLiveCaptureCustodyStatusState {
    match (
        input.live_capture_proof.proof_state,
        input.raw_capture_storage_proof.storage_state,
    ) {
        (NetworkLiveCaptureProofState::Unavailable, _)
        | (_, NetworkRawCaptureStorageState::Unavailable) => {
            NetworkLiveCaptureCustodyStatusState::Unavailable
        }
        (NetworkLiveCaptureProofState::Degraded, _)
        | (_, NetworkRawCaptureStorageState::Degraded) => {
            NetworkLiveCaptureCustodyStatusState::Degraded
        }
        (NetworkLiveCaptureProofState::ProofReady, NetworkRawCaptureStorageState::CustodyReady) => {
            NetworkLiveCaptureCustodyStatusState::CustodyReady
        }
        _ => NetworkLiveCaptureCustodyStatusState::ManualRequired,
    }
}

fn missing_artifacts(
    input: &NetworkLiveCaptureCustodyStatusInput,
) -> Vec<NetworkLiveCaptureCustodyStatusMissingArtifact> {
    let live_capture_missing = input
        .live_capture_proof
        .missing_artifacts
        .iter()
        .copied()
        .map(NetworkLiveCaptureCustodyStatusMissingArtifact::LiveCapture);
    let raw_storage_missing = input
        .raw_capture_storage_proof
        .missing_artifacts
        .iter()
        .copied()
        .map(NetworkLiveCaptureCustodyStatusMissingArtifact::RawCaptureStorage);

    live_capture_missing.chain(raw_storage_missing).collect()
}
