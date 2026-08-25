use super::*;

pub(super) fn is_dev_tools_route_impl(route: &ParentRouteId) -> bool {
    matches!(
        route,
        ParentRouteId::Diagnostics
            | ParentRouteId::ProofPanels
            | ParentRouteId::Commands
            | ParentRouteId::Events
            | ParentRouteId::Logs
            | ParentRouteId::AppLayout
            | ParentRouteId::FrameTuner
    )
}

pub(super) fn is_lan_surface_route_impl(route: &ParentRouteId) -> bool {
    matches!(
        route,
        ParentRouteId::Devices
            | ParentRouteId::LanPairing
            | ParentRouteId::CapabilityStatus
            | ParentRouteId::PlatformsInstall
            | ParentRouteId::InstallUpdates
    )
}

pub(super) fn requires_lan_read_model_for_load(route: &ParentRouteId) -> bool {
    is_lan_command_route_impl(route) || matches!(route, ParentRouteId::Start)
}

pub(super) fn requires_lan_read_model_for_action(action: &ParentUiAction) -> bool {
    requires_lan_read_model_for_load(&action.route)
        && (!matches!(
            action.action,
            ParentUiActionKind::LanPairingBrowserDiscoveryScanRequested
        ) || is_lan_command_route_impl(&action.route))
}

pub(super) fn is_lan_command_route_impl(route: &ParentRouteId) -> bool {
    is_lan_surface_route_impl(route) || matches!(route, ParentRouteId::PolicyNetwork)
}
