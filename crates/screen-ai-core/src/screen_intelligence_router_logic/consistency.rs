use crate::screen_intelligence_router::{
    ScreenCaptureScope, ScreenIntelligencePolicySensitivity, ScreenIntelligenceRouteDecision,
    ScreenIntelligenceRouteKind, ScreenIntelligenceRouteRequest, ScreenIntelligenceSourceKind,
    ScreenManagedBrowserStructuredExtraction, ScreenStructuredExtractionFallbackState,
};

use super::extraction_consistency;

pub(super) fn screen_managed_browser_structured_extraction_is_consistent(
    value: &ScreenManagedBrowserStructuredExtraction,
) -> bool {
    extraction_consistency::is_consistent(value)
}

pub(super) fn screen_managed_browser_structured_extraction_is_ready_for_route(
    value: &ScreenManagedBrowserStructuredExtraction,
) -> bool {
    extraction_consistency::is_ready_for_structured_route(value)
}

pub(super) fn managed_browser_structured_extraction_producer_is_unavailable(
    value: &ScreenIntelligenceRouteRequest,
) -> bool {
    value.source_kind == ScreenIntelligenceSourceKind::ManagedBrowser
        && value.parent_allows_managed_browser_structured_extraction
        && value
            .structured_extraction
            .as_ref()
            .is_none_or(|extraction| {
                !screen_managed_browser_structured_extraction_is_ready_for_route(extraction)
            })
}

pub(super) fn screen_capture_is_unsafe(value: &ScreenIntelligenceRouteRequest) -> bool {
    [
        value.protected_surface_suspected,
        value.credential_prompt_suspected,
        value.policy_sensitivity == ScreenIntelligencePolicySensitivity::ProtectedSurface,
        value.policy_sensitivity == ScreenIntelligencePolicySensitivity::CredentialRisk,
    ]
    .into_iter()
    .any(|flag| flag)
}

pub(super) fn screen_intelligence_route_request_is_consistent(
    value: &ScreenIntelligenceRouteRequest,
) -> bool {
    value.schema_version
        == crate::screen_intelligence_router::SCREEN_INTELLIGENCE_ROUTER_SCHEMA_VERSION
        && !value.request_id.trim().is_empty()
        && !value.requested_at.trim().is_empty()
        && !value.device_ref.trim().is_empty()
        && !value.capture_reason.trim().is_empty()
        && !value.policy_question.trim().is_empty()
        && !value
            .allowed_capture_scopes
            .contains(&ScreenCaptureScope::FullScreen)
        && redacted_evidence_refs_are_consistent(&value.existing_evidence_refs)
        && (value.source_kind == ScreenIntelligenceSourceKind::ManagedBrowser
            || (!value.parent_allows_managed_browser_structured_extraction
                && value.structured_extraction.is_none()))
        && value
            .structured_extraction
            .as_ref()
            .is_none_or(|extraction| {
                value.source_kind == ScreenIntelligenceSourceKind::ManagedBrowser
                    && extraction_consistency::is_consistent(extraction)
            })
}

fn redacted_evidence_refs_are_consistent(
    value: &[crate::screen_intelligence_router::ActivityEvidenceRef],
) -> bool {
    value.iter().all(|reference| {
        !reference.evidence_id.trim().is_empty()
            && !reference.kind.trim().is_empty()
            && !reference.digest.trim().is_empty()
            && reference.uri.is_none()
    })
}

pub(super) fn screen_intelligence_route_decision_is_consistent(
    value: &ScreenIntelligenceRouteDecision,
) -> bool {
    screen_intelligence_base_flags_are_consistent(value)
        && screen_intelligence_route_decision_matches_route_kind(value)
}

fn screen_intelligence_base_flags_are_consistent(value: &ScreenIntelligenceRouteDecision) -> bool {
    [
        value.checked_existing_evidence_first,
        !value.remote_ai_allowed,
        !value.raw_screenshot_retained,
    ]
    .into_iter()
    .all(|value| value)
}

fn screen_intelligence_route_decision_matches_route_kind(
    value: &ScreenIntelligenceRouteDecision,
) -> bool {
    match value.route_kind {
        ScreenIntelligenceRouteKind::NoScreenNeeded => [
            value.source_kind == ScreenIntelligenceSourceKind::ManagedBrowser,
            value.screenshot_skipped,
            value.capture_scope.is_none(),
            value.managed_browser_structured_extraction_first,
            value.structured_extraction_id.is_some(),
            value.structured_extraction_fallback_state
                == ScreenStructuredExtractionFallbackState::NotRequired,
        ]
        .into_iter()
        .all(|value| value),
        ScreenIntelligenceRouteKind::ManagedBrowserStructuredExtraction => [
            value.source_kind == ScreenIntelligenceSourceKind::ManagedBrowser,
            value.screenshot_skipped,
            value.managed_browser_structured_extraction_first,
            value.capture_scope.is_none(),
            value.structured_extraction_id.is_some(),
            value.structured_extraction_fallback_state
                == ScreenStructuredExtractionFallbackState::NotAttempted,
        ]
        .into_iter()
        .all(|value| value),
        ScreenIntelligenceRouteKind::ScreenCaptureActiveWindow => [
            !value.screenshot_skipped,
            value.capture_scope == Some(ScreenCaptureScope::ActiveWindow),
            value.structured_extraction_fallback_state
                != ScreenStructuredExtractionFallbackState::NotRequired,
        ]
        .into_iter()
        .all(|value| value),
        ScreenIntelligenceRouteKind::ScreenCaptureSelectedWindow => [
            !value.screenshot_skipped,
            value.capture_scope == Some(ScreenCaptureScope::SelectedWindow),
            value.structured_extraction_fallback_state
                != ScreenStructuredExtractionFallbackState::NotRequired,
        ]
        .into_iter()
        .all(|value| value),
        ScreenIntelligenceRouteKind::ManualRequired => [
            value.screenshot_skipped,
            value.capture_scope.is_none(),
            value.manual_required_reason.is_some(),
        ]
        .into_iter()
        .all(|value| value),
        ScreenIntelligenceRouteKind::Unavailable => [
            value.screenshot_skipped,
            value.capture_scope.is_none(),
            value.unavailable_reason.is_some(),
        ]
        .into_iter()
        .all(|value| value),
    }
}
