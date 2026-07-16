use ocentra_parent_agent_protocol::lan_pairing::LanPairingOptionalText;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;

pub(crate) fn lan_ai_provider_status_get(
    runtime: crate::lan_pairing::LanPairingRuntime,
    origin: LanPairingOptionalText,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    super::lan_ai_job_flow::lan_ai_provider_status_get(runtime, origin, command)
}

pub(crate) fn lan_ai_job_submit(
    runtime: &crate::lan_pairing::LanPairingRuntime,
    origin: LanPairingOptionalText,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    super::lan_ai_job_flow::lan_ai_job_submit(runtime, origin, command)
}
