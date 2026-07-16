use super::{NetworkScreenSummaryTriggerError, NetworkScreenSummaryTriggerInput};

pub(super) fn validate_screen_summary_trigger_input(
    input: &NetworkScreenSummaryTriggerInput,
) -> Result<(), NetworkScreenSummaryTriggerError> {
    [
        (
            normalize_ref(&input.queue_job_ref).is_none(),
            NetworkScreenSummaryTriggerError::EmptyQueueJobRef,
        ),
        (
            normalize_ref(&input.screen_queue_ref).is_none(),
            NetworkScreenSummaryTriggerError::EmptyScreenQueueRef,
        ),
        (
            normalize_ref(&input.parent_setting_ref).is_none(),
            NetworkScreenSummaryTriggerError::EmptyParentSettingRef,
        ),
        (
            normalize_ref(&input.retention_policy_ref).is_none(),
            NetworkScreenSummaryTriggerError::EmptyRetentionPolicyRef,
        ),
    ]
    .into_iter()
    .find_map(|(invalid, error)| invalid.then_some(error))
    .map_or_else(|| validate_screen_summary_non_claims(input), Err)
}

pub(super) fn normalize_ref(value: &str) -> Option<String> {
    let trimmed = value.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_owned())
}

fn validate_screen_summary_non_claims(
    input: &NetworkScreenSummaryTriggerInput,
) -> Result<(), NetworkScreenSummaryTriggerError> {
    [
        (
            input.raw_image_retention_requested,
            NetworkScreenSummaryTriggerError::RawImageRetentionRejected,
        ),
        (
            input.remote_upload_requested,
            NetworkScreenSummaryTriggerError::RemoteUploadRejected,
        ),
        (
            input.screen_content_available,
            NetworkScreenSummaryTriggerError::ScreenContentRejected,
        ),
        (
            input.bundle.decrypted_payload_available,
            NetworkScreenSummaryTriggerError::DecryptedPayloadRejected,
        ),
        (
            input.policy_action_authority || input.bundle.policy_action_authority,
            NetworkScreenSummaryTriggerError::PolicyAuthorityRejected,
        ),
        (
            input.adapter_action_authority || input.bundle.adapter_action_authorized,
            NetworkScreenSummaryTriggerError::AdapterAuthorityRejected,
        ),
        (
            input.enforcement_command_published,
            NetworkScreenSummaryTriggerError::EnforcementCommandRejected,
        ),
    ]
    .into_iter()
    .find_map(|(claimed, error)| claimed.then_some(error))
    .map_or(Ok(()), Err)
}
