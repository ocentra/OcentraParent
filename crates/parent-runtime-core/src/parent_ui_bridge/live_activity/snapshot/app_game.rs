use super::*;

pub(super) fn apply_app_game_live_activity_impl(
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
                input
                    .app_game_notification_readiness_snapshot
                    .and_then(|snapshot| snapshot.status_read_models.as_ref()),
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
    snapshot.activity_app_game_platform_extension_read_model = input
        .app_game_platform_proof_status_snapshot
        .and_then(|snapshot| app_game_platform_extension_adapter_value(&snapshot.read_model));
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
