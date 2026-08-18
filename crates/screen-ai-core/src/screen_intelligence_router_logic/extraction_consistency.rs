use crate::screen_intelligence_router::ScreenManagedBrowserStructuredExtraction;

pub(super) fn is_consistent(value: &ScreenManagedBrowserStructuredExtraction) -> bool {
    value.authority_is_managed_browser()
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
