use crate::agent_service_client::types::{
    AppGameAdapterDispatchPreflightAgentServiceSnapshot,
    AppGameAdapterDispatchResultAgentServiceSnapshot,
    AppGameChildRuntimeTransportReceiptAgentServiceSnapshot,
    AppGameNotificationReadinessAgentServiceSnapshot,
    AppGamePlatformProofStatusAgentServiceSnapshot, AppGamePolicyReadinessAgentServiceSnapshot,
    AppGameTimerParentSurfaceAgentServiceSnapshot, AppUseReadModelAgentServiceSnapshot,
    GamesReadModelAgentServiceSnapshot,
};
use crate::agent_service_client::{
    load_activity_app_use_read_model_snapshot, load_activity_games_read_model_snapshot,
    load_app_game_adapter_dispatch_preflight_read_model_snapshot,
    load_app_game_adapter_dispatch_result_read_model_snapshot,
    load_app_game_child_runtime_transport_receipt_read_model_snapshot,
    load_app_game_notification_readiness_read_model_snapshot,
    load_app_game_platform_proof_status_read_model_snapshot,
    load_app_game_policy_readiness_read_model_snapshot,
    load_app_game_timer_parent_surface_read_model_snapshot,
};
use crate::parent_ui_bridge::route_requirements::route_requires_app_game_session_read_models;
use crate::parent_ui_bridge::ParentRouteId;

pub(super) struct AppGameDependencies {
    pub(super) notification_readiness_snapshot:
        Option<AppGameNotificationReadinessAgentServiceSnapshot>,
    pub(super) policy_readiness_snapshot: Option<AppGamePolicyReadinessAgentServiceSnapshot>,
    pub(super) platform_proof_status_snapshot:
        Option<AppGamePlatformProofStatusAgentServiceSnapshot>,
    pub(super) child_runtime_transport_receipt_snapshot:
        Option<AppGameChildRuntimeTransportReceiptAgentServiceSnapshot>,
    pub(super) adapter_dispatch_preflight_snapshot:
        Option<AppGameAdapterDispatchPreflightAgentServiceSnapshot>,
    pub(super) adapter_dispatch_result_snapshot:
        Option<AppGameAdapterDispatchResultAgentServiceSnapshot>,
    pub(super) timer_parent_surface_snapshot: Option<AppGameTimerParentSurfaceAgentServiceSnapshot>,
}

pub(super) fn load_app_use(route: &ParentRouteId) -> Option<AppUseReadModelAgentServiceSnapshot> {
    if route_requires_app_game_session_read_models(route) {
        load_activity_app_use_read_model_snapshot(None).ok()
    } else {
        None
    }
}

pub(super) fn load_games(route: &ParentRouteId) -> Option<GamesReadModelAgentServiceSnapshot> {
    if route_requires_app_game_session_read_models(route) {
        load_activity_games_read_model_snapshot(None).ok()
    } else {
        None
    }
}

pub(super) fn load_remaining(route: &ParentRouteId) -> AppGameDependencies {
    let app_game_required = route_requires_app_game_session_read_models(route);
    let notification_readiness_snapshot = if app_game_required {
        load_app_game_notification_readiness_read_model_snapshot(None).ok()
    } else {
        None
    };
    let policy_readiness_snapshot = if app_game_required {
        load_app_game_policy_readiness_read_model_snapshot(None).ok()
    } else {
        None
    };
    let platform_proof_status_snapshot = if app_game_required {
        load_app_game_platform_proof_status_read_model_snapshot(None).ok()
    } else {
        None
    };
    let child_runtime_transport_receipt_snapshot = if app_game_required {
        load_app_game_child_runtime_transport_receipt_read_model_snapshot(None).ok()
    } else {
        None
    };
    let adapter_dispatch_preflight_snapshot = if app_game_required {
        load_app_game_adapter_dispatch_preflight_read_model_snapshot(None).ok()
    } else {
        None
    };
    let adapter_dispatch_result_snapshot = if app_game_required {
        load_app_game_adapter_dispatch_result_read_model_snapshot(None).ok()
    } else {
        None
    };
    let timer_parent_surface_snapshot = if app_game_required {
        load_app_game_timer_parent_surface_read_model_snapshot(None).ok()
    } else {
        None
    };
    AppGameDependencies {
        notification_readiness_snapshot,
        policy_readiness_snapshot,
        platform_proof_status_snapshot,
        child_runtime_transport_receipt_snapshot,
        adapter_dispatch_preflight_snapshot,
        adapter_dispatch_result_snapshot,
        timer_parent_surface_snapshot,
    }
}
