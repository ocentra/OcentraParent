use super::state::ActionDispatchState;
use super::*;
use crate::agent_service_client::types::{AgentCommandText, AgentServiceError};

pub(super) fn dispatch_parent_ui_action_agent_command(
    action: &ParentUiAction,
    action_owned: bool,
    state: &mut ActionDispatchState,
) {
    if !matches!(action.action, ParentUiActionKind::AgentCommandRequested)
        || lan_route::is_lan_command_route(&action.route)
    {
        return;
    }

    let generic_command_result = action
        .command
        .as_deref()
        .ok_or_else(missing_agent_command_error)
        .and_then(|command_name| {
            dispatch_agent_command(AgentCommandText(command_name), &action.payload, None)
        });
    match generic_command_result {
        Ok(result) => apply_generic_command_result(action_owned, result, state),
        Err(error) => state.reject(error.to_string()),
    }
}

fn apply_generic_command_result(
    action_owned: bool,
    result: AgentServiceCommandResult,
    state: &mut ActionDispatchState,
) {
    state.accepted = action_owned && !result.is_rejected();
    if let Some(rejection_message) = result.rejection_message() {
        state.message = rejection_message.to_string();
    }
    if result.response_event.event == AgentEventName::AgentNetworkFlowReadModelReported {
        match network_flow_snapshot_from_parts(&result.response_event, &result.events) {
            Ok(snapshot) => state.network_flow_snapshot = Some(snapshot),
            Err(error) => {
                state.accepted = false;
                state.message = error;
            }
        }
    }
    state.events = result.events;
}

fn missing_agent_command_error() -> AgentServiceError {
    AgentServiceError::from_display(
        "parent Rust facade rejected agent command request without a command name",
    )
}
