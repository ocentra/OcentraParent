use crate::screen_intelligence_router::{
    ScreenCaptureScope, ScreenEvidenceCustodyState, ScreenIntelligenceRouteDecision,
    ScreenIntelligenceRouteKind, ScreenIntelligenceRouteRequest, ScreenIntelligenceSourceKind,
    ScreenManagedBrowserStructuredExtraction, ScreenStructuredExtractionState,
};

pub(super) fn screen_managed_browser_structured_extraction_is_consistent(
    value: &ScreenManagedBrowserStructuredExtraction,
) -> bool {
    !screen_managed_browser_structured_extraction_has_invalid_static_shape(value)
        && (value.extraction_state != ScreenStructuredExtractionState::EnoughForPolicy
            || screen_managed_browser_structured_extraction_ready_for_policy(value))
        && (value.extraction_state == ScreenStructuredExtractionState::EnoughForPolicy
            || screen_managed_browser_structured_extraction_needs_screenshot_matches_state(value))
}

pub(super) fn screen_intelligence_route_request_is_consistent(
    value: &ScreenIntelligenceRouteRequest,
) -> bool {
    !value
        .allowed_capture_scopes
        .contains(&ScreenCaptureScope::FullScreen)
        && value
            .structured_extraction
            .as_ref()
            .is_none_or(|extraction| {
                value.source_kind == ScreenIntelligenceSourceKind::ManagedBrowser
                    && screen_managed_browser_structured_extraction_is_consistent(extraction)
            })
}

pub(super) fn screen_intelligence_route_decision_is_consistent(
    value: &ScreenIntelligenceRouteDecision,
) -> bool {
    screen_intelligence_base_flags_are_consistent(value)
        && screen_intelligence_route_decision_matches_route_kind(value)
}

fn screen_managed_browser_structured_extraction_has_invalid_static_shape(
    value: &ScreenManagedBrowserStructuredExtraction,
) -> bool {
    screen_intelligence_booleans_are_true(&[
        value.raw_dom_included,
        value.visible_text_character_count
            > crate::screen_intelligence_router::SCREEN_MANAGED_BROWSER_STRUCTURED_TEXT_LIMIT,
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

fn screen_intelligence_base_flags_are_consistent(value: &ScreenIntelligenceRouteDecision) -> bool {
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
        ScreenIntelligenceRouteKind::ScreenCaptureActiveWindow => {
            screen_intelligence_booleans_are_true(&[
                !value.screenshot_skipped,
                value.capture_scope == Some(ScreenCaptureScope::ActiveWindow),
            ])
        }
        ScreenIntelligenceRouteKind::ScreenCaptureSelectedWindow => {
            screen_intelligence_booleans_are_true(&[
                !value.screenshot_skipped,
                value.capture_scope == Some(ScreenCaptureScope::SelectedWindow),
            ])
        }
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

fn screen_intelligence_booleans_are_true(values: &[bool]) -> bool {
    values.iter().copied().all(|value| value)
}
