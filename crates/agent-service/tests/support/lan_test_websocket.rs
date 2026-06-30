use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogLevel};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventEnvelope, AgentEventName,
};

use crate::{
    event_builder::{build_event, portal_peer},
    fields::fields_from_pairs,
    lan_pairing::{
        build_lan_pairing_status_report, route_lan_command, LanCommandDecision, LanPairingRuntime,
    },
    lan_runtime_stream_api::build_lan_runtime_event_chain_stream_report,
};

pub(crate) async fn handle_command_text_for_test(
    text: &str,
    lan_pairing: LanPairingRuntime,
    origin: Option<String>,
) -> AgentEventEnvelope {
    if text.len() > constants::lan_pairing::LAN_WEBSOCKET_COMMAND_MAX_BYTES {
        return oversized_command_text_rejected();
    }

    match serde_json::from_str::<AgentCommandEnvelope>(text) {
        Ok(command) => {
            let (command, audit_fields) =
                match route_lan_command(lan_pairing.clone(), origin, command).await {
                    LanCommandDecision::Continue {
                        command,
                        audit_fields,
                    } => (command, audit_fields),
                    LanCommandDecision::Respond(event) => return event,
                };

            let mut event = match command.command.clone() {
                AgentCommandName::AgentHealthCheck => build_health_report(command),
                AgentCommandName::AgentLanRuntimeEventChainStreamGet => {
                    build_lan_runtime_event_chain_stream_report(&lan_pairing, command).await
                }
                command_name if is_lan_runtime_command(&command_name) => {
                    build_lan_pairing_status_report(&lan_pairing, command)
                }
                _ => build_log_snapshot_report(command),
            };

            if let Some(audit_fields) = audit_fields {
                event.payload.extend(audit_fields);
            }
            event
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
}

fn is_lan_runtime_command(command: &AgentCommandName) -> bool {
    matches!(
        command,
        AgentCommandName::AgentLanPairingProofSubmit
            | AgentCommandName::AgentLanPairingRouteSelect
            | AgentCommandName::AgentLanPairingRouteRevoke
            | AgentCommandName::AgentLanPairingStatusGet
            | AgentCommandName::AgentLanRuntimeEventChainStreamGet
            | AgentCommandName::AgentLanPairingBrowserDiscoveryScan
            | AgentCommandName::AgentLanPairingAddDeviceRequest
            | AgentCommandName::AgentLanPairingControllerLeaseRenew
            | AgentCommandName::AgentLanPairingControllerLeaseRelease
            | AgentCommandName::AgentLanPairingControllerLeaseTakeover
            | AgentCommandName::AgentLanAiProviderStatusGet
            | AgentCommandName::AgentLanAiJobSubmit
    )
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

fn build_health_report(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    build_event(
        constants::event_id::HEALTH_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentHealthReported,
        LogLevel::Info,
        fields_from_pairs(vec![
            (constants::field::ONLINE, LogFieldValue::Boolean(true)),
            (
                constants::field::TRANSPORT,
                LogFieldValue::String(constants::value::TRANSPORT_WEBSOCKET.to_string()),
            ),
        ]),
        None,
    )
}

fn build_log_snapshot_report(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    build_event(
        constants::event_id::LOG_SNAPSHOT_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentLogSnapshotReported,
        LogLevel::Info,
        fields_from_pairs(vec![(
            constants::field::ENTRIES,
            LogFieldValue::Number(1.0),
        )]),
        None,
    )
}
