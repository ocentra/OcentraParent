use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventName,
};

use crate::test_text::TestText;
use crate::{
    event_builder::build_event,
    fields::fields_from_pairs,
    lan_pairing::{
        build_lan_pairing_status_report, command_routing::route_lan_command, LanCommandDecision,
        LanPairingRuntime,
    },
    lan_runtime_stream_api::build_lan_runtime_event_chain_stream_report,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingOptionalText;

const LAN_RUNTIME_COMMANDS: &[AgentCommandName] = &[
    AgentCommandName::AgentLanPairingProofSubmit,
    AgentCommandName::AgentLanPairingRouteSelect,
    AgentCommandName::AgentLanPairingRouteRevoke,
    AgentCommandName::AgentLanPairingStatusGet,
    AgentCommandName::AgentLanRuntimeEventChainStreamGet,
    AgentCommandName::AgentLanPairingBrowserDiscoveryScan,
    AgentCommandName::AgentLanPairingAddDeviceRequest,
    AgentCommandName::AgentLanPairingControllerLeaseRenew,
    AgentCommandName::AgentLanPairingControllerLeaseRelease,
    AgentCommandName::AgentLanPairingControllerLeaseTakeover,
    AgentCommandName::AgentLanAiProviderStatusGet,
    AgentCommandName::AgentLanAiJobSubmit,
];

pub(crate) async fn dispatch_lan_test_command(
    lan_pairing: LanPairingRuntime,
    origin: Option<TestText>,
    command: AgentCommandEnvelope,
) -> ocentra_parent_agent_protocol::transport::AgentEventEnvelope {
    let (command, audit_fields) = match route_lan_command(
        lan_pairing.clone(),
        crate::lan_pairing::command_routing::LanCommandOrigin(LanPairingOptionalText(
            origin.map(|value| value.0),
        )),
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

    let mut event = dispatch_command_event(&lan_pairing, command).await;
    if let Some(audit_fields) = audit_fields {
        event = with_audit_fields(event, audit_fields);
    }
    event
}

fn with_audit_fields(
    mut event: ocentra_parent_agent_protocol::transport::AgentEventEnvelope,
    audit_fields: LogFields,
) -> ocentra_parent_agent_protocol::transport::AgentEventEnvelope {
    for (key, value) in audit_fields.into_inner() {
        event.payload.insert(key, value);
    }
    event
}

async fn dispatch_command_event(
    lan_pairing: &LanPairingRuntime,
    command: AgentCommandEnvelope,
) -> ocentra_parent_agent_protocol::transport::AgentEventEnvelope {
    match command.command.clone() {
        AgentCommandName::AgentHealthCheck => build_health_report(command),
        AgentCommandName::AgentLanRuntimeEventChainStreamGet => {
            build_lan_runtime_event_chain_stream_report(lan_pairing, command).await
        }
        command_name if is_lan_runtime_command(&command_name) => {
            build_lan_pairing_status_report(lan_pairing, command)
        }
        _ => build_log_snapshot_report(command),
    }
}

fn is_lan_runtime_command(command: &AgentCommandName) -> bool {
    LAN_RUNTIME_COMMANDS.contains(command)
}

fn build_health_report(
    command: AgentCommandEnvelope,
) -> ocentra_parent_agent_protocol::transport::AgentEventEnvelope {
    build_event(
        constants::event_id::HEALTH_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentHealthReported,
        ocentra_parent_agent_protocol::logging::LogLevel::Info,
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

fn build_log_snapshot_report(
    command: AgentCommandEnvelope,
) -> ocentra_parent_agent_protocol::transport::AgentEventEnvelope {
    build_event(
        constants::event_id::LOG_SNAPSHOT_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentLogSnapshotReported,
        ocentra_parent_agent_protocol::logging::LogLevel::Info,
        fields_from_pairs(vec![(
            constants::field::ENTRIES,
            LogFieldValue::Number(1.0),
        )]),
        None,
    )
}
