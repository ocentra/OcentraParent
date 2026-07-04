use crate::screen_intelligence_router::{
    ActivityEvidenceRef, ScreenCaptureScope, ScreenEvidenceCustodyState,
    ScreenIntelligencePolicySensitivity, ScreenIntelligenceRouteDecision,
    ScreenIntelligenceRouteKind, ScreenIntelligenceRouteRequest, ScreenIntelligenceSourceKind,
    ScreenManagedBrowserStructuredExtraction, ScreenStructuredExtractionRedactionState,
    ScreenStructuredExtractionState,
};

pub(crate) fn screen_managed_browser_structured_extraction_is_consistent(
    value: &ScreenManagedBrowserStructuredExtraction,
) -> bool {
    if screen_managed_browser_structured_extraction_has_invalid_static_shape(value) {
        return false;
    }
    if value.extraction_state == ScreenStructuredExtractionState::EnoughForPolicy {
        return screen_managed_browser_structured_extraction_ready_for_policy(value);
    }
    screen_managed_browser_structured_extraction_needs_screenshot_matches_state(value)
}

pub(crate) fn screen_intelligence_route_request_is_consistent(
    value: &ScreenIntelligenceRouteRequest,
) -> bool {
    if !screen_intelligence_route_request_capture_scopes_are_supported(value) {
        return false;
    }
    screen_intelligence_route_request_structured_extraction_is_consistent(value)
}

pub(crate) fn screen_intelligence_route_decision_is_consistent(
    value: &ScreenIntelligenceRouteDecision,
) -> bool {
    if !screen_intelligence_route_decision_base_is_consistent(value) {
        return false;
    }
    screen_intelligence_route_decision_matches_route_kind(value)
}

pub(crate) fn plan_screen_intelligence_route(
    request: &ScreenIntelligenceRouteRequest,
    route_id: impl Into<String>,
) -> ScreenIntelligenceRouteDecision {
    let route_kind = route_kind_for(request);
    let capture_scope = capture_scope_for_route(request, &route_kind);
    let structured_extraction_id = structured_extraction_for_route(request);

    ScreenIntelligenceRouteDecision {
        schema_version: crate::screen_intelligence_router::SCREEN_INTELLIGENCE_ROUTER_SCHEMA_VERSION,
        route_id: route_id.into(),
        request_id: request.request_id.clone(),
        decided_at: request.requested_at.clone(),
        source_kind: request.source_kind.clone(),
        route_kind: route_kind.clone(),
        capture_scope: capture_scope.clone(),
        structured_extraction_id,
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
        manual_required_reason: if route_kind == ScreenIntelligenceRouteKind::ManualRequired {
            Some(manual_reason_for(request).to_string())
        } else {
            None
        },
        unavailable_reason: if route_kind == ScreenIntelligenceRouteKind::Unavailable {
            Some(unavailable_reason_for(request).to_string())
        } else {
            None
        },
        remote_ai_allowed: false,
        raw_screenshot_retained: false,
    }
}

fn screen_managed_browser_structured_extraction_has_invalid_static_shape(
    value: &ScreenManagedBrowserStructuredExtraction,
) -> bool {
    screen_intelligence_booleans_are_true(&[
        value.raw_dom_included,
        value.visible_text_character_count > crate::screen_intelligence_router::SCREEN_MANAGED_BROWSER_STRUCTURED_TEXT_LIMIT,
        value.custody_state == ScreenEvidenceCustodyState::OcentraHostedNonActivity,
    ])
}

fn screen_managed_browser_structured_extraction_ready_for_policy(
    value: &ScreenManagedBrowserStructuredExtraction,
) -> bool {
    screen_intelligence_booleans_are_true(&[
        value.enough_for_policy,
        value.policy_question_answered,
        value.no_screen_needed,
        !value.screenshot_required,
        value.category_candidate.is_some(),
    ])
}

fn screen_managed_browser_structured_extraction_needs_screenshot_matches_state(
    value: &ScreenManagedBrowserStructuredExtraction,
) -> bool {
    !value.no_screen_needed
        && value.screenshot_required
            == (value.extraction_state == ScreenStructuredExtractionState::NeedsScreenshot)
}

