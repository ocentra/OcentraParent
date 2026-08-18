use crate::screen_intelligence_router::{
    ScreenCaptureScope, ScreenIntelligenceRouteKind, ScreenIntelligenceRouteRequest,
};

pub(super) fn capture_scope_for_route(
    request: &ScreenIntelligenceRouteRequest,
    route_kind: &ScreenIntelligenceRouteKind,
) -> Option<ScreenCaptureScope> {
    match route_kind {
        ScreenIntelligenceRouteKind::ScreenCaptureActiveWindow
        | ScreenIntelligenceRouteKind::ScreenCaptureSelectedWindow => {
            preferred_capture_scope(&request.allowed_capture_scopes).cloned()
        }
        _ => None,
    }
}

pub(super) fn structured_extraction_for_route(
    request: &ScreenIntelligenceRouteRequest,
    route_kind: &ScreenIntelligenceRouteKind,
) -> Option<String> {
    if !matches!(route_kind, ScreenIntelligenceRouteKind::NoScreenNeeded) {
        return None;
    }
    request
        .structured_extraction
        .as_ref()
        .map(|value| value.extraction_id().to_owned())
}

pub(super) fn preferred_capture_scope(
    scopes: &[ScreenCaptureScope],
) -> Option<&ScreenCaptureScope> {
    scopes
        .iter()
        .find(|scope| **scope == ScreenCaptureScope::ActiveWindow)
        .or_else(|| {
            scopes
                .iter()
                .find(|scope| **scope == ScreenCaptureScope::SelectedWindow)
        })
}
