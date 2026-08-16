use ocentra_parent_agent_protocol::{
    constants,
    lan_pairing::LanPairingOptionalText,
    logging::{LogFieldValue, LogLevel},
    transport::{AgentCommandEnvelope, AgentEventEnvelope, AgentEventName},
};
use std::{future::Future, pin::Pin};

use crate::{
    browser_policy_runtime::BrowserPolicyRuntime,
    event_builder::{build_event, portal_peer},
    fields::fields_from_pairs,
    lan_pairing::{
        command_routing::route_lan_command, extend_log_fields, LanCommandDecision,
        LanPairingRuntime,
    },
    screen_settings_runtime::ScreenSettingsRuntime,
};

use super::{command_dispatch::build_command_event, WebsocketCommandOrigin, WebsocketCommandText};

pub(super) fn handle_command_text(
    text: WebsocketCommandText,
    lan_pairing: LanPairingRuntime,
    browser_policy: BrowserPolicyRuntime,
    screen_settings: ScreenSettingsRuntime,
    origin: WebsocketCommandOrigin,
) -> Pin<Box<dyn Future<Output = AgentEventEnvelope> + Send + 'static>> {
    Box::pin(async move {
        if text.0.len() > constants::lan_pairing::LAN_WEBSOCKET_COMMAND_MAX_BYTES {
            return oversized_command_text_rejected();
        }

        match serde_json::from_str::<AgentCommandEnvelope>(text.0.as_str()) {
            Ok(command) => {
                handle_command(
                    command,
                    lan_pairing,
                    browser_policy,
                    screen_settings,
                    origin,
                )
                .await
            }
            Err(error) => build_event(
                constants::event_id::COMMAND_REJECTED,
                constants::event_id::UNKNOWN_COMMAND,
                portal_peer(),
                AgentEventName::AgentCommandRejected,
                LogLevel::Warn,
                fields_from_pairs(vec![(
                    constants::field::REASON,
                    LogFieldValue::String(error.to_string()),
                )]),
                None,
            ),
        }
    })
}

fn handle_command(
    command: AgentCommandEnvelope,
    lan_pairing: LanPairingRuntime,
    browser_policy: BrowserPolicyRuntime,
    screen_settings: ScreenSettingsRuntime,
    origin: WebsocketCommandOrigin,
) -> Pin<Box<dyn Future<Output = AgentEventEnvelope> + Send + 'static>> {
    Box::pin(async move {
        let (command, audit_fields) = match route_lan_command(
            lan_pairing.clone(),
            crate::lan_pairing::command_routing::LanCommandOrigin(LanPairingOptionalText(origin.0)),
            command,
        )
        .await
        {
            LanCommandDecision::Continue {
                command,
                audit_fields,
            } => (command, audit_fields),
            LanCommandDecision::Respond(event) => return event,
        };

        let mut event =
            build_command_event(command, lan_pairing, browser_policy, screen_settings).await;
        if let Some(audit_fields) = audit_fields {
            extend_log_fields(&mut event.payload, audit_fields);
        }
        event
    })
}

fn oversized_command_text_rejected() -> AgentEventEnvelope {
    build_event(
        constants::event_id::COMMAND_REJECTED,
        constants::event_id::UNKNOWN_COMMAND,
        portal_peer(),
        AgentEventName::AgentCommandRejected,
        LogLevel::Warn,
        fields_from_pairs(vec![
            (
                constants::field::LAN_CONTROL_STATE,
                LogFieldValue::String(constants::value::LAN_CONTROL_REJECTED.to_string()),
            ),
            (
                constants::field::LAN_AUDIT_EVENT_TYPE,
                LogFieldValue::String(constants::value::LAN_AUDIT_CONTROL_REJECTED.to_string()),
            ),
            (
                constants::field::LAN_REJECTION_REASON,
                LogFieldValue::String(constants::value::LAN_REASON_PAYLOAD_TOO_LARGE.to_string()),
            ),
            (
                constants::field::LAN_AUTHENTICATION_STATE,
                LogFieldValue::String(constants::value::LAN_AUTH_UNAUTHENTICATED.to_string()),
            ),
            (
                constants::field::REASON,
                LogFieldValue::String(constants::value::LAN_REASON_PAYLOAD_TOO_LARGE.to_string()),
            ),
        ]),
        None,
    )
}
