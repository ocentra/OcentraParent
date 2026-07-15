use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::screen_settings::ScreenSettingsRejectionReason;
use ocentra_parent_agent_protocol::screen_settings::ScreenSettingsUpdateResponse;
use ocentra_parent_agent_protocol::screen_settings::ScreenSettingsUpdateStatus;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::SCREEN_EVIDENCE_SCHEMA_VERSION;

#[path = "screen_settings_api/event_response.rs"]
mod event_response;

use crate::{
    event_builder::build_event,
    screen_settings_payload::screen_settings_response_payload,
    screen_settings_request::{
        kind_for_command, parse_screen_settings_request, request_id_from_command,
    },
    screen_settings_runtime::ScreenSettingsRuntime,
};

pub(crate) async fn build_screen_settings_event(
    runtime: ScreenSettingsRuntime,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let response = match parse_screen_settings_request(&command) {
        Ok(request) => runtime.handle_request(request).await,
        Err(reason) => invalid_request_response(&command, reason),
    };
    let event_name = event_response::event_name_for_response(response.kind, response.status);
    let event_id = event_response::event_id_for_response(response.kind, response.status);
    let severity = match response.status {
        ScreenSettingsUpdateStatus::Accepted => LogLevel::Info,
        ScreenSettingsUpdateStatus::Rejected => LogLevel::Warn,
    };
    build_event(
        event_id.0,
        &command.message_id,
        command.source.clone(),
        event_name,
        severity,
        screen_settings_response_payload(&response),
        None,
    )
}

fn invalid_request_response(
    command: &AgentCommandEnvelope,
    reason: ScreenSettingsRejectionReason,
) -> ScreenSettingsUpdateResponse {
    ScreenSettingsUpdateResponse {
        schema_version: SCREEN_EVIDENCE_SCHEMA_VERSION,
        request_id: request_id_from_command(command).0,
        kind: kind_for_command(command),
        status: ScreenSettingsUpdateStatus::Rejected,
        setting: None,
        audit_event_id: None,
        rejection_reason: Some(reason),
        message: Some(constants::screen_settings::MESSAGE_INVALID_SETTING.to_string()),
    }
}
