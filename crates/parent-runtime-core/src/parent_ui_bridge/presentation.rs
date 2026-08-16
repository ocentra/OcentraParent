use ocentra_parent_agent_protocol::activity_surface::{
    ActivityReadModelState, ActivityScreenReadModel, ActivityScreenReadModelRow,
};
use ocentra_parent_agent_protocol::app_game_adapter_dispatch_preflight::AppGameAdapterDispatchPreflightReadModel;
use ocentra_parent_agent_protocol::app_game_adapter_dispatch_result::AppGameAdapterDispatchResultReadModel;
use ocentra_parent_agent_protocol::app_game_child_runtime_transport_receipt::AppGameChildRuntimeTransportReceiptReadModel;
use ocentra_parent_agent_protocol::app_game_notification_readiness::{
    AppGameNotificationReadinessReadModel, AppGameNotificationReadinessRow,
};
use ocentra_parent_agent_protocol::app_game_notification_status::{
    AppGameNotificationPreferenceStatusEntry, AppGameNotificationStatusReadModels,
};
use ocentra_parent_agent_protocol::app_game_platform_proof_status::AppGamePlatformProofStatusReadModel;
use ocentra_parent_agent_protocol::app_game_policy_readiness::{
    self, AppGamePolicyReadinessReadModel, AppGamePolicyReadinessRow,
};
use ocentra_parent_agent_protocol::app_game_timer_parent_preference_setup_request::AppGameTimerParentPreferenceSetupRequest;
use ocentra_parent_agent_protocol::app_game_timer_parent_surface_read_model::AppGameTimerParentSurfaceReadModel;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceReadModel;
use ocentra_parent_agent_protocol::notification_provider_status_boundary::V08NotificationProviderStatusBoundaryEntry;
use ocentra_schema::parent_ui_bridge::{
    ParentActivityTrackingReadModelResultSnapshot, ParentActivityTrackingReadModelRowSnapshot,
    ParentAppGameActionRowSnapshot, ParentAppGameAdapterDispatchPanelSnapshot,
    ParentAppGameNotificationParentSurfacePanelRowSnapshot,
    ParentAppGameNotificationParentSurfacePanelSnapshot, ParentAppGamePanelDetailSnapshot,
    ParentAppGamePanelRowSnapshot, ParentAppGamePanelSnapshot,
    ParentAppGameTimerParentSurfacePanelSnapshot, ParentBridgeConnectionState,
    ParentBrowserPanelDetailSnapshot, ParentBrowserPanelRowSnapshot, ParentBrowserPanelSnapshot,
    ParentPolicyPreviewPanelCardSnapshot, ParentPolicyPreviewPanelDetailSnapshot,
    ParentPolicyPreviewPanelSnapshot, ParentPolicyPreviewReadModelSnapshot,
    ParentPortalParentAccessState, ParentPortalRowSnapshot, ParentPortalShellStatusCardSnapshot,
    ParentPortalShellStatusSnapshot, ParentPortalTone, ParentRouteBrowserPanelsSnapshot,
    ParentRouteDataSource, ParentRouteEventSnapshot, ParentRouteId,
    ParentRouteLiveActivitySnapshot, ParentRouteSummary, ParentScreenSummaryPanelDetailSnapshot,
    ParentScreenSummaryPanelRowSnapshot, ParentScreenSummaryPanelSnapshot,
    ParentSetupFirstRunPanelCardSnapshot, ParentSetupFirstRunPanelDetailSnapshot,
    ParentSetupFirstRunPanelSnapshot, ParentTrackingStatusPanelCardSnapshot,
    ParentTrackingStatusPanelDetailSnapshot, ParentTrackingStatusPanelSnapshot, ParentUiAction,
    ParentUiActionKind,
};
use serde_json::{json, Value};

