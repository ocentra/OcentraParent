use super::route_projection::preferred_capture_scope;
use crate::screen_intelligence_router::{
    ScreenCaptureScope, ScreenIntelligenceRouteKind, ScreenIntelligenceRouteRequest,
    ScreenIntelligenceSourceKind,
};
use crate::screen_intelligence_router_logic::consistency;

pub(super) fn route_kind_for(
    request: &ScreenIntelligenceRouteRequest,
) -> ScreenIntelligenceRouteKind {
    if consistency::screen_capture_is_unsafe(request) {
        return ScreenIntelligenceRouteKind::Unavailable;
    }
    if request
        .structured_extraction
        .as_ref()
        .is_some_and(|value| consistency::protected_content_skipped(value))
    {
        return ScreenIntelligenceRouteKind::Unavailable;
    }
    if request.source_kind == ScreenIntelligenceSourceKind::ManagedBrowser
        && request.parent_allows_managed_browser_structured_extraction
        && request.structured_extraction.as_ref().is_some_and(|value| {
            consistency::screen_managed_browser_structured_extraction_can_answer_policy(value)
        })
    {
        return ScreenIntelligenceRouteKind::NoScreenNeeded;
    }
    if managed_browser_structured_extraction_should_precede_capture(request) {
        return ScreenIntelligenceRouteKind::ManagedBrowserStructuredExtraction;
    }
    if !request.parent_allows_screen_capture {
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

fn managed_browser_structured_extraction_should_precede_capture(
    request: &ScreenIntelligenceRouteRequest,
) -> bool {
    // With no owner-issued receipt, this is only the extraction-first handoff;
    // the no-screen route requires a verified receipt above.
    request.source_kind == ScreenIntelligenceSourceKind::ManagedBrowser
        && request.parent_allows_managed_browser_structured_extraction
        && request.structured_extraction.is_none()
}
