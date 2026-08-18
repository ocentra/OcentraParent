mod consistency;
mod decision_build;
mod extraction_consistency;
mod fallback;
mod route_kind;

use crate::screen_intelligence_router::{
    ScreenIntelligenceRouteDecision, ScreenIntelligenceRouteRequest,
    ScreenManagedBrowserStructuredExtraction,
};

pub(crate) fn screen_managed_browser_structured_extraction_is_consistent(
    value: &ScreenManagedBrowserStructuredExtraction,
) -> bool {
    consistency::screen_managed_browser_structured_extraction_is_consistent(value)
}

pub(crate) fn screen_intelligence_route_request_is_consistent(
    value: &ScreenIntelligenceRouteRequest,
) -> bool {
    consistency::screen_intelligence_route_request_is_consistent(value)
}

pub(crate) fn screen_intelligence_route_decision_is_consistent(
    value: &ScreenIntelligenceRouteDecision,
) -> bool {
    consistency::screen_intelligence_route_decision_is_consistent(value)
}

pub(crate) fn structured_extraction_fallback_state_for(
    request: &ScreenIntelligenceRouteRequest,
) -> crate::screen_intelligence_router::ScreenStructuredExtractionFallbackState {
    fallback::structured_extraction_fallback_state_for(request)
}

pub(crate) fn plan_screen_intelligence_route(
    request: &ScreenIntelligenceRouteRequest,
    route_id: impl Into<String>,
) -> ScreenIntelligenceRouteDecision {
    decision_build::plan_screen_intelligence_route(request, route_id)
}
