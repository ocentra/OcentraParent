use super::support::expect_agent_event;
use super::*;

pub(super) fn apply(
    result: &AgentServiceCommandResult,
    snapshot_overlay: &mut ParentRouteSnapshotOverlay,
) -> Result<(), String> {
    expect_agent_event(
        &result.response_event.event,
        &AgentEventName::AgentActivityTrackingRetentionSettingsWriteReported,
    )?;
    snapshot_overlay.activity_tracking_retention_settings_write_result =
        Some(response_json_payload_field(
            &result.response_event,
            constants::field::ACTIVITY_TRACKING_RETENTION_SETTINGS_WRITE_RESULT,
        )?);
    Ok(())
}
