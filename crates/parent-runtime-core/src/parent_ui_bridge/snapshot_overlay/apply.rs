use super::*;

#[path = "apply/app_game_adapter_dispatch_execute_requested.rs"]
mod app_game_adapter_dispatch_execute_requested;
#[path = "apply/app_game_timer_parent_preference_setup_requested.rs"]
mod app_game_timer_parent_preference_setup_requested;
#[path = "apply/handlers.rs"]
mod handlers;
#[path = "apply/policy_request_assistant_preview_confirm_requested.rs"]
mod policy_request_assistant_preview_confirm_requested;
#[path = "apply/policy_request_parent_resolution_requested.rs"]
mod policy_request_parent_resolution_requested;
#[path = "apply/screen_settings_get_requested.rs"]
mod screen_settings_get_requested;
#[path = "apply/screen_settings_replace_requested.rs"]
mod screen_settings_replace_requested;
#[path = "apply/tracking_retention_settings_write_requested.rs"]
mod tracking_retention_settings_write_requested;

pub(super) fn apply_snapshot_overlay_for_action(
    action: &ParentUiActionKind,
    result: &AgentServiceCommandResult,
    snapshot_overlay: &mut ParentRouteSnapshotOverlay,
) -> Result<(), String> {
    if result.is_rejected() {
        return Ok(());
    }

    handlers::apply_snapshot_overlay_for_action_impl(action, result, snapshot_overlay)
}
