use super::fallback::structured_extraction_fallback_state_for;
use super::route_kind::route_kind_for;
use super::route_projection::{capture_scope_for_route, structured_extraction_for_route};
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
        structured_extraction_id: structured_extraction_for_route(request, &route_kind),
        screenshot_skipped: capture_scope.is_none(),
        checked_existing_evidence_first: true,
        managed_browser_structured_extraction_first: request.source_kind
            == ScreenIntelligenceSourceKind::ManagedBrowser
            && matches!(
                &route_kind,
                ScreenIntelligenceRouteKind::ManagedBrowserStructuredExtraction
                    | ScreenIntelligenceRouteKind::NoScreenNeeded
            ),
        structured_extraction_fallback_state: structured_extraction_fallback_state_for(
            request,
            &route_kind,
        ),
        policy_question: request.policy_question.clone(),
        policy_sensitivity: request.policy_sensitivity.clone(),
        evidence_refs: if route_kind == ScreenIntelligenceRouteKind::Unavailable {
            Vec::new()
        } else if matches!(
            &route_kind,
            ScreenIntelligenceRouteKind::NoScreenNeeded
                | ScreenIntelligenceRouteKind::ManagedBrowserStructuredExtraction
        ) {
            request
                .structured_extraction
                .as_ref()
                .map(|value| value.evidence_refs().to_vec())
                .unwrap_or_else(|| request.existing_evidence_refs.clone())
        } else {
            request.existing_evidence_refs.clone()
        },
        custody_state: if matches!(
            &route_kind,
            ScreenIntelligenceRouteKind::NoScreenNeeded
                | ScreenIntelligenceRouteKind::ManagedBrowserStructuredExtraction
        ) {
            request
                .structured_extraction
                .as_ref()
                .map(|value| value.custody_state())
                .unwrap_or(ScreenEvidenceCustodyState::Unavailable)
        } else if route_kind == ScreenIntelligenceRouteKind::Unavailable {
            ScreenEvidenceCustodyState::Unavailable
        } else {
            ScreenEvidenceCustodyState::ChildDeviceQueryStore
        },
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
    } else if matches!(
        request.source_kind,
        ScreenIntelligenceSourceKind::NetworkOrSessionSummary
            | ScreenIntelligenceSourceKind::ScreenAdjacentEvidence
    ) {
        crate::screen_intelligence_router::MANUAL_REQUIRED_EVIDENCE_ONLY
    } else {
        crate::screen_intelligence_router::MANUAL_REQUIRED_UNSUPPORTED_SCOPE
    }
}

fn unavailable_reason_for(request: &ScreenIntelligenceRouteRequest) -> &'static str {
    if !super::consistency::screen_intelligence_route_request_is_consistent(request) {
        crate::screen_intelligence_router::UNAVAILABLE_INCONSISTENT_REQUEST
    } else if request.protected_surface_suspected
        || request.policy_sensitivity == ScreenIntelligencePolicySensitivity::ProtectedSurface
        || request
            .structured_extraction
            .as_ref()
            .is_some_and(|value| value.protected_content_skipped())
    {
        crate::screen_intelligence_router::UNAVAILABLE_PROTECTED_SURFACE
    } else if request.credential_prompt_suspected
        || request.policy_sensitivity == ScreenIntelligencePolicySensitivity::CredentialRisk
    {
        crate::screen_intelligence_router::UNAVAILABLE_CREDENTIAL_PROMPT
    } else if request.source_kind == ScreenIntelligenceSourceKind::ManagedBrowser {
        crate::screen_intelligence_router::UNAVAILABLE_MANAGED_BROWSER_STRUCTURED_EXTRACTION
    } else {
        crate::screen_intelligence_router::UNAVAILABLE_CREDENTIAL_PROMPT
    }
}
