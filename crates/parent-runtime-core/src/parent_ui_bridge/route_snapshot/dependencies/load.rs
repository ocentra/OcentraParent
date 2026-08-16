use super::*;

#[path = "load/activity.rs"]
mod activity;
#[path = "load/app_game.rs"]
mod app_game;
#[path = "load/browser.rs"]
mod browser;
#[path = "load/network.rs"]
mod network;

pub(super) fn load_parent_route_snapshot_dependencies_impl(
    route: &ParentRouteId,
    network_flow_snapshot: Option<&NetworkFlowAgentServiceSnapshot>,
) -> ParentRouteSnapshotDependencies {
    let network = network::load(route, network_flow_snapshot);
    let activity = activity::load(route);
    let app_use_read_model_snapshot = app_game::load_app_use(route);
    let browser_activity_read_model_snapshot = browser::load_activity(route);
    let games_read_model_snapshot = app_game::load_games(route);
    let browser_inventory_read_model_snapshot = browser::load_inventory(route);
    let browser_evidence_read_model_snapshot = browser::load_evidence(route);
    let browser_status = browser::load_status(route);
    let app_game = app_game::load_remaining(route);
    ParentRouteSnapshotDependencies {
        network_flow_snapshot: network.network_flow_snapshot,
        network_runtime_event_chain_snapshot: network.network_runtime_event_chain_snapshot,
        policy_preview_snapshot: network.policy_preview_snapshot,
        tracking_read_model_snapshot: activity.tracking_read_model_snapshot,
        screen_read_model_snapshot: activity.screen_read_model_snapshot,
        app_use_read_model_snapshot,
        browser_activity_read_model_snapshot,
        games_read_model_snapshot,
        browser_inventory_read_model_snapshot,
        browser_evidence_read_model_snapshot,
        browser_managed_status_snapshot: browser_status.managed_status_snapshot,
        browser_intervention_read_model_snapshot: browser_status.intervention_read_model_snapshot,
        app_game_notification_readiness_snapshot: app_game.notification_readiness_snapshot,
        app_game_policy_readiness_snapshot: app_game.policy_readiness_snapshot,
        app_game_platform_proof_status_snapshot: app_game.platform_proof_status_snapshot,
        app_game_child_runtime_transport_receipt_snapshot: app_game
            .child_runtime_transport_receipt_snapshot,
        app_game_adapter_dispatch_preflight_snapshot: app_game.adapter_dispatch_preflight_snapshot,
        app_game_adapter_dispatch_result_snapshot: app_game.adapter_dispatch_result_snapshot,
        app_game_timer_parent_surface_snapshot: app_game.timer_parent_surface_snapshot,
    }
}
