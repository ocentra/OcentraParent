use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventEnvelope,
};

use crate::{
    lan_pairing::{build_lan_pairing_status_report, LanPairingRuntime},
    lan_runtime_stream_api::build_lan_runtime_event_chain_stream_report,
};

pub(super) async fn build_lan_command_report(
    lan_pairing: &LanPairingRuntime,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    match command.command.clone() {
        AgentCommandName::AgentLanRuntimeEventChainStreamGet => {
            build_lan_runtime_event_chain_stream_report(lan_pairing, command).await
        }
        _ => build_lan_pairing_status_report(lan_pairing, command),
    }
}
