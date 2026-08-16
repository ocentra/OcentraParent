use super::*;

pub(super) fn apply_snapshot_overlay_for_action_impl(
    action: &ParentUiActionKind,
    result: &AgentServiceCommandResult,
    snapshot_overlay: &mut ParentRouteSnapshotOverlay,
) -> Result<(), String> {
    match action {
        ParentUiActionKind::PolicyRequestAssistantPreviewConfirmRequested => {
            policy_request_assistant_preview_confirm_requested::apply(result, snapshot_overlay)
        }
        ParentUiActionKind::PolicyRequestParentResolutionRequested => {
            policy_request_parent_resolution_requested::apply(result, snapshot_overlay)
        }
        ParentUiActionKind::TrackingRetentionSettingsWriteRequested => {
            tracking_retention_settings_write_requested::apply(result, snapshot_overlay)
        }
        ParentUiActionKind::ScreenSettingsGetRequested => {
            screen_settings_get_requested::apply(result, snapshot_overlay)
        }
        ParentUiActionKind::ScreenSettingsReplaceRequested => {
            screen_settings_replace_requested::apply(result, snapshot_overlay)
        }
        ParentUiActionKind::AppGameAdapterDispatchExecuteRequested => {
            app_game_adapter_dispatch_execute_requested::apply(result, snapshot_overlay)
        }
        ParentUiActionKind::AppGameTimerParentPreferenceSetupRequested => {
            app_game_timer_parent_preference_setup_requested::apply(result)
        }
        _ => Ok(()),
    }
}
