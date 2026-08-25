use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingOptionalText, LanPairingRejectionReason,
};
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentEventEnvelope, AgentEventName,
};

use crate::{
    event_builder::build_event,
    lan_pairing::LanPairingRuntime,
    lan_pairing_browser_add_device_state::browser_add_device_read_model,
    lan_pairing_status::discovery_state_for_runtime,
    lan_runtime_stream_payload::{
        lan_runtime_event_chain_stream_payload, stream_lan_runtime_event_chain_for_read_model,
    },
};

pub(crate) async fn build_lan_runtime_event_chain_stream_report(
    runtime: &LanPairingRuntime,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let fallback_command = command.clone();
    let runtime_for_read_model = runtime.clone();
    let command_for_read_model = command.clone();
    let read_model = match tokio::task::spawn_blocking(move || {
        let discovery_state = discovery_state_for_runtime(&runtime_for_read_model);
        browser_add_device_read_model(
            &runtime_for_read_model,
            &command_for_read_model,
            &discovery_state,
        )
    })
    .await
    {
        Ok(read_model) => read_model,
        Err(_) => {
            return crate::lan_pairing::rejection_event(
                fallback_command,
                &LanPairingRejectionReason::Malformed,
                None,
                &LanPairingOptionalText(None),
            );
        }
    };
    let stream = stream_lan_runtime_event_chain_for_read_model(&read_model);
    build_event(
        constants::lan_pairing::EVENT_RUNTIME_EVENT_CHAIN_STREAM_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentLanRuntimeEventChainStreamReported,
        LogLevel::Info,
        lan_runtime_event_chain_stream_payload(&stream),
        None,
    )
}
