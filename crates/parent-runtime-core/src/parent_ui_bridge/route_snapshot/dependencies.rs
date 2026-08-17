use super::*;
use ocentra_schema::parent_ui_bridge::{
    ParentPortalShellStatusSnapshot, ParentRouteLiveActivitySnapshot,
};

use crate::agent_service_client::types::{
    AppUseReadModelAgentServiceSnapshot, BrowserActivityReadModelAgentServiceSnapshot,
    BrowserEvidenceReadModelAgentServiceSnapshot, BrowserInventoryReadModelAgentServiceSnapshot,
    GamesReadModelAgentServiceSnapshot,
};

#[path = "dependencies/load.rs"]
mod load;

#[derive(Default)]
pub(super) struct DependencyFailures {
    labels: Vec<&'static str>,
}

impl DependencyFailures {
    pub(super) fn capture<T, E>(&mut self, label: &'static str, result: Result<T, E>) -> Option<T> {
        match result {
            Ok(value) => Some(value),
            Err(_) => {
                self.labels.push(label);
                None
            }
        }
    }

    pub(super) fn is_empty(&self) -> bool {
        self.labels.is_empty()
    }

    pub(super) fn redacted_detail(&self) -> String {
        format!(
            "route dependency reads unavailable ({})",
            self.labels.join(", ")
        )
    }
}

pub(super) struct ParentRouteSnapshotDependencies {
    pub(super) dependency_failures: DependencyFailures,
    pub(super) network_flow_snapshot: Option<NetworkFlowAgentServiceSnapshot>,
    pub(super) network_runtime_event_chain_snapshot:
        Option<NetworkRuntimeEventChainAgentServiceSnapshot>,
    pub(super) policy_preview_snapshot: Option<PolicyPreviewAgentServiceSnapshot>,
    pub(super) tracking_read_model_snapshot: Option<TrackingReadModelAgentServiceSnapshot>,
    pub(super) screen_read_model_snapshot: Option<ScreenReadModelAgentServiceSnapshot>,
    pub(super) app_use_read_model_snapshot: Option<AppUseReadModelAgentServiceSnapshot>,
    pub(super) browser_activity_read_model_snapshot:
        Option<BrowserActivityReadModelAgentServiceSnapshot>,
    pub(super) games_read_model_snapshot: Option<GamesReadModelAgentServiceSnapshot>,
    pub(super) browser_inventory_read_model_snapshot:
        Option<BrowserInventoryReadModelAgentServiceSnapshot>,
    pub(super) browser_evidence_read_model_snapshot:
        Option<BrowserEvidenceReadModelAgentServiceSnapshot>,
    pub(super) browser_managed_status_snapshot: Option<BrowserManagedStatusAgentServiceSnapshot>,
    pub(super) browser_intervention_read_model_snapshot:
        Option<BrowserInterventionReadModelAgentServiceSnapshot>,
    pub(super) app_game_notification_readiness_snapshot:
        Option<AppGameNotificationReadinessAgentServiceSnapshot>,
    pub(super) app_game_policy_readiness_snapshot:
        Option<AppGamePolicyReadinessAgentServiceSnapshot>,
    pub(super) app_game_platform_proof_status_snapshot:
        Option<AppGamePlatformProofStatusAgentServiceSnapshot>,
    pub(super) app_game_child_runtime_transport_receipt_snapshot:
        Option<AppGameChildRuntimeTransportReceiptAgentServiceSnapshot>,
    pub(super) app_game_adapter_dispatch_preflight_snapshot:
        Option<AppGameAdapterDispatchPreflightAgentServiceSnapshot>,
    pub(super) app_game_adapter_dispatch_result_snapshot:
        Option<AppGameAdapterDispatchResultAgentServiceSnapshot>,
    pub(super) app_game_timer_parent_surface_snapshot:
        Option<AppGameTimerParentSurfaceAgentServiceSnapshot>,
}

pub(super) fn build_live_activity_snapshot(
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
        app_use_read_model_snapshot: loaded.app_use_read_model_snapshot.as_ref(),
        browser_activity_read_model_snapshot: loaded.browser_activity_read_model_snapshot.as_ref(),
        games_read_model_snapshot: loaded.games_read_model_snapshot.as_ref(),
        browser_inventory_read_model_snapshot: loaded
            .browser_inventory_read_model_snapshot
            .as_ref(),
        browser_evidence_read_model_snapshot: loaded.browser_evidence_read_model_snapshot.as_ref(),
        browser_managed_status_snapshot: loaded.browser_managed_status_snapshot.as_ref(),
        browser_intervention_read_model_snapshot: loaded
            .browser_intervention_read_model_snapshot
            .as_ref(),
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

pub(super) fn load_parent_route_snapshot_dependencies(
    route: &ParentRouteId,
    network_flow_snapshot: Option<&NetworkFlowAgentServiceSnapshot>,
) -> ParentRouteSnapshotDependencies {
    load::load_parent_route_snapshot_dependencies_impl(route, network_flow_snapshot)
}
