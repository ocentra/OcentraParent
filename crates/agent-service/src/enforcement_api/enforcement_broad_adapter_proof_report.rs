use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentEventEnvelope, AgentEventName, LogLevel,
};

use crate::{event_builder::build_event, time::timestamp_now};

use super::{
    enforcement_broad_adapter_proof_payload::enforcement_broad_adapter_proof_payload,
    enforcement_broad_adapter_proof_read_model::v08_broad_adapter_proof_read_model,
};

pub async fn build_enforcement_broad_adapter_proof_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let generated_at = timestamp_now();
    let read_model = v08_broad_adapter_proof_read_model(&generated_at);
    build_event(
        constants::event_id::ENFORCEMENT_BROAD_ADAPTER_PROOF_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentEnforcementBroadAdapterProofReported,
        LogLevel::Info,
        enforcement_broad_adapter_proof_payload(&read_model),
        None,
    )
}
