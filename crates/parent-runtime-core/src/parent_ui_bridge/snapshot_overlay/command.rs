use super::*;

pub(super) fn rust_owned_command_for_action(
    action: &ParentUiActionKind,
) -> Option<AgentCommandName> {
    match action {
        ParentUiActionKind::PolicyRequestAssistantPreviewConfirmRequested => {
            Some(AgentCommandName::AgentPolicyRequestAssistantPreviewConfirm)
        }
        ParentUiActionKind::PolicyRequestParentResolutionRequested => {
            Some(AgentCommandName::AgentPolicyRequestParentResolutionResolve)
        }
        ParentUiActionKind::TrackingRetentionSettingsWriteRequested => {
            Some(AgentCommandName::AgentActivityTrackingRetentionSettingsWrite)
        }
        ParentUiActionKind::ScreenSettingsGetRequested => {
            Some(AgentCommandName::AgentScreenSettingsGet)
        }
        ParentUiActionKind::ScreenSettingsReplaceRequested => {
            Some(AgentCommandName::AgentScreenSettingsReplace)
        }
        ParentUiActionKind::AppGameAdapterDispatchExecuteRequested => {
            Some(AgentCommandName::AgentActivityAppGameAdapterDispatchExecute)
        }
        ParentUiActionKind::AppGameTimerParentPreferenceSetupRequested => {
            Some(AgentCommandName::AgentActivityAppGameTimerParentPreferenceSetupRequest)
        }
        _ => None,
    }
}
