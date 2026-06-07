use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentCommandName, LogFieldValue, ScreenSettingsGetRequest,
    ScreenSettingsRejectionReason, ScreenSettingsUpdateKind, ScreenSettingsUpdateRequest,
};

pub(crate) fn parse_screen_settings_request(
    command: &AgentCommandEnvelope,
) -> Result<ScreenSettingsUpdateRequest, ScreenSettingsRejectionReason> {
    match command
        .payload
        .get(constants::field::SCREEN_SETTINGS_REQUEST)
    {
        Some(LogFieldValue::String(text)) => {
            serde_json::from_str(text).map_err(|_| ScreenSettingsRejectionReason::InvalidSetting)
        }
        _ if command.command == AgentCommandName::AgentScreenSettingsGet => {
            Ok(ScreenSettingsUpdateRequest::Get(ScreenSettingsGetRequest {
                schema_version: ocentra_parent_agent_protocol::SCREEN_EVIDENCE_SCHEMA_VERSION,
                request_id: command.message_id.clone(),
                kind: ScreenSettingsUpdateKind::Get,
            }))
        }
        _ => Err(ScreenSettingsRejectionReason::InvalidSetting),
    }
}

pub(crate) fn request_id_from_command(command: &AgentCommandEnvelope) -> String {
    match command
        .payload
        .get(constants::field::SCREEN_SETTINGS_REQUEST)
    {
        Some(LogFieldValue::String(text)) => serde_json::from_str::<serde_json::Value>(text)
            .ok()
            .and_then(|value| {
                value
                    .get(constants::field::SCREEN_SETTINGS_REQUEST_ID)
                    .and_then(|request_id| request_id.as_str().map(ToString::to_string))
            })
            .unwrap_or_else(|| command.message_id.clone()),
        _ => command.message_id.clone(),
    }
}

pub(crate) fn kind_for_command(command: &AgentCommandEnvelope) -> ScreenSettingsUpdateKind {
    match command.command {
        AgentCommandName::AgentScreenSettingsReplace => ScreenSettingsUpdateKind::Replace,
        _ => ScreenSettingsUpdateKind::Get,
    }
}
