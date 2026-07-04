use super::{UnmanagedBrowserCorrelationError, UnmanagedBrowserCorrelationInput};

pub(super) fn validate_unmanaged_browser_input(
    input: &UnmanagedBrowserCorrelationInput,
) -> Result<(), UnmanagedBrowserCorrelationError> {
    if input.observation_ref.trim().is_empty() {
        return Err(UnmanagedBrowserCorrelationError::EmptyObservationRef);
    }
    [
        (
            input.process_name.as_ref(),
            UnmanagedBrowserCorrelationError::EmptyProcessName,
        ),
        (
            input.redacted_executable_path_ref.as_ref(),
            UnmanagedBrowserCorrelationError::EmptyRedactedExecutablePathRef,
        ),
        (
            input.signature_ref.as_ref(),
            UnmanagedBrowserCorrelationError::EmptySignatureRef,
        ),
        (
            input.hash_ref.as_ref(),
            UnmanagedBrowserCorrelationError::EmptyHashRef,
        ),
        (
            input.browser_family.as_ref(),
            UnmanagedBrowserCorrelationError::EmptyBrowserFamily,
        ),
        (
            input.possible_bypass_reason_ref.as_ref(),
            UnmanagedBrowserCorrelationError::EmptyPossibleBypassReasonRef,
        ),
    ]
    .into_iter()
    .try_for_each(|(value, error)| validate_optional_ref(value, error))?;
    if let Some(confidence) = input.confidence.filter(|confidence| *confidence > 100) {
        return Err(UnmanagedBrowserCorrelationError::InvalidConfidence(
            confidence,
        ));
    }
    validate_unmanaged_browser_non_claims(input)
}

fn validate_unmanaged_browser_non_claims(
    input: &UnmanagedBrowserCorrelationInput,
) -> Result<(), UnmanagedBrowserCorrelationError> {
    [
        (
            input.exact_url_claimed,
            UnmanagedBrowserCorrelationError::UnsupportedExactUrlClaim,
        ),
        (
            input.active_tab_claimed,
            UnmanagedBrowserCorrelationError::UnsupportedActiveTabClaim,
        ),
        (
            input.page_title_claimed,
            UnmanagedBrowserCorrelationError::UnsupportedPageTitleClaim,
        ),
        (
            input.page_content_claimed,
            UnmanagedBrowserCorrelationError::UnsupportedPageContentClaim,
        ),
        (
            input.decrypted_payload_claimed,
            UnmanagedBrowserCorrelationError::UnsupportedDecryptedPayloadClaim,
        ),
        (
            input.policy_action_authority,
            UnmanagedBrowserCorrelationError::UnsupportedPolicyAuthorityClaim,
        ),
        (
            input.adapter_action_authorized,
            UnmanagedBrowserCorrelationError::UnsupportedAdapterAuthorityClaim,
        ),
        (
            input.enforcement_command_published,
            UnmanagedBrowserCorrelationError::UnsupportedEnforcementCommandClaim,
        ),
    ]
    .into_iter()
    .find_map(|(claimed, error)| claimed.then_some(error))
    .map_or(Ok(()), Err)
}

fn validate_optional_ref(
    value: Option<&String>,
    error: UnmanagedBrowserCorrelationError,
) -> Result<(), UnmanagedBrowserCorrelationError> {
    value
        .is_some_and(|value| value.trim().is_empty())
        .then_some(error)
        .map_or(Ok(()), Err)
}
