use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentEventEnvelope, AgentEventName, LogFieldValue, LogFields,
    LogLevel, V08SupportedAdapterRuntimeProofReadModel,
};

use crate::{event_builder::build_event, fields::fields_from_pairs, time::timestamp_now};

use super::enforcement_supported_adapter_runtime_proof_read_model::v08_supported_adapter_runtime_proof_read_model;

pub async fn build_enforcement_supported_adapter_runtime_proof_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let generated_at = timestamp_now();
    let read_model = v08_supported_adapter_runtime_proof_read_model(&generated_at);
    build_event(
        constants::event_id::ENFORCEMENT_SUPPORTED_ADAPTER_RUNTIME_PROOF_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentEnforcementSupportedAdapterRuntimeProofReported,
        LogLevel::Info,
        enforcement_supported_adapter_runtime_proof_payload(&read_model),
        None,
    )
}

fn enforcement_supported_adapter_runtime_proof_payload(
    read_model: &V08SupportedAdapterRuntimeProofReadModel,
) -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::READ_MODEL_ID,
            LogFieldValue::String(read_model.read_model_id.clone()),
        ),
        (
            constants::field::GENERATED_AT,
            LogFieldValue::String(read_model.generated_at.clone()),
        ),
        (
            constants::field::RETURNED,
            LogFieldValue::Number(read_model.entries.len() as f64),
        ),
        (
            constants::field::ENFORCEMENT_SUPPORTED_ADAPTER_RUNTIME_PROOF_READ_MODEL,
            LogFieldValue::String(read_model_json(read_model)),
        ),
    ])
}

fn read_model_json(read_model: &V08SupportedAdapterRuntimeProofReadModel) -> String {
    serde_json::to_string(read_model).expect(constants::error::AGENT_EVENT_SERIALIZES)
}
