use super::state::ActionDispatchState;
use super::*;

pub(super) fn dispatch_parent_ui_action_network_flow_refresh(
    action: &ParentUiAction,
    state: &mut ActionDispatchState,
) {
    if !matches!(
        action.action,
        ParentUiActionKind::NetworkFlowReadModelRefreshRequested
    ) {
        return;
    }

    match load_network_flow_read_model_snapshot(None) {
        Ok(snapshot) => {
            state.events = snapshot.events.clone();
            state.network_flow_snapshot = Some(snapshot);
        }
        Err(error) => state.reject(error.to_string()),
    }
}
