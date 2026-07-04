use super::support::serialized_label;
use super::*;

pub(super) fn apply(
    result: &AgentServiceCommandResult,
    snapshot_overlay: &mut ParentRouteSnapshotOverlay,
) -> Result<(), String> {
    if !matches!(
        result.response_event.event,
        AgentEventName::AgentScreenSettingsReplaceAccepted
            | AgentEventName::AgentScreenSettingsReplaceRejected
    ) {
        return Err(format!(
            "agent-service expected screen settings replace response event, received {}",
            serialized_label(&result.response_event.event)
        ));
    }
    snapshot_overlay.screen_settings_service_response = Some(response_json_payload_field(
        &result.response_event,
        constants::field::SCREEN_SETTINGS_RESPONSE,
    )?);
    Ok(())
}
