use super::support::expect_agent_event;
use super::*;

pub(super) fn apply(
    result: &AgentServiceCommandResult,
    snapshot_overlay: &mut ParentRouteSnapshotOverlay,
) -> Result<(), String> {
    expect_agent_event(
        &result.response_event.event,
        &AgentEventName::AgentActivityAppGameAdapterDispatchExecuted,
    )?;
    snapshot_overlay.app_game_adapter_dispatch_executed_result = Some(response_json_payload_field(
        &result.response_event,
        constants::field::APP_GAME_ADAPTER_DISPATCH_EXECUTE_RESULT,
    )?);
    Ok(())
}
