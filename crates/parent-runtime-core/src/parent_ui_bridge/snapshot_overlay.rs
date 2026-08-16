use super::*;

#[path = "snapshot_overlay/apply.rs"]
mod apply;
#[path = "snapshot_overlay/command.rs"]
mod command;
#[path = "snapshot_overlay/support.rs"]
mod support;

pub(super) fn rust_owned_command_for_action(
    action: &ParentUiActionKind,
) -> Option<AgentCommandName> {
    command::rust_owned_command_for_action(action)
}

pub(super) fn apply_snapshot_overlay_for_action(
    action: &ParentUiActionKind,
    result: &AgentServiceCommandResult,
    snapshot_overlay: &mut ParentRouteSnapshotOverlay,
) -> Result<(), String> {
    apply::apply_snapshot_overlay_for_action(action, result, snapshot_overlay)
}
