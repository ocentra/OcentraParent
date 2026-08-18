use crate::screen_intelligence_router::{
    ScreenIntelligenceRouteRequest, ScreenStructuredExtractionAuthority,
    ScreenStructuredExtractionFallbackState, ScreenStructuredExtractionFreshness,
    ScreenStructuredExtractionRedactionState, ScreenStructuredExtractionState,
};

pub(super) fn structured_extraction_fallback_state_for(
    request: &ScreenIntelligenceRouteRequest,
) -> ScreenStructuredExtractionFallbackState {
    let Some(extraction) = request.structured_extraction.as_ref() else {
        return ScreenStructuredExtractionFallbackState::NotAttempted;
    };
    if extraction.no_screen_needed
        && extraction.authority == ScreenStructuredExtractionAuthority::ManagedBrowserCdp
        && extraction.freshness == ScreenStructuredExtractionFreshness::Fresh
        && extraction.redaction_state
            != ScreenStructuredExtractionRedactionState::ProtectedContentSkipped
    {
        return if request.parent_allows_managed_browser_structured_extraction {
            ScreenStructuredExtractionFallbackState::NotRequired
        } else {
            ScreenStructuredExtractionFallbackState::ScreenshotRequired
        };
    }
    if extraction.freshness == ScreenStructuredExtractionFreshness::Stale {
        return ScreenStructuredExtractionFallbackState::Stale;
    }
    match extraction.extraction_state {
        ScreenStructuredExtractionState::NeedsScreenshot => {
            ScreenStructuredExtractionFallbackState::ScreenshotRequired
        }
        ScreenStructuredExtractionState::Unavailable => {
            if extraction.authority != ScreenStructuredExtractionAuthority::ManagedBrowserCdp {
                ScreenStructuredExtractionFallbackState::AuthorityUnavailable
            } else if extraction.redaction_state
                == ScreenStructuredExtractionRedactionState::ProtectedContentSkipped
            {
                ScreenStructuredExtractionFallbackState::RedactedEvidenceInsufficient
            } else {
                ScreenStructuredExtractionFallbackState::AuthorityUnavailable
            }
        }
        ScreenStructuredExtractionState::EnoughForPolicy => {
            ScreenStructuredExtractionFallbackState::RedactedEvidenceInsufficient
        }
    }
}
