use crate::screen_intelligence_router::{
    ScreenIntelligenceRouteKind, ScreenIntelligenceRouteRequest,
    ScreenStructuredExtractionFallbackState,
};

pub(super) fn structured_extraction_fallback_state_for(
    request: &ScreenIntelligenceRouteRequest,
    route_kind: &ScreenIntelligenceRouteKind,
) -> ScreenStructuredExtractionFallbackState {
    let Some(extraction) = request.structured_extraction.as_ref() else {
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
    if extraction.requires_screenshot() {
        return ScreenStructuredExtractionFallbackState::ScreenshotRequired;
    }
    if extraction.is_unavailable() {
        return ScreenStructuredExtractionFallbackState::AuthorityUnavailable;
    }
    ScreenStructuredExtractionFallbackState::ScreenshotRequired
}
