use super::*;

#[path = "portal/status.rs"]
mod status;
#[path = "portal/summary.rs"]
mod summary;

pub(super) fn summary_for_route(
    route: &ParentRouteId,
    data_source: &ParentRouteDataSource,
    lan_add_device_read_model: Option<&LanBrowserAddDeviceReadModel>,
) -> ParentRouteSummary {
    summary::summary_for_route(route, data_source, lan_add_device_read_model)
}

pub(super) fn parent_portal_rows_for_route(
    route: &ParentRouteId,
    summary: &ParentRouteSummary,
    data_source: &ParentRouteDataSource,
    lan_add_device_read_model: Option<&LanBrowserAddDeviceReadModel>,
) -> Option<Vec<ParentPortalRowSnapshot>> {
    summary::parent_portal_rows_for_route(route, summary, data_source, lan_add_device_read_model)
}

pub(super) fn parent_portal_shell_status(
    route: &ParentRouteId,
    summary: &ParentRouteSummary,
    data_source: &ParentRouteDataSource,
    connection_state: &ParentBridgeConnectionState,
    lan_add_device_read_model: Option<&LanBrowserAddDeviceReadModel>,
) -> ParentPortalShellStatusSnapshot {
    status::parent_portal_shell_status(
        route,
        summary,
        data_source,
        connection_state,
        lan_add_device_read_model,
    )
}

pub(super) const BROWSER_PANEL_EYEBROW: &str = "Browser route";
pub(super) const BROWSER_PANEL_NOT_REPORTED: &str = "not reported";
