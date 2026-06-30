use super::*;
use ocentra_schema::parent_ui_bridge::{
    ParentPortalShellStatusSnapshot, ParentRouteLiveActivitySnapshot,
};

pub(super) fn build_parent_route_snapshot_impl(
    route: ParentRouteId,
    lan_route_query: &LanRouteQuery,
    network_flow_snapshot: Option<&NetworkFlowAgentServiceSnapshot>,
    snapshot_overlay: Option<&ParentRouteSnapshotOverlay>,
) -> ParentRouteSnapshot {
    let loaded = load_parent_route_snapshot_dependencies(&route, network_flow_snapshot);
    let lan_add_device_read_model = lan_route_query.read_model();
    let data_source = data_source_for_route(&route, lan_route_query);
    let connection_state = connection_state_for_route(&route, lan_route_query);
    let command_enabled = command_enabled_for_route(&route, &connection_state);
    let summary = summary_for_route(&route, &data_source, lan_add_device_read_model);
    let parent_portal_rows =
        parent_portal_rows_for_route(&route, &summary, &data_source, lan_add_device_read_model);
    let diagnostic_panels_enabled = is_dev_tools_route(&route);
    let browser_panels = browser_route_panels_snapshot(&route);
    let setup_first_run_panel = setup_first_run_panel_snapshot(&route);
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

struct ParentRouteSnapshotDependencies {
    network_flow_snapshot: Option<NetworkFlowAgentServiceSnapshot>,
    network_runtime_event_chain_snapshot: Option<NetworkRuntimeEventChainAgentServiceSnapshot>,
    policy_preview_snapshot: Option<PolicyPreviewAgentServiceSnapshot>,
    tracking_read_model_snapshot: Option<TrackingReadModelAgentServiceSnapshot>,
    screen_read_model_snapshot: Option<ScreenReadModelAgentServiceSnapshot>,
    app_game_notification_readiness_snapshot:
        Option<AppGameNotificationReadinessAgentServiceSnapshot>,
    app_game_policy_readiness_snapshot: Option<AppGamePolicyReadinessAgentServiceSnapshot>,
    app_game_platform_proof_status_snapshot: Option<AppGamePlatformProofStatusAgentServiceSnapshot>,
    app_game_child_runtime_transport_receipt_snapshot:
        Option<AppGameChildRuntimeTransportReceiptAgentServiceSnapshot>,
    app_game_adapter_dispatch_preflight_snapshot:
        Option<AppGameAdapterDispatchPreflightAgentServiceSnapshot>,
    app_game_adapter_dispatch_result_snapshot:
        Option<AppGameAdapterDispatchResultAgentServiceSnapshot>,
    app_game_timer_parent_surface_snapshot: Option<AppGameTimerParentSurfaceAgentServiceSnapshot>,
}

fn build_live_activity_snapshot(
    route: &ParentRouteId,
    lan_route_query: &LanRouteQuery,
    network_flow_snapshot: Option<&NetworkFlowAgentServiceSnapshot>,
    loaded: &ParentRouteSnapshotDependencies,
    parent_portal_shell_status: &ParentPortalShellStatusSnapshot,
    snapshot_overlay: Option<&ParentRouteSnapshotOverlay>,
) -> Option<ParentRouteLiveActivitySnapshot> {
    let live_activity_input = ParentRouteLiveActivitySnapshotInput {
        route,
        lan_route_query,
        network_flow_snapshot: network_flow_snapshot.or(loaded.network_flow_snapshot.as_ref()),
        network_runtime_event_chain_snapshot: loaded.network_runtime_event_chain_snapshot.as_ref(),
        policy_preview_snapshot: loaded.policy_preview_snapshot.as_ref(),
        parent_access_state: &parent_portal_shell_status.parent_access_state,
        tracking_read_model_snapshot: loaded.tracking_read_model_snapshot.as_ref(),
        screen_read_model_snapshot: loaded.screen_read_model_snapshot.as_ref(),
        app_game_notification_readiness_snapshot: loaded
            .app_game_notification_readiness_snapshot
            .as_ref(),
        app_game_policy_readiness_snapshot: loaded.app_game_policy_readiness_snapshot.as_ref(),
        app_game_platform_proof_status_snapshot: loaded
            .app_game_platform_proof_status_snapshot
            .as_ref(),
        app_game_child_runtime_transport_receipt_snapshot: loaded
            .app_game_child_runtime_transport_receipt_snapshot
            .as_ref(),
        app_game_adapter_dispatch_preflight_snapshot: loaded
            .app_game_adapter_dispatch_preflight_snapshot
            .as_ref(),
        app_game_adapter_dispatch_result_snapshot: loaded
            .app_game_adapter_dispatch_result_snapshot
            .as_ref(),
        app_game_timer_parent_surface_snapshot: loaded
            .app_game_timer_parent_surface_snapshot
            .as_ref(),
        app_game_adapter_dispatch_execute_result: snapshot_overlay
            .and_then(|overlay| overlay.app_game_adapter_dispatch_executed_result.as_ref()),
    };
    let mut live_activity = live_activity_snapshot(&live_activity_input);
    if let (Some(live_activity), Some(snapshot_overlay)) =
        (live_activity.as_mut(), snapshot_overlay)
    {
        if let Some(value) = snapshot_overlay
            .activity_tracking_retention_settings_write_result
            .as_ref()
        {
            live_activity.activity_tracking_retention_settings_write_result = Some(value.clone());
        }
    }
    live_activity
}

fn load_parent_route_snapshot_dependencies(
    route: &ParentRouteId,
    network_flow_snapshot: Option<&NetworkFlowAgentServiceSnapshot>,
) -> ParentRouteSnapshotDependencies {
    let loaded_network_flow_snapshot =
        if network_flow_snapshot.is_none() && route_requires_network_flow_read_model(route) {
            load_network_flow_read_model_snapshot(None).ok()
        } else {
            None
        };
    let network_flow_snapshot = network_flow_snapshot.or(loaded_network_flow_snapshot.as_ref());
    let network_runtime_event_chain_snapshot = if network_flow_snapshot.is_some()
        || route_requires_network_runtime_event_chain_stream(route)
    {
        load_network_runtime_event_chain_stream_snapshot(None).ok()
    } else {
        None
    };
    let policy_preview_snapshot =
        if network_flow_snapshot.is_some() || route_requires_policy_preview_read_model(route) {
            load_policy_preview_read_model_snapshot(None).ok()
        } else {
            None
        };
    let tracking_read_model_snapshot = if route_requires_tracking_read_model(route) {
        load_tracking_read_model_snapshot(None).ok()
    } else {
        None
    };
    let screen_read_model_snapshot = if route_requires_screen_summary_read_model(route) {
        load_activity_screen_read_model_snapshot(None).ok()
    } else {
        None
    };
    let app_game_required = route_requires_app_game_session_read_models(route);
    ParentRouteSnapshotDependencies {
        network_flow_snapshot: loaded_network_flow_snapshot,
        network_runtime_event_chain_snapshot,
        policy_preview_snapshot,
        tracking_read_model_snapshot,
        screen_read_model_snapshot,
        app_game_notification_readiness_snapshot: if app_game_required {
            load_app_game_notification_readiness_read_model_snapshot(None).ok()
        } else {
            None
        },
        app_game_policy_readiness_snapshot: if app_game_required {
            load_app_game_policy_readiness_read_model_snapshot(None).ok()
        } else {
            None
        },
        app_game_platform_proof_status_snapshot: if app_game_required {
            load_app_game_platform_proof_status_read_model_snapshot(None).ok()
        } else {
            None
        },
        app_game_child_runtime_transport_receipt_snapshot: if app_game_required {
            load_app_game_child_runtime_transport_receipt_read_model_snapshot(None).ok()
        } else {
            None
        },
        app_game_adapter_dispatch_preflight_snapshot: if app_game_required {
            load_app_game_adapter_dispatch_preflight_read_model_snapshot(None).ok()
        } else {
            None
        },
        app_game_adapter_dispatch_result_snapshot: if app_game_required {
            load_app_game_adapter_dispatch_result_read_model_snapshot(None).ok()
        } else {
            None
        },
        app_game_timer_parent_surface_snapshot: if app_game_required {
            load_app_game_timer_parent_surface_read_model_snapshot(None).ok()
        } else {
            None
        },
    }
}
