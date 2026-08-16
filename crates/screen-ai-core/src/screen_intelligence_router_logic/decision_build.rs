use super::route_kind::{capture_scope_for_route, route_kind_for, structured_extraction_for_route};
use crate::screen_intelligence_router::{
    ScreenEvidenceCustodyState, ScreenIntelligencePolicySensitivity,
    ScreenIntelligenceRouteDecision, ScreenIntelligenceRouteKind, ScreenIntelligenceRouteRequest,
    ScreenIntelligenceSourceKind,
};

pub(super) fn plan_screen_intelligence_route(
    request: &ScreenIntelligenceRouteRequest,
    route_id: impl Into<String>,
) -> ScreenIntelligenceRouteDecision {
    let route_kind = route_kind_for(request);
    let capture_scope = capture_scope_for_route(request, &route_kind);

    ScreenIntelligenceRouteDecision {
        schema_version:
            crate::screen_intelligence_router::SCREEN_INTELLIGENCE_ROUTER_SCHEMA_VERSION,
        route_id: route_id.into(),
        request_id: request.request_id.clone(),
        decided_at: request.requested_at.clone(),
        source_kind: request.source_kind.clone(),
        route_kind: route_kind.clone(),
        capture_scope: capture_scope.clone(),
        structured_extraction_id: structured_extraction_for_route(request),
        screenshot_skipped: capture_scope.is_none(),
        checked_existing_evidence_first: true,
        managed_browser_structured_extraction_first: request.source_kind
            == ScreenIntelligenceSourceKind::ManagedBrowser
            && matches!(
                route_kind,
                ScreenIntelligenceRouteKind::ManagedBrowserStructuredExtraction
                    | ScreenIntelligenceRouteKind::NoScreenNeeded
            ),
        policy_question: request.policy_question.clone(),
        policy_sensitivity: request.policy_sensitivity.clone(),
        evidence_refs: request
            .structured_extraction
            .as_ref()
            .map(|value| value.evidence_refs.clone())
            .unwrap_or_else(|| request.existing_evidence_refs.clone()),
        custody_state: request
            .structured_extraction
            .as_ref()
            .map(|value| value.custody_state.clone())
            .unwrap_or(ScreenEvidenceCustodyState::ChildDeviceQueryStore),
        manual_required_reason: (route_kind == ScreenIntelligenceRouteKind::ManualRequired)
            .then(|| manual_reason_for(request).to_string()),
        unavailable_reason: (route_kind == ScreenIntelligenceRouteKind::Unavailable)
            .then(|| unavailable_reason_for(request).to_string()),
        remote_ai_allowed: false,
        raw_screenshot_retained: false,
    }
}

fn manual_reason_for(request: &ScreenIntelligenceRouteRequest) -> &'static str {
    if !request.parent_allows_screen_capture {
        crate::screen_intelligence_router::MANUAL_REQUIRED_PARENT_DISABLED
    } else {
        crate::screen_intelligence_router::MANUAL_REQUIRED_UNSUPPORTED_SCOPE
    }
}

fn unavailable_reason_for(request: &ScreenIntelligenceRouteRequest) -> &'static str {
    if request.protected_surface_suspected
        || request.policy_sensitivity == ScreenIntelligencePolicySensitivity::ProtectedSurface
    {
        crate::screen_intelligence_router::UNAVAILABLE_PROTECTED_SURFACE
    } else {
        crate::screen_intelligence_router::UNAVAILABLE_CREDENTIAL_PROMPT
    }
}
