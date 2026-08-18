use crate::screen_intelligence_router::{
    ScreenEvidenceCustodyState, ScreenManagedBrowserStructuredExtraction,
    ScreenStructuredExtractionAuthority, ScreenStructuredExtractionFallbackState,
    ScreenStructuredExtractionFreshness, ScreenStructuredExtractionRedactionState,
    ScreenStructuredExtractionState,
};

pub(super) fn is_consistent(value: &ScreenManagedBrowserStructuredExtraction) -> bool {
    !has_invalid_static_shape(value) && state_is_consistent(value)
}

pub(super) fn can_answer_policy(value: &ScreenManagedBrowserStructuredExtraction) -> bool {
    is_consistent(value) && ready_for_policy(value)
}

fn has_invalid_static_shape(value: &ScreenManagedBrowserStructuredExtraction) -> bool {
    value.source_id != crate::screen_intelligence_router::MANAGED_BROWSER_STRUCTURED_SOURCE_ID
        || value.authority != ScreenStructuredExtractionAuthority::ManagedBrowserCdp
        || value.source_identity_ref.trim().is_empty()
        || !value
            .source_identity_ref
            .starts_with(crate::screen_intelligence_router::MANAGED_BROWSER_TARGET_REF_PREFIX)
        || value.captured_at.trim().is_empty()
        || !value.url_title_metadata_captured
        || value.evidence_refs.len() < 3
        || value.evidence_refs.iter().any(|reference| {
            reference.evidence_id.trim().is_empty()
                || reference.digest.trim().is_empty()
                || reference.uri.is_some()
        })
        || !value.evidence_refs.iter().any(|reference| {
            reference.evidence_id == value.source_identity_ref
                && reference
                    .evidence_id
                    .starts_with(crate::screen_intelligence_router::MANAGED_BROWSER_TARGET_REF_PREFIX)
        })
        || !value.evidence_refs.iter().any(|reference| {
            reference
                .evidence_id
                .starts_with(crate::screen_intelligence_router::MANAGED_BROWSER_URL_REF_PREFIX)
        })
        || !value.evidence_refs.iter().any(|reference| {
            reference
                .evidence_id
                .starts_with(crate::screen_intelligence_router::MANAGED_BROWSER_TITLE_REF_PREFIX)
        })
        || value.raw_dom_included
        || value.visible_text_character_count
            > crate::screen_intelligence_router::SCREEN_MANAGED_BROWSER_STRUCTURED_TEXT_LIMIT
        || value.visible_text_summary.as_ref().is_some_and(|summary| {
            summary.len() > crate::screen_intelligence_router::SCREEN_MANAGED_BROWSER_STRUCTURED_TEXT_LIMIT
        })
        || !redaction_flags_are_consistent(value)
        || value.custody_state == ScreenEvidenceCustodyState::OcentraHostedNonActivity
}

fn ready_for_policy(value: &ScreenManagedBrowserStructuredExtraction) -> bool {
    [
        value.enough_for_policy,
        value.policy_question_answered,
        value.no_screen_needed,
        !value.screenshot_required,
        value.category_candidate.is_some(),
        value.freshness == ScreenStructuredExtractionFreshness::Fresh,
        value.fallback_state == ScreenStructuredExtractionFallbackState::NotRequired,
        value.redaction_state != ScreenStructuredExtractionRedactionState::ProtectedContentSkipped,
    ]
    .into_iter()
    .all(|value| value)
}

fn needs_screenshot_matches_state(value: &ScreenManagedBrowserStructuredExtraction) -> bool {
    !value.no_screen_needed
        && value.screenshot_required
        && matches!(
            value.fallback_state,
            ScreenStructuredExtractionFallbackState::ScreenshotRequired
                | ScreenStructuredExtractionFallbackState::Stale
        )
}

fn state_is_consistent(value: &ScreenManagedBrowserStructuredExtraction) -> bool {
    match value.extraction_state {
        ScreenStructuredExtractionState::EnoughForPolicy => ready_for_policy(value),
        ScreenStructuredExtractionState::NeedsScreenshot => needs_screenshot_matches_state(value),
        ScreenStructuredExtractionState::Unavailable => {
            !value.no_screen_needed
                && !value.screenshot_required
                && matches!(
                    value.fallback_state,
                    ScreenStructuredExtractionFallbackState::AuthorityUnavailable
                        | ScreenStructuredExtractionFallbackState::Stale
                        | ScreenStructuredExtractionFallbackState::RedactedEvidenceInsufficient
                )
        }
    }
}

fn redaction_flags_are_consistent(value: &ScreenManagedBrowserStructuredExtraction) -> bool {
    match value.redaction_state {
        ScreenStructuredExtractionRedactionState::None => {
            !value.private_content_redacted && !value.dom_overflow_redacted
        }
        ScreenStructuredExtractionRedactionState::PrivateTextRedacted => {
            value.private_content_redacted && !value.dom_overflow_redacted
        }
        ScreenStructuredExtractionRedactionState::OverflowRedacted => {
            !value.private_content_redacted && value.dom_overflow_redacted
        }
        ScreenStructuredExtractionRedactionState::ProtectedContentSkipped => true,
    }
}
