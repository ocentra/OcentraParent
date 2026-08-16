use super::{
    requirements::artifact_refs, NetworkRawCaptureStorageError, NetworkRawCaptureStorageInput,
};

pub(super) fn validate_input(
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
    [
        (
            input.live_capture_execution_claimed,
            NetworkRawCaptureStorageError::LiveCaptureExecutionClaimRejected,
        ),
        (
            input.remote_upload_claimed,
            NetworkRawCaptureStorageError::RemoteUploadClaimRejected,
        ),
        (
            input.raw_pcap_without_custody_claimed,
            NetworkRawCaptureStorageError::RawPcapWithoutCustodyClaimRejected,
        ),
        (
            input.exact_url_claimed,
            NetworkRawCaptureStorageError::ExactUrlClaimRejected,
        ),
        (
            input.decrypted_payload_claimed,
            NetworkRawCaptureStorageError::DecryptedPayloadClaimRejected,
        ),
        (
            input.page_content_claimed,
            NetworkRawCaptureStorageError::PageContentClaimRejected,
        ),
        (
            input.private_message_claimed,
            NetworkRawCaptureStorageError::PrivateMessageClaimRejected,
        ),
        (
            input.search_query_claimed,
            NetworkRawCaptureStorageError::SearchQueryClaimRejected,
        ),
        (
            input.policy_authority_claimed,
            NetworkRawCaptureStorageError::PolicyAuthorityClaimRejected,
        ),
        (
            input.adapter_authority_claimed,
            NetworkRawCaptureStorageError::AdapterAuthorityClaimRejected,
        ),
        (
            input.enforcement_command_claimed,
            NetworkRawCaptureStorageError::EnforcementCommandClaimRejected,
        ),
    ]
    .into_iter()
    .find_map(|(claimed, error)| claimed.then_some(error))
    .map_or(Ok(()), Err)
}
