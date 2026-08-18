use crate::screen_intelligence_router::{
    ScreenIntelligenceRouteKind, ScreenIntelligenceRouteRequest,
    ScreenStructuredExtractionFallbackState,
};

use super::consistency;

pub(super) fn structured_extraction_fallback_state_for(
    request: &ScreenIntelligenceRouteRequest,
    route_kind: &ScreenIntelligenceRouteKind,
) -> ScreenStructuredExtractionFallbackState {
    if let Some(safety_blocked_state) = safety_blocked_fallback_state_for(request, route_kind) {
        return safety_blocked_state;
    }
    if matches!(
        route_kind,
        ScreenIntelligenceRouteKind::ManagedBrowserStructuredExtraction
    ) {
        return ScreenStructuredExtractionFallbackState::NotAttempted;
    }
    let Some(extraction) = request.structured_extraction.as_ref() else {
        if consistency::managed_browser_structured_extraction_producer_is_unavailable(request) {
            return ScreenStructuredExtractionFallbackState::AuthorityUnavailable;
        }
        return ScreenStructuredExtractionFallbackState::NotAttempted;
    };

    if matches!(route_kind, ScreenIntelligenceRouteKind::NoScreenNeeded)
        && extraction.can_answer_policy()
    {
        return ScreenStructuredExtractionFallbackState::NotRequired;
    }
    if extraction.protected_content_skipped() {
        return ScreenStructuredExtractionFallbackState::RedactedEvidenceInsufficient;
    }
    if extraction.is_stale() {
        return ScreenStructuredExtractionFallbackState::Stale;
    }
    if extraction.is_unavailable()
        || consistency::managed_browser_structured_extraction_producer_is_unavailable(request)
    {
        return ScreenStructuredExtractionFallbackState::AuthorityUnavailable;
    }
    if extraction.requires_screenshot() {
        return ScreenStructuredExtractionFallbackState::ScreenshotRequired;
    }
    ScreenStructuredExtractionFallbackState::ScreenshotRequired
}

fn safety_blocked_fallback_state_for(
    request: &ScreenIntelligenceRouteRequest,
    route_kind: &ScreenIntelligenceRouteKind,
) -> Option<ScreenStructuredExtractionFallbackState> {
    if !matches!(route_kind, ScreenIntelligenceRouteKind::Unavailable) {
        return None;
    }
    if request
        .structured_extraction
        .as_ref()
        .is_some_and(|value| value.protected_content_skipped())
    {
        return Some(ScreenStructuredExtractionFallbackState::RedactedEvidenceInsufficient);
    }
    if consistency::screen_capture_is_unsafe(request) {
        return Some(ScreenStructuredExtractionFallbackState::AuthorityUnavailable);
    }
    if request.source_kind
        == crate::screen_intelligence_router::ScreenIntelligenceSourceKind::ManagedBrowser
    {
        return Some(ScreenStructuredExtractionFallbackState::AuthorityUnavailable);
    }
    None
}
