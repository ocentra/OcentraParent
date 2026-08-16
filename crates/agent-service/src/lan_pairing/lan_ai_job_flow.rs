#[path = "lan_ai_job_flow/fields.rs"]
pub(crate) mod fields;
#[path = "lan_ai_job_flow/job_submit.rs"]
mod job_submit;
#[path = "lan_ai_job_flow/provider_status_get.rs"]
mod provider_status_get;

use ocentra_parent_agent_protocol::lan_pairing::LanPairingOptionalText;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;

use crate::lan_pairing::LanPairingRuntime;

pub(crate) fn lan_ai_provider_status_get(
    runtime: LanPairingRuntime,
    origin: LanPairingOptionalText,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    provider_status_get::lan_ai_provider_status_get(runtime, origin, command)
}

pub(crate) fn lan_ai_job_submit(
    runtime: &LanPairingRuntime,
    origin: LanPairingOptionalText,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    job_submit::lan_ai_job_submit(runtime, origin, command)
}
