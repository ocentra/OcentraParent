use super::route_projection::preferred_capture_scope;
use crate::screen_intelligence_router::{
    ScreenCaptureScope, ScreenIntelligenceRouteKind, ScreenIntelligenceRouteRequest,
    ScreenIntelligenceSourceKind,
};
use crate::screen_intelligence_router_logic::consistency;

pub(super) fn route_kind_for(
    request: &ScreenIntelligenceRouteRequest,
) -> ScreenIntelligenceRouteKind {
    if !consistency::screen_intelligence_route_request_is_consistent(request) {
        return ScreenIntelligenceRouteKind::Unavailable;
    }
    if consistency::screen_capture_is_unsafe(request) {
        return ScreenIntelligenceRouteKind::Unavailable;
    }
    if request
        .structured_extraction
        .as_ref()
        .is_some_and(|value| value.protected_content_skipped())
    {
        return ScreenIntelligenceRouteKind::Unavailable;
    }
    if request.source_kind == ScreenIntelligenceSourceKind::ManagedBrowser
        && request.parent_allows_managed_browser_structured_extraction
    {
        // NoScreenNeeded remains unavailable until a policy owner issues an
        // affirmative safe-disclosure classification; ReviewRequired evidence
        // must not be promoted by this router.
        if request.structured_extraction.as_ref().is_some_and(|value| {
            consistency::screen_managed_browser_structured_extraction_is_ready_for_route(value)
        }) {
            return ScreenIntelligenceRouteKind::ManagedBrowserStructuredExtraction;
        }
        // No service-owned handoff means the browser producer boundary is
        // unavailable; never advertise a structured route without evidence.
        return ScreenIntelligenceRouteKind::Unavailable;
    }
    if request.source_kind == ScreenIntelligenceSourceKind::ManagedBrowser {
        // Do not fall through to desktop capture when the service cannot
        // compose the managed-browser owner handoff.
        return ScreenIntelligenceRouteKind::Unavailable;
    }
    if !request.parent_allows_screen_capture {
        return ScreenIntelligenceRouteKind::ManualRequired;
    }
    if matches!(
        request.source_kind,
        ScreenIntelligenceSourceKind::NetworkOrSessionSummary
            | ScreenIntelligenceSourceKind::ScreenAdjacentEvidence
    ) {
        return ScreenIntelligenceRouteKind::ManualRequired;
    }
    match preferred_capture_scope(&request.allowed_capture_scopes) {
        Some(ScreenCaptureScope::ActiveWindow) => {
            ScreenIntelligenceRouteKind::ScreenCaptureActiveWindow
        }
        Some(ScreenCaptureScope::SelectedWindow) => {
            ScreenIntelligenceRouteKind::ScreenCaptureSelectedWindow
        }
        _ => ScreenIntelligenceRouteKind::ManualRequired,
    }
}
