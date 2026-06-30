use super::*;
use crate::parent_ui_bridge::ParentRouteLiveActivitySnapshotInput;

pub(super) fn live_activity_snapshot_impl(
    input: &ParentRouteLiveActivitySnapshotInput<'_>,
) -> Option<ParentRouteLiveActivitySnapshot> {
    if matches!(
        input.route,
        ParentRouteId::Commands
            | ParentRouteId::Events
            | ParentRouteId::Logs
            | ParentRouteId::AppLayout
            | ParentRouteId::FrameTuner
    ) {
        return None;
    }

    let mut snapshot = empty_live_activity_snapshot();
    apply_lan_live_activity(input.lan_route_query, &mut snapshot);
    apply_network_live_activity(
        input.policy_preview_snapshot,
        input.parent_access_state,
        input.network_flow_snapshot,
        input.network_runtime_event_chain_snapshot,
        input.route,
        &mut snapshot,
    );
    apply_tracking_and_screen_live_activity(
        input.tracking_read_model_snapshot,
        input.screen_read_model_snapshot,
        input.route,
        &mut snapshot,
    );
    apply_app_game_live_activity(input, &mut snapshot);
    Some(snapshot)
}

fn apply_lan_live_activity(
    lan_route_query: &LanRouteQuery,
    snapshot: &mut ParentRouteLiveActivitySnapshot,
) {
    if let Some(read_model) = lan_route_query.read_model() {
        snapshot.lan_add_device_read_model =
            current_lan_add_device_read_model_value(Some(read_model));
    }
    if let Some(event) = lan_route_query.discovery_event() {
        snapshot.lan_pairing_browser_discovery_event = Some(event.clone());
    }
}

fn apply_network_live_activity(
    policy_preview_snapshot: Option<&PolicyPreviewAgentServiceSnapshot>,
    parent_access_state: &ParentPortalParentAccessState,
    network_flow_snapshot: Option<&NetworkFlowAgentServiceSnapshot>,
    network_runtime_event_chain_snapshot: Option<&NetworkRuntimeEventChainAgentServiceSnapshot>,
    route: &ParentRouteId,
    snapshot: &mut ParentRouteLiveActivitySnapshot,
) {
    if let Some(network_flow_snapshot) = network_flow_snapshot {
        snapshot.network_flow_event = Some(network_flow_snapshot.event.clone());
        snapshot.network_flow_read_model = Some(network_flow_read_model_snapshot(
            &network_flow_snapshot.read_model,
        ));
    }
    if let Some(network_runtime_event_chain_snapshot) = network_runtime_event_chain_snapshot {
        snapshot.network_runtime_event_chain_stream =
            Some(network_runtime_event_chain_snapshot.stream.clone());
    }
    if policy_preview_snapshot.is_some() || route_requires_policy_preview_read_model(route) {
        snapshot.policy_preview_panel = Some(policy_preview_panel_snapshot(
            policy_preview_snapshot
                .as_ref()
                .map(|snapshot| &snapshot.event),
            policy_preview_snapshot
                .as_ref()
                .map(|snapshot| &snapshot.read_model),
            parent_access_state,
        ));
    }
    snapshot.network_evidence_summary = network_evidence_summary_snapshot(
        snapshot.network_flow_event.as_ref(),
        snapshot.network_runtime_event_chain_stream.as_ref(),
        policy_preview_snapshot
            .as_ref()
            .map(|snapshot| &snapshot.read_model),
    );
}

fn apply_tracking_and_screen_live_activity(
    tracking_read_model_snapshot: Option<&TrackingReadModelAgentServiceSnapshot>,
    screen_read_model_snapshot: Option<&ScreenReadModelAgentServiceSnapshot>,
    route: &ParentRouteId,
    snapshot: &mut ParentRouteLiveActivitySnapshot,
) {
    if let Some(tracking_read_model_snapshot) = tracking_read_model_snapshot {
        snapshot.activity_tracking_read_model_event =
            Some(tracking_read_model_snapshot.event.clone());
        snapshot.activity_tracking_read_model =
            Some(tracking_read_model_snapshot.read_model.clone());
    }
    if snapshot.activity_tracking_read_model.is_some() || route_requires_tracking_read_model(route)
    {
        snapshot.activity_tracking_panel = Some(activity_tracking_panel_snapshot(
            snapshot.activity_tracking_read_model.as_ref(),
            snapshot
                .activity_tracking_retention_settings_write_result
                .as_ref(),
        ));
    }
    if screen_read_model_snapshot.is_some() || matches!(route, ParentRouteId::ScreenAnalysis) {
        snapshot.screen_summary_panel = Some(screen_summary_panel_snapshot(
            screen_read_model_snapshot.map(|snapshot| &snapshot.read_model),
        ));
    }
}

fn apply_app_game_live_activity(
    input: &ParentRouteLiveActivitySnapshotInput<'_>,
    snapshot: &mut ParentRouteLiveActivitySnapshot,
) {
    if input.app_game_notification_readiness_snapshot.is_some()
        || matches!(input.route, ParentRouteId::AppGameSessions)
    {
        snapshot.app_game_notification_parent_surface_panel =
            Some(app_game_notification_parent_surface_panel_snapshot(
                input
                    .app_game_notification_readiness_snapshot
                    .map(|snapshot| &snapshot.read_model),
            ));
    }
    if input.app_game_policy_readiness_snapshot.is_some()
        || matches!(input.route, ParentRouteId::AppGameSessions)
    {
        snapshot.app_game_policy_readiness_panel = Some(app_game_policy_readiness_panel_snapshot(
            input
                .app_game_policy_readiness_snapshot
                .map(|snapshot| &snapshot.read_model),
        ));
    }
    if input.app_game_platform_proof_status_snapshot.is_some()
        || matches!(input.route, ParentRouteId::AppGameSessions)
    {
        snapshot.app_game_platform_proof_status_panel =
            Some(app_game_platform_proof_status_panel_snapshot(
                input
                    .app_game_platform_proof_status_snapshot
                    .map(|snapshot| &snapshot.read_model),
            ));
    }
    if input
        .app_game_child_runtime_transport_receipt_snapshot
        .is_some()
        || matches!(input.route, ParentRouteId::AppGameSessions)
    {
        snapshot.app_game_child_runtime_transport_receipt_panel =
            Some(app_game_child_runtime_transport_receipt_panel_snapshot(
                input
                    .app_game_child_runtime_transport_receipt_snapshot
                    .map(|snapshot| &snapshot.read_model),
            ));
    }
    if input.app_game_adapter_dispatch_preflight_snapshot.is_some()
        || input.app_game_adapter_dispatch_result_snapshot.is_some()
        || input.app_game_adapter_dispatch_execute_result.is_some()
        || matches!(input.route, ParentRouteId::AppGameSessions)
    {
        snapshot.app_game_adapter_dispatch_panel = Some(app_game_adapter_dispatch_panel_snapshot(
            input
                .app_game_adapter_dispatch_preflight_snapshot
                .map(|snapshot| &snapshot.read_model),
            input
                .app_game_adapter_dispatch_result_snapshot
                .map(|snapshot| &snapshot.read_model),
            input.app_game_adapter_dispatch_execute_result,
        ));
    }
    if input.app_game_timer_parent_surface_snapshot.is_some()
        || matches!(input.route, ParentRouteId::AppGameSessions)
    {
        snapshot.app_game_timer_parent_surface_panel =
            Some(app_game_timer_parent_surface_panel_snapshot(
                input
                    .app_game_timer_parent_surface_snapshot
                    .map(|snapshot| &snapshot.read_model),
            ));
    }
}
