use crate::screen_intelligence_router::{
    ScreenCaptureScope, ScreenIntelligencePolicySensitivity, ScreenIntelligenceRouteKind,
    ScreenIntelligenceRouteRequest, ScreenIntelligenceSourceKind,
};

pub(super) fn route_kind_for(
    request: &ScreenIntelligenceRouteRequest,
) -> ScreenIntelligenceRouteKind {
    if screen_capture_is_unsafe(request) {
        return ScreenIntelligenceRouteKind::Unavailable;
    }
    if request
        .structured_extraction
        .as_ref()
        .is_some_and(|value| value.no_screen_needed)
    {
        return ScreenIntelligenceRouteKind::NoScreenNeeded;
    }
    if request.source_kind == ScreenIntelligenceSourceKind::ManagedBrowser
        && request.parent_allows_managed_browser_structured_extraction
    {
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
) -> Option<String> {
    request
        .structured_extraction
        .as_ref()
        .map(|value| value.extraction_id.clone())
}

fn screen_capture_is_unsafe(request: &ScreenIntelligenceRouteRequest) -> bool {
    [
        request.protected_surface_suspected,
        request.credential_prompt_suspected,
        request.policy_sensitivity == ScreenIntelligencePolicySensitivity::ProtectedSurface,
        request.policy_sensitivity == ScreenIntelligencePolicySensitivity::CredentialRisk,
    ]
    .into_iter()
    .all(|value| value)
}

fn preferred_capture_scope(scopes: &[ScreenCaptureScope]) -> Option<&ScreenCaptureScope> {
    scopes
        .iter()
        .find(|scope| **scope == ScreenCaptureScope::ActiveWindow)
        .or_else(|| {
            scopes
                .iter()
                .find(|scope| **scope == ScreenCaptureScope::SelectedWindow)
        })
}
