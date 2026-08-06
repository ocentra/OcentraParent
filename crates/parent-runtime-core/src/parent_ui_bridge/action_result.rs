use super::*;
use crate::parent_ui_bridge::action_result_app_game;

pub(super) fn action_result_message(action: &ParentUiAction) -> String {
    match action.action {
        ParentUiActionKind::RefreshRoute => "route snapshot refreshed by parent Rust facade",
        ParentUiActionKind::Reconnect => "parent Rust facade reloaded route state",
        ParentUiActionKind::AgentCommandRequested => {
            "parent Rust facade forwarded LAN agent command request"
        }
        ParentUiActionKind::PolicyRequestAssistantPreviewConfirmRequested => {
            "parent Rust facade requested policy preview parent confirmation"
        }
        ParentUiActionKind::LanPairingBrowserDiscoveryScanRequested => {
            "parent Rust facade requested LAN pairing browser discovery scan"
        }
        ParentUiActionKind::NetworkFlowReadModelRefreshRequested => {
            "parent Rust facade requested network flow read model refresh"
        }
        ParentUiActionKind::TrackingRetentionSettingsWriteRequested => {
            "parent Rust facade requested tracking retention settings write"
        }
        ParentUiActionKind::ScreenSettingsGetRequested => {
            "parent Rust facade requested screen settings readback"
        }
        ParentUiActionKind::ScreenSettingsReplaceRequested => {
            "parent Rust facade requested screen settings replace"
        }
        ParentUiActionKind::AppGameAdapterDispatchExecuteRequested
        | ParentUiActionKind::AppGameTimerParentPreferenceSetupRequested => {
            return action_result_app_game::action_result_message(&action.action).to_string();
        }
    }
    .to_string()
}
