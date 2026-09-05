use super::*;
use crate::parent_service_health::ParentAgentServiceHealthReason;
use ocentra_schema::parent_ui_bridge::ParentRouteDataSource;

#[path = "route_snapshot/dependencies.rs"]
pub(super) mod dependencies;
use self::dependencies::build_live_activity_snapshot;
use super::parent_desktop_distribution::parent_desktop_distribution_snapshot_for_route;

pub(super) fn build_parent_route_snapshot_impl(
    route: ParentRouteId,
    lan_route_query: &LanRouteQuery,
    network_flow_snapshot: Option<&NetworkFlowAgentServiceSnapshot>,
    snapshot_overlay: Option<&ParentRouteSnapshotOverlay>,
    service_health: Option<&ParentAgentServiceHealth>,
) -> ParentRouteSnapshot {
    if let Some(health) = service_health.filter(|health| !health.is_ready()) {
        return unavailable_parent_route_snapshot(&route, health, None);
    }
    let loaded =
        dependencies::load_parent_route_snapshot_dependencies(&route, network_flow_snapshot);
    build_parent_route_snapshot_from_dependencies(
        route,
        lan_route_query,
        network_flow_snapshot,
        snapshot_overlay,
        service_health,
        loaded,
    )
}

pub(super) fn build_parent_route_snapshot_from_dependencies(
    route: ParentRouteId,
    lan_route_query: &LanRouteQuery,
    network_flow_snapshot: Option<&NetworkFlowAgentServiceSnapshot>,
    snapshot_overlay: Option<&ParentRouteSnapshotOverlay>,
    service_health: Option<&ParentAgentServiceHealth>,
    mut loaded: dependencies::ParentRouteSnapshotDependencies,
) -> ParentRouteSnapshot {
    if matches!(lan_route_query, LanRouteQuery::Unavailable(_)) {
        loaded.dependency_failures.record("lan-route-query");
    }
    if !loaded.dependency_failures.is_empty() {
        return route_dependency_failure_snapshot(&route, service_health, &loaded);
    }
    let lan_add_device_read_model = lan_route_query.read_model();
    let data_source = data_source_for_route(&route, lan_route_query);
    let connection_state = connection_state_for_route(&route, lan_route_query);
    let command_enabled = command_enabled_for_route(&route, &connection_state);
    let summary = summary_for_route(&route, &data_source, lan_add_device_read_model);
    let parent_portal_rows =
        parent_portal_rows_for_route(&route, &summary, &data_source, lan_add_device_read_model);
    let diagnostic_panels_enabled = is_dev_tools_route(&route);
    let browser_panels = browser_route_panels_snapshot(&route, &loaded);
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
    let parent_desktop_distribution = parent_desktop_distribution_snapshot_for_route(&route);

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
        parent_desktop_distribution,
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

fn route_dependency_failure_snapshot(
    route: &ParentRouteId,
    service_health: Option<&ParentAgentServiceHealth>,
    loaded: &dependencies::ParentRouteSnapshotDependencies,
) -> ParentRouteSnapshot {
    let dependency_health = service_health
        .map(|health| {
            ParentAgentServiceHealth::degraded(
                ParentAgentServiceHealthReason::RouteDependencyUnavailable,
                health.trace.clone(),
            )
        })
        .unwrap_or_else(|| {
            ParentAgentServiceHealth::unavailable_with_reason(
                ParentAgentServiceHealthReason::RouteDependencyUnavailable,
            )
        });
    let dependency_detail = loaded.dependency_failures.redacted_detail();
    unavailable_parent_route_snapshot(route, &dependency_health, Some(&dependency_detail))
}

fn unavailable_parent_route_snapshot(
    route: &ParentRouteId,
    service_health: &ParentAgentServiceHealth,
    dependency_detail: Option<&str>,
) -> ParentRouteSnapshot {
    let detail = dependency_detail
        .map(|dependency_detail| {
            format!("{}; {dependency_detail}", service_health.redacted_detail())
        })
        .unwrap_or_else(|| service_health.redacted_detail());
    let lan_route_query = LanRouteQuery::Unavailable(detail);
    let data_source = ParentRouteDataSource::Unavailable;
    let connection_state = ParentBridgeConnectionState::Error;
    let summary = summary_for_route(route, &data_source, None);
    let parent_portal_shell_status =
        parent_portal_shell_status(route, &summary, &data_source, &connection_state, None);
    let parent_desktop_distribution = parent_desktop_distribution_snapshot_for_route(route);
    let live_activity = match route {
        ParentRouteId::PolicyTracking => build_live_activity_snapshot(
            route,
            &lan_route_query,
            None,
            &dependencies::ParentRouteSnapshotDependencies::default(),
            &parent_portal_shell_status,
            None,
        ),
        _ => None,
    };
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
        parent_desktop_distribution,
        diagnostic_panels_enabled: false,
        parent_portal_rows: None,
        parent_portal_shell_status: Some(parent_portal_shell_status),
        live_activity,
        browser_panels: None,
        setup_first_run_panel: setup_first_run_panel_snapshot(route, &lan_route_query),
        screen_settings_service_response: None,
    }
}
