use super::support::expect_agent_event;
use super::*;

pub(super) fn apply(
    result: &AgentServiceCommandResult,
    snapshot_overlay: &mut ParentRouteSnapshotOverlay,
) -> Result<(), String> {
    expect_agent_event(
        &result.response_event.event,
        &AgentEventName::AgentScreenSettingsReported,
    )?;
    snapshot_overlay.screen_settings_service_response = Some(response_json_payload_field(
        &result.response_event,
        constants::field::SCREEN_SETTINGS_RESPONSE,
    )?);
    Ok(())
}