use self::app_game_adapter_dispatch::app_game_adapter_dispatch_panel_snapshot;
use self::app_game_adapter_dispatch_execute_summary_details::app_game_adapter_dispatch_execute_summary_details;
use self::app_game_child_runtime_transport_receipt::app_game_child_runtime_transport_receipt_panel_snapshot;
use self::app_game_notification::app_game_notification_parent_surface_panel_snapshot;
use self::app_game_platform::app_game_platform_proof_status_panel_snapshot;
use self::app_game_policy::app_game_policy_readiness_panel_snapshot;
use self::app_game_readiness_labels::*;
use self::app_game_shared_panels::*;
use self::app_game_timer::app_game_timer_parent_surface_panel_snapshot;
use self::app_game_timer_labels::*;
use self::live_activity::*;
use self::policy_preview::*;
use self::portal::*;
use self::screen_summary::*;
use super::lan_route::{is_lan_surface_route, LanRouteQuery};
use super::route_metadata::{
    connection_tone, current_lan_add_device_read_model_value, data_source_label, data_source_tone,
    global_connection_state_for_connection, network_evidence_summary_snapshot,
    network_flow_read_model_snapshot, parent_portal_shell_status_card_id, portal_row_snapshot,
    route_capability_state_for_data_source, route_capability_tone, route_title,
    serialized_enum_label,
};
use super::route_requirements::{
    route_requires_policy_preview_read_model, route_requires_tracking_read_model,
};
use super::{
    NetworkFlowAgentServiceSnapshot, NetworkRuntimeEventChainAgentServiceSnapshot,
    ParentRouteLiveActivitySnapshotInput, PolicyPreviewAgentServiceSnapshot,
    ScreenReadModelAgentServiceSnapshot, TrackingReadModelAgentServiceSnapshot,
};

#[path = "action_result.rs"]
pub(super) mod action_result;
#[path = "app_game_adapter_dispatch.rs"]
pub(super) mod app_game_adapter_dispatch;
#[path = "app_game_adapter_dispatch_execute_summary_details.rs"]
pub(super) mod app_game_adapter_dispatch_execute_summary_details;
#[path = "app_game_child_runtime_transport_receipt.rs"]
pub(super) mod app_game_child_runtime_transport_receipt;
#[path = "app_game_notification.rs"]
pub(super) mod app_game_notification;
#[path = "app_game_platform.rs"]
pub(super) mod app_game_platform;
#[path = "app_game_policy.rs"]
pub(super) mod app_game_policy;
#[path = "app_game_readiness_labels.rs"]
pub(super) mod app_game_readiness_labels;
#[path = "app_game_shared_panels.rs"]
pub(super) mod app_game_shared_panels;
#[path = "app_game_timer.rs"]
pub(super) mod app_game_timer;
#[path = "app_game_timer_labels.rs"]
pub(super) mod app_game_timer_labels;
#[path = "browser.rs"]
pub(super) mod browser;
#[path = "live_activity.rs"]
pub(super) mod live_activity;
#[path = "policy_preview.rs"]
pub(super) mod policy_preview;
#[path = "portal.rs"]
pub(super) mod portal;
#[path = "screen_summary.rs"]
pub(super) mod screen_summary;

pub(super) fn summary_for_route(
    route: &ParentRouteId,
    data_source: &ParentRouteDataSource,
    lan_add_device_read_model: Option<&LanBrowserAddDeviceReadModel>,
) -> ParentRouteSummary {
    portal::summary_for_route(route, data_source, lan_add_device_read_model)
}

pub(super) fn parent_portal_rows_for_route(
    route: &ParentRouteId,
    summary: &ParentRouteSummary,
    data_source: &ParentRouteDataSource,
    lan_add_device_read_model: Option<&LanBrowserAddDeviceReadModel>,
) -> Option<Vec<ParentPortalRowSnapshot>> {
    portal::parent_portal_rows_for_route(route, summary, data_source, lan_add_device_read_model)
}

pub(super) fn parent_portal_shell_status(
    route: &ParentRouteId,
    summary: &ParentRouteSummary,
    data_source: &ParentRouteDataSource,
    connection_state: &ParentBridgeConnectionState,
    lan_add_device_read_model: Option<&LanBrowserAddDeviceReadModel>,
) -> ParentPortalShellStatusSnapshot {
    portal::parent_portal_shell_status(
        route,
        summary,
        data_source,
        connection_state,
        lan_add_device_read_model,
    )
}

pub(super) fn parent_access_state_for_lan_read_model(
    read_model: Option<&LanBrowserAddDeviceReadModel>,
) -> ParentPortalParentAccessState {
    app_game_readiness_labels::parent_access_state_for_read_model(read_model)
}

pub(super) fn browser_route_panels_snapshot(
    route: &ParentRouteId,
) -> Option<ParentRouteBrowserPanelsSnapshot> {
    browser::browser_route_panels_snapshot(route)
}

pub(super) fn setup_first_run_panel_snapshot(
    route: &ParentRouteId,
) -> Option<ParentSetupFirstRunPanelSnapshot> {
    browser::setup_first_run_panel_snapshot(route)
}

pub(super) fn live_activity_snapshot(
    input: &ParentRouteLiveActivitySnapshotInput<'_>,
) -> Option<ParentRouteLiveActivitySnapshot> {
    live_activity::live_activity_snapshot(input)
}

pub(super) fn action_result_message(action: &ParentUiAction) -> String {
    action_result::action_result_message(action)
}
