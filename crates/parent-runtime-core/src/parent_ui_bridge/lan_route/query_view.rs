use super::*;

pub(super) fn data_source_for_route_impl(
    route: &ParentRouteId,
    lan_route_query: &LanRouteQuery,
) -> ParentRouteDataSource {
    if is_dev_tools_route_impl(route) {
        return ParentRouteDataSource::DevDiagnostics;
    }
    if is_lan_surface_route_impl(route) {
        return if matches!(lan_route_query, LanRouteQuery::Available(_)) {
            ParentRouteDataSource::RustReadModel
        } else {
            ParentRouteDataSource::Unavailable
        };
    }
    ParentRouteDataSource::HostBridge
}

pub(super) fn connection_state_for_route_impl(
    route: &ParentRouteId,
    lan_route_query: &LanRouteQuery,
) -> ParentBridgeConnectionState {
    if is_lan_surface_route_impl(route) {
        return match lan_route_query {
            LanRouteQuery::Available(_) => ParentBridgeConnectionState::Connected,
            LanRouteQuery::NotRequired | LanRouteQuery::Unavailable(_) => {
                ParentBridgeConnectionState::Error
            }
        };
    }
    ParentBridgeConnectionState::Connected
}

pub(super) fn command_enabled_for_route_impl(
    route: &ParentRouteId,
    connection_state: &ParentBridgeConnectionState,
) -> bool {
    if matches!(
        route,
        ParentRouteId::PlatformsInstall | ParentRouteId::InstallUpdates
    ) {
        return false;
    }
    if is_lan_surface_route_impl(route) {
        return matches!(connection_state, ParentBridgeConnectionState::Connected);
    }
    true
}
