use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::screen_settings::ScreenSettingsRejectionReason;
use ocentra_parent_agent_protocol::screen_settings::ScreenSettingsUpdateKind;
use ocentra_parent_agent_protocol::screen_settings::ScreenSettingsUpdateResponse;
use ocentra_parent_agent_protocol::screen_settings::ScreenSettingsUpdateStatus;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;
use ocentra_parent_agent_protocol::SCREEN_EVIDENCE_SCHEMA_VERSION;

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
    let event_name = event_name_for_response(response.kind, response.status);
    let event_id = event_id_for_response(response.kind, response.status);
    let severity = match response.status {
        ScreenSettingsUpdateStatus::Accepted => LogLevel::Info,
        ScreenSettingsUpdateStatus::Rejected => LogLevel::Warn,
    };
    build_event(
        event_id,
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
        request_id: request_id_from_command(command),
        kind: kind_for_command(command),
        status: ScreenSettingsUpdateStatus::Rejected,
        setting: None,
        audit_event_id: None,
        rejection_reason: Some(reason),
        message: Some(constants::screen_settings::MESSAGE_INVALID_SETTING.to_string()),
    }
}

fn event_id_for_response(
    kind: ScreenSettingsUpdateKind,
    status: ScreenSettingsUpdateStatus,
) -> &'static str {
    match (kind, status) {
        (ScreenSettingsUpdateKind::Replace, ScreenSettingsUpdateStatus::Accepted) => {
            constants::event_id::SCREEN_SETTINGS_REPLACE_ACCEPTED
        }
        (ScreenSettingsUpdateKind::Replace, ScreenSettingsUpdateStatus::Rejected) => {
            constants::event_id::SCREEN_SETTINGS_REPLACE_REJECTED
        }
        _ => constants::event_id::SCREEN_SETTINGS_REPORTED,
    }
}

fn event_name_for_response(
    kind: ScreenSettingsUpdateKind,
    status: ScreenSettingsUpdateStatus,
) -> AgentEventName {
    match (kind, status) {
        (ScreenSettingsUpdateKind::Replace, ScreenSettingsUpdateStatus::Accepted) => {
            AgentEventName::AgentScreenSettingsReplaceAccepted
        }
        (ScreenSettingsUpdateKind::Replace, ScreenSettingsUpdateStatus::Rejected) => {
            AgentEventName::AgentScreenSettingsReplaceRejected
        }
        _ => AgentEventName::AgentScreenSettingsReported,
    }
}
