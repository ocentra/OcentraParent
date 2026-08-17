use super::*;

#[path = "route_snapshot/dependencies.rs"]
mod dependencies;
use self::dependencies::build_live_activity_snapshot;

pub(super) fn build_parent_route_snapshot_impl(
    route: ParentRouteId,
    lan_route_query: &LanRouteQuery,
    network_flow_snapshot: Option<&NetworkFlowAgentServiceSnapshot>,
    snapshot_overlay: Option<&ParentRouteSnapshotOverlay>,
    service_health: Option<&ParentAgentServiceHealth>,
) -> ParentRouteSnapshot {
    if let Some(health) = service_health.filter(|health| !health.is_ready()) {
        return unavailable_parent_route_snapshot(&route, health);
    }
    let loaded =
        dependencies::load_parent_route_snapshot_dependencies(&route, network_flow_snapshot);
    let lan_add_device_read_model = lan_route_query.read_model();
    let data_source = data_source_for_route(&route, lan_route_query);
    let connection_state = connection_state_for_route(&route, lan_route_query);
    let command_enabled = command_enabled_for_route(&route, &connection_state);
    let summary = summary_for_route(&route, &data_source, lan_add_device_read_model);
    let parent_portal_rows =
        parent_portal_rows_for_route(&route, &summary, &data_source, lan_add_device_read_model);
    let diagnostic_panels_enabled = is_dev_tools_route(&route);
    let browser_panels = browser_route_panels_snapshot(&route);
    let setup_first_run_panel = setup_first_run_panel_snapshot(&route, lan_route_query);
    let generated_at = lan_add_device_read_model
        .as_ref()
        .map(|read_model| read_model.generated_at.clone())
        .unwrap_or_else(|| EMPTY_TIMESTAMP.to_string());
    let last_updated = lan_route_query
        .event()
        .and_then(|event| event.sent_at.clone())
        .unwrap_or_else(|| generated_at.clone());
    let parent_portal_shell_status = parent_portal_shell_status(
        &route,
        &summary,
        &data_source,
        &connection_state,
        lan_add_device_read_model,
    );
    let live_activity = build_live_activity_snapshot(
        &route,
        lan_route_query,
        network_flow_snapshot,
        &loaded,
        &parent_portal_shell_status,
        snapshot_overlay,
    );

    ParentRouteSnapshot {
        schema_version: PARENT_UI_BRIDGE_SCHEMA_VERSION,
        route,
        generated_at,
        season_label: season_label_for_connection(&connection_state).to_string(),
        last_updated,
        connection_state: connection_state.clone(),
        command_enabled,
        agent_endpoint: HOST_BRIDGE_URL.to_string(),
        data_source,
        summary,
        service_health: service_health.map(ParentAgentServiceHealth::to_route_snapshot),
        diagnostic_panels_enabled,
        parent_portal_rows,
        parent_portal_shell_status: Some(parent_portal_shell_status),
        live_activity,
        browser_panels,
        setup_first_run_panel,
        screen_settings_service_response: snapshot_overlay
            .and_then(|overlay| overlay.screen_settings_service_response.clone()),
    }
}

fn unavailable_parent_route_snapshot(
    route: &ParentRouteId,
    service_health: &ParentAgentServiceHealth,
) -> ParentRouteSnapshot {
    let lan_route_query = LanRouteQuery::Unavailable(service_health.redacted_detail());
    let data_source = ParentRouteDataSource::Unavailable;
    let connection_state = ParentBridgeConnectionState::Error;
    let summary = summary_for_route(route, &data_source, None);
    let parent_portal_shell_status =
        parent_portal_shell_status(route, &summary, &data_source, &connection_state, None);
    ParentRouteSnapshot {
        schema_version: PARENT_UI_BRIDGE_SCHEMA_VERSION,
        route: route.clone(),
        generated_at: EMPTY_TIMESTAMP.to_string(),
        season_label: season_label_for_connection(&connection_state).to_string(),
        last_updated: EMPTY_TIMESTAMP.to_string(),
        connection_state,
        command_enabled: false,
        agent_endpoint: HOST_BRIDGE_URL.to_string(),
        data_source,
        summary,
        service_health: Some(service_health.to_route_snapshot()),
        diagnostic_panels_enabled: false,
        parent_portal_rows: None,
        parent_portal_shell_status: Some(parent_portal_shell_status),
        live_activity: None,
        browser_panels: None,
        setup_first_run_panel: setup_first_run_panel_snapshot(route, &lan_route_query),
        screen_settings_service_response: None,
    }
}
