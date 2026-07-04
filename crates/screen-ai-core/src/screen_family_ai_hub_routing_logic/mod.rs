mod consistency;
mod degradation;
mod planning;

use crate::screen_family_ai_hub_routing::{
    ScreenChildLocalAnalysisAttempt, ScreenFamilyAiHubCapability, ScreenFamilyAiHubRoute,
    ScreenFamilyAiHubRouteRequest,
};

pub(crate) fn screen_family_ai_hub_capability_is_consistent(
    value: &ScreenFamilyAiHubCapability,
) -> bool {
    consistency::screen_family_ai_hub_capability_is_consistent(value)
}

pub(crate) fn screen_child_local_attempt_is_consistent(
    value: &ScreenChildLocalAnalysisAttempt,
) -> bool {
    consistency::screen_child_local_attempt_is_consistent(value)
}

pub(crate) fn screen_family_ai_hub_route_is_consistent(value: &ScreenFamilyAiHubRoute) -> bool {
    consistency::screen_family_ai_hub_route_is_consistent(value)
}

pub(crate) fn plan_screen_family_ai_hub_route(
    request: &ScreenFamilyAiHubRouteRequest,
) -> ScreenFamilyAiHubRoute {
    planning::plan_screen_family_ai_hub_route(request)
}
