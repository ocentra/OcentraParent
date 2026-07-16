use super::super::snapshot_overlay::{
    apply_snapshot_overlay_for_action, rust_owned_command_for_action,
};
use super::state::ActionDispatchState;
use super::*;

pub(super) fn dispatch_parent_ui_action_rust_owned_command(
    action: &ParentUiAction,
    action_owned: bool,
    state: &mut ActionDispatchState,
) {
    let Some(command) = rust_owned_command_for_action(&action.action) else {
        return;
    };

    match dispatch_known_agent_command(command, &action.payload, None) {
        Ok(result) => apply_rust_owned_command_result(action, action_owned, result, state),
        Err(error) => state.reject(error.to_string()),
    }
}

fn apply_rust_owned_command_result(
    action: &ParentUiAction,
    action_owned: bool,
    result: AgentServiceCommandResult,
    state: &mut ActionDispatchState,
) {
    state.accepted = action_owned && !result.is_rejected();
    if let Some(rejection_message) = result.rejection_message() {
        state.message = rejection_message.to_string();
    }
    if let Err(error) =
        apply_snapshot_overlay_for_action(&action.action, &result, &mut state.snapshot_overlay)
    {
        state.accepted = false;
        state.message = error;
    }
    state.events = result.events;
}
