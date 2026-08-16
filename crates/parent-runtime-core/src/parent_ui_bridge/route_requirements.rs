use super::*;

pub(super) fn route_requires_network_flow_read_model(route: &ParentRouteId) -> bool {
    matches!(route, ParentRouteId::ProofPanels)
}

pub(super) fn route_requires_network_runtime_event_chain_stream(route: &ParentRouteId) -> bool {
    matches!(route, ParentRouteId::ProofPanels)
}

pub(super) fn route_requires_policy_preview_read_model(route: &ParentRouteId) -> bool {
    matches!(
        route,
        ParentRouteId::PolicyNetwork
            | ParentRouteId::RuleManagement
            | ParentRouteId::Schedules
            | ParentRouteId::Approvals
            | ParentRouteId::Enforcement
            | ParentRouteId::ProofPanels
    )
}

pub(super) fn route_requires_screen_summary_read_model(route: &ParentRouteId) -> bool {
    matches!(route, ParentRouteId::ScreenAnalysis)
}

pub(super) fn route_requires_tracking_read_model(route: &ParentRouteId) -> bool {
    matches!(
        route,
        ParentRouteId::PolicyTracking | ParentRouteId::ProofPanels
    )
}

pub(super) fn route_requires_app_game_session_read_models(route: &ParentRouteId) -> bool {
    matches!(route, ParentRouteId::AppGameSessions)
}

pub(super) fn route_requires_browser_read_models(route: &ParentRouteId) -> bool {
    matches!(route, ParentRouteId::Browser)
}

pub(super) fn route_requires_browser_activity_read_model(route: &ParentRouteId) -> bool {
    matches!(route, ParentRouteId::Activity | ParentRouteId::Browser)
}

pub(super) fn route_requires_browser_inventory_read_model(route: &ParentRouteId) -> bool {
    matches!(route, ParentRouteId::Browser)
}
