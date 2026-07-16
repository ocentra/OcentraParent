use super::{
    ManagedBrowserCorrelationError, ManagedBrowserCorrelationInput, ManagedBrowserPageEvidence,
};

pub(super) fn validate_managed_browser_input(
    input: &ManagedBrowserCorrelationInput,
) -> Result<(), ManagedBrowserCorrelationError> {
    if input.network_flow.flow_ref.trim().is_empty() {
        return Err(ManagedBrowserCorrelationError::EmptyFlowRef);
    }
    if input
        .network_flow
        .observed_domain
        .as_ref()
        .is_some_and(|domain| domain.trim().is_empty())
    {
        return Err(ManagedBrowserCorrelationError::EmptyObservedDomain);
    }
    if let Some(browser) = &input.managed_browser {
        validate_browser_evidence(browser)?;
    }
    Ok(())
}

pub(super) fn domains_match(network_domain: &Option<String>, browser_domain: &str) -> bool {
    network_domain
        .as_deref()
        .map(str::trim)
        .is_some_and(|domain| domain.eq_ignore_ascii_case(browser_domain.trim()))
}

fn validate_browser_evidence(
    browser: &ManagedBrowserPageEvidence,
) -> Result<(), ManagedBrowserCorrelationError> {
    [
        (
            browser.browser_ref.trim().is_empty(),
            ManagedBrowserCorrelationError::EmptyBrowserRef,
        ),
        (
            browser.tab_ref.trim().is_empty(),
            ManagedBrowserCorrelationError::EmptyTabRef,
        ),
        (
            browser.page_url.trim().is_empty(),
            ManagedBrowserCorrelationError::EmptyPageUrl,
        ),
        (
            browser.page_domain.trim().is_empty(),
            ManagedBrowserCorrelationError::EmptyPageDomain,
        ),
        (
            browser.source_ref.trim().is_empty(),
            ManagedBrowserCorrelationError::EmptyBrowserSourceRef,
        ),
    ]
    .into_iter()
    .find_map(|(invalid, error)| invalid.then_some(error))
    .map_or(Ok(()), Err)
}
