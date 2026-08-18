use crate::screen_intelligence_router::{
    ScreenIntelligenceRouteKind, ScreenIntelligenceRouteRequest,
    ScreenStructuredExtractionFallbackState,
};

use super::consistency;

pub(super) fn structured_extraction_fallback_state_for(
    request: &ScreenIntelligenceRouteRequest,
    route_kind: &ScreenIntelligenceRouteKind,
) -> ScreenStructuredExtractionFallbackState {
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
