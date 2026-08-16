use super::*;

pub(super) fn load_parent_route_snapshot_dependencies_impl(
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
    let browser_activity_required = route_requires_browser_activity_read_model(route);
    let browser_inventory_required = route_requires_browser_inventory_read_model(route);
    let browser_required = route_requires_browser_read_models(route);
    ParentRouteSnapshotDependencies {
        network_flow_snapshot: loaded_network_flow_snapshot,
        network_runtime_event_chain_snapshot,
        policy_preview_snapshot,
        tracking_read_model_snapshot,
        screen_read_model_snapshot,
        app_use_read_model_snapshot: if app_game_required {
            load_activity_app_use_read_model_snapshot(None).ok()
        } else {
            None
        },
        browser_activity_read_model_snapshot: if browser_activity_required {
            load_browser_activity_read_model_snapshot(None).ok()
        } else {
            None
        },
        games_read_model_snapshot: if app_game_required {
            load_activity_games_read_model_snapshot(None).ok()
        } else {
            None
        },
        browser_inventory_read_model_snapshot: if browser_inventory_required {
            load_browser_inventory_read_model_snapshot(None).ok()
        } else {
            None
        },
        browser_managed_status_snapshot: if browser_required {
            load_browser_managed_status_snapshot(None).ok()
        } else {
            None
        },
        browser_intervention_read_model_snapshot: if browser_required {
            load_browser_intervention_read_model_snapshot(None).ok()
        } else {
            None
        },
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
