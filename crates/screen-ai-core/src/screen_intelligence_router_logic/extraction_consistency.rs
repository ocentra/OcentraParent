use crate::screen_intelligence_router::ScreenManagedBrowserStructuredExtraction;

pub(super) fn is_consistent(value: &ScreenManagedBrowserStructuredExtraction) -> bool {
    value.is_verified()
        && value.authority_is_managed_browser()
        && !value.extraction_id().trim().is_empty()
        && !value.captured_at().trim().is_empty()
        && !value.evidence_refs().is_empty()
}

pub(super) fn can_answer_policy(value: &ScreenManagedBrowserStructuredExtraction) -> bool {
    is_consistent(value) && value.can_answer_policy()
}

pub(super) fn protected_content_skipped(value: &ScreenManagedBrowserStructuredExtraction) -> bool {
    value.protected_content_skipped()
}

pub(super) fn is_ready_for_structured_route(
    value: &ScreenManagedBrowserStructuredExtraction,
) -> bool {
    is_consistent(value)
        && value.is_fresh()
        && (value.has_structured_evidence() || value.requires_review())
        && !value.protected_content_skipped()
        && !value.is_unavailable()
}
