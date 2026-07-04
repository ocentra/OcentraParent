#[path = "live_activity/snapshot.rs"]
mod snapshot;
#[path = "live_activity/tracking_panel.rs"]
mod tracking_panel;

use self::snapshot::live_activity_snapshot_impl;
use self::tracking_panel::activity_tracking_panel_snapshot_impl;
use super::*;
use crate::parent_ui_bridge::ParentRouteLiveActivitySnapshotInput;

pub(super) fn live_activity_snapshot(
    input: &ParentRouteLiveActivitySnapshotInput<'_>,
) -> Option<ParentRouteLiveActivitySnapshot> {
    live_activity_snapshot_impl(input)
}

fn empty_live_activity_snapshot() -> ParentRouteLiveActivitySnapshot {
    ParentRouteLiveActivitySnapshot {
        recent_summary: None,
        ingest_status: None,
        activity_screen_read_model: None,
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

pub(super) const SCREEN_SUMMARY_DETAIL_SEPARATOR: &str = " | ";
pub(super) const SCREEN_SUMMARY_NOT_REPORTED: &str = "Not reported";
pub(super) const SCREEN_SUMMARY_UNAVAILABLE: &str = "Unavailable";
