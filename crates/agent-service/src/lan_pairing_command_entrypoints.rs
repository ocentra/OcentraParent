use ocentra_parent_agent_protocol::lan_pairing::{LanPairingOptionalText, LanParentIntentEnvelope};
use ocentra_parent_agent_protocol::transport::{AgentCommandEnvelope, AgentEventEnvelope};

use super::{runtime_commands, LanCommandDecision, LanPairingRuntime};

pub(crate) fn lan_pairing_route_select(
    runtime: LanPairingRuntime,
    origin: LanPairingOptionalText,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    runtime_commands::lan_pairing_route_select(runtime, origin, command)
}

pub(crate) fn lan_pairing_status_get(
    runtime: LanPairingRuntime,
    origin: LanPairingOptionalText,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    runtime_commands::lan_pairing_status_get(runtime, origin, command)
}

pub(crate) fn signed_child_agent_observed(
    runtime: &LanPairingRuntime,
    origin: &LanPairingOptionalText,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    runtime_commands::signed_child_agent_observed(runtime, origin, command)
}

pub(crate) async fn submit_pairing_proof(
    runtime: LanPairingRuntime,
    origin: LanPairingOptionalText,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    runtime_commands::submit_pairing_proof(runtime, origin, command).await
}

pub(crate) fn validate_control_intent(
    runtime: LanPairingRuntime,
    origin: &LanPairingOptionalText,
    command: AgentCommandEnvelope,
    intent: LanParentIntentEnvelope,
) -> LanCommandDecision {
    runtime_commands::validate_control_intent(runtime, origin, command, intent)
}
