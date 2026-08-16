use super::*;
use crate::parent_ui_bridge::ParentRouteLiveActivitySnapshotInput;

#[path = "snapshot/app_game.rs"]
mod app_game;
#[path = "snapshot/browser.rs"]
mod browser;
#[path = "snapshot/lan.rs"]
mod lan;
#[path = "snapshot/network.rs"]
mod network;
#[path = "snapshot/tracking.rs"]
mod tracking;

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
    lan::apply_lan_live_activity_impl(input.lan_route_query, &mut snapshot);
    network::apply_network_live_activity_impl(
        input.policy_preview_snapshot,
        input.parent_access_state,
        input.network_flow_snapshot,
        input.network_runtime_event_chain_snapshot,
        input.route,
        &mut snapshot,
    );
    tracking::apply_tracking_and_screen_live_activity_impl(
        input.tracking_read_model_snapshot,
        input.screen_read_model_snapshot,
        input.app_use_read_model_snapshot,
        input.games_read_model_snapshot,
        input.route,
        &mut snapshot,
    );
    browser::apply_browser_live_activity_impl(input, &mut snapshot);
    app_game::apply_app_game_live_activity_impl(input, &mut snapshot);
    Some(snapshot)
}

fn empty_live_activity_snapshot() -> ParentRouteLiveActivitySnapshot {
    ParentRouteLiveActivitySnapshot {
        recent_summary: None,
        ingest_status: None,
        activity_screen_read_model: None,
        activity_app_use_read_model: None,
        activity_browser_read_model: None,
        activity_games_read_model: None,
        screen_summary_panel: None,
        browser_managed_event: None,
        browser_managed_status: None,
        local_ai_runtime_status_event: None,
        lan_ai_job_event: None,
        parent_assistant_boundary_event: None,
        activity_memory_graph_read_model: None,
        network_flow_event: None,
        network_flow_read_model: None,
        network_evidence_summary: None,
        network_runtime_event_chain_stream: None,
        lan_pairing_browser_discovery_event: None,
        lan_add_device_read_model: None,
        policy_preview_panel: None,
        app_game_notification_parent_surface_panel: None,
        app_game_policy_readiness_panel: None,
        app_game_platform_proof_status_panel: None,
        app_game_child_runtime_transport_receipt_panel: None,
        app_game_adapter_dispatch_panel: None,
        app_game_timer_parent_surface_panel: None,
        browser_intervention_event: None,
        browser_intervention_read_model: None,
        activity_tracking_read_model_event: None,
        activity_tracking_read_model: None,
        activity_tracking_panel: None,
        activity_tracking_retention_settings_write_result: None,
    }
}

fn activity_tracking_panel_snapshot(
    read_model_result: Option<&ParentActivityTrackingReadModelResultSnapshot>,
    write_result: Option<&Value>,
) -> ParentTrackingStatusPanelSnapshot {
    activity_tracking_panel_snapshot_impl(read_model_result, write_result)
}
