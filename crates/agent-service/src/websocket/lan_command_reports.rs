use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingOptionalText, LanPairingRejectionReason,
};
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
        _ => build_lan_pairing_status_report_blocking(lan_pairing, command).await,
    }
}

async fn build_lan_pairing_status_report_blocking(
    runtime: &LanPairingRuntime,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let fallback_command = command.clone();
    let runtime = runtime.clone();
    match tokio::task::spawn_blocking(move || build_lan_pairing_status_report(&runtime, command))
        .await
    {
        Ok(event) => event,
        Err(_) => crate::lan_pairing::rejection_event(
            fallback_command,
            &LanPairingRejectionReason::Malformed,
            None,
            &LanPairingOptionalText(None),
        ),
    }
}