fn screen_intelligence_route_request_capture_scopes_are_supported(
    value: &ScreenIntelligenceRouteRequest,
) -> bool {
    !value.allowed_capture_scopes.contains(&ScreenCaptureScope::FullScreen)
}

fn screen_intelligence_route_request_structured_extraction_is_consistent(
    value: &ScreenIntelligenceRouteRequest,
) -> bool {
    match &value.structured_extraction {
        Some(extraction) => {
            value.source_kind == ScreenIntelligenceSourceKind::ManagedBrowser
                && screen_managed_browser_structured_extraction_is_consistent(extraction)
        }
        None => true,
    }
}

fn screen_intelligence_route_decision_base_is_consistent(
    value: &ScreenIntelligenceRouteDecision,
) -> bool {
    screen_intelligence_booleans_are_true(&[
        value.checked_existing_evidence_first,
        !value.remote_ai_allowed,
        !value.raw_screenshot_retained,
    ])
}

fn screen_intelligence_route_decision_matches_route_kind(
    value: &ScreenIntelligenceRouteDecision,
) -> bool {
    match value.route_kind {
        ScreenIntelligenceRouteKind::NoScreenNeeded => screen_intelligence_booleans_are_true(&[
            value.screenshot_skipped,
            value.capture_scope.is_none(),
            value.structured_extraction_id.is_some(),
        ]),
        ScreenIntelligenceRouteKind::ManagedBrowserStructuredExtraction => {
            screen_intelligence_booleans_are_true(&[
                value.screenshot_skipped,
                value.managed_browser_structured_extraction_first,
                value.capture_scope.is_none(),
            ])
        }
        ScreenIntelligenceRouteKind::ScreenCaptureActiveWindow => screen_intelligence_booleans_are_true(&[
            !value.screenshot_skipped,
            value.capture_scope == Some(ScreenCaptureScope::ActiveWindow),
        ]),
        ScreenIntelligenceRouteKind::ScreenCaptureSelectedWindow => screen_intelligence_booleans_are_true(&[
            !value.screenshot_skipped,
            value.capture_scope == Some(ScreenCaptureScope::SelectedWindow),
        ]),
        ScreenIntelligenceRouteKind::ManualRequired => screen_intelligence_booleans_are_true(&[
            value.screenshot_skipped,
            value.capture_scope.is_none(),
            value.manual_required_reason.is_some(),
        ]),
        ScreenIntelligenceRouteKind::Unavailable => screen_intelligence_booleans_are_true(&[
            value.screenshot_skipped,
            value.capture_scope.is_none(),
            value.unavailable_reason.is_some(),
        ]),
    }
}

fn screen_capture_is_unsafe(request: &ScreenIntelligenceRouteRequest) -> bool {
    screen_intelligence_booleans_are_true(&[
        request.protected_surface_suspected,
        request.credential_prompt_suspected,
        request.policy_sensitivity == ScreenIntelligencePolicySensitivity::ProtectedSurface,
        request.policy_sensitivity == ScreenIntelligencePolicySensitivity::CredentialRisk,
    ])
}

fn route_kind_for(request: &ScreenIntelligenceRouteRequest) -> ScreenIntelligenceRouteKind {
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
    capture_route_kind_for(preferred_capture_scope(&request.allowed_capture_scopes))
}

fn capture_route_kind_for(
    capture_scope: Option<&ScreenCaptureScope>,
) -> ScreenIntelligenceRouteKind {
    match capture_scope {
        Some(ScreenCaptureScope::ActiveWindow) => {
            ScreenIntelligenceRouteKind::ScreenCaptureActiveWindow
        }
        Some(ScreenCaptureScope::SelectedWindow) => {
            ScreenIntelligenceRouteKind::ScreenCaptureSelectedWindow
        }
        _ => ScreenIntelligenceRouteKind::ManualRequired,
    }
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

fn capture_scope_for_route(
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

fn structured_extraction_for_route(request: &ScreenIntelligenceRouteRequest) -> Option<String> {
    request
        .structured_extraction
        .as_ref()
        .map(|value| value.extraction_id.clone())
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

fn screen_intelligence_booleans_are_true(values: &[bool]) -> bool {
    values.iter().copied().all(|value| value)
}
