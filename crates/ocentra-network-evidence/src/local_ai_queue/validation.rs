use super::{NetworkLocalAiQueueError, NetworkLocalAiQueueInput, NetworkLocalAiQueueStatus};

pub(super) fn validate_refs(
    input: &NetworkLocalAiQueueInput,
) -> Result<(), NetworkLocalAiQueueError> {
    super::refs::normalize_ref(&input.queue_job_ref)
        .ok_or(NetworkLocalAiQueueError::EmptyQueueJobRef)?;
    super::refs::normalize_ref(&input.queue_ref).ok_or(NetworkLocalAiQueueError::EmptyQueueRef)?;
    super::refs::normalize_ref(&input.model_runtime_ref)
        .ok_or(NetworkLocalAiQueueError::EmptyModelRuntimeRef)?;
    super::refs::normalized_summary_refs(&input.summary_refs)?;
    Ok(())
}

pub(super) fn validate_no_claims(
    input: &NetworkLocalAiQueueInput,
) -> Result<(), NetworkLocalAiQueueError> {
    [
        (
            input.raw_network_payload_available,
            NetworkLocalAiQueueError::RawNetworkPayloadRejected,
        ),
        (
            input.page_content_available,
            NetworkLocalAiQueueError::PageContentRejected,
        ),
        (
            input.bundle.decrypted_payload_available,
            NetworkLocalAiQueueError::DecryptedPayloadRejected,
        ),
        (
            input.policy_action_authority || input.bundle.policy_action_authority,
            NetworkLocalAiQueueError::PolicyAuthorityRejected,
        ),
        (
            input.adapter_action_authority || input.bundle.adapter_action_authorized,
            NetworkLocalAiQueueError::AdapterAuthorityRejected,
        ),
    ]
    .into_iter()
    .find_map(|(claimed, error)| claimed.then_some(error))
    .map_or(Ok(()), Err)
}

pub(super) fn queue_status(input: &NetworkLocalAiQueueInput) -> NetworkLocalAiQueueStatus {
    if !input.bundle.local_ai_review_recommended {
        return NetworkLocalAiQueueStatus::NotRecommended;
    }
    if !input.local_ai_enabled {
        return NetworkLocalAiQueueStatus::DisabledByParent;
    }
    if !input.model_runtime_available {
        return NetworkLocalAiQueueStatus::ModelUnavailable;
    }
    if !input.queue_available {
        return NetworkLocalAiQueueStatus::QueueUnavailable;
    }
    NetworkLocalAiQueueStatus::Queued
}
