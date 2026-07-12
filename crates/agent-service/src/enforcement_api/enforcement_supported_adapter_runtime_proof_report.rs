use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::enforcement_integrity_runtime_audit::V08EnforcementIntegrityRuntimeAuditReadModel;
use ocentra_parent_agent_protocol::enforcement_supported_adapter_runtime_proof::V08SupportedAdapterRuntimeProofReadModel;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;

use crate::{
    event_builder::build_event, fields::fields_from_pairs, json_contract::serialize_json_string,
    time::timestamp_now,
};

use super::enforcement_integrity_runtime_audit_read_model::{
    v08_enforcement_integrity_runtime_audit_read_model,
    GeneratedAtTextRef as IntegrityGeneratedAtTextRef,
};
use super::enforcement_supported_adapter_runtime_proof_read_model::{
    v08_supported_adapter_runtime_proof_read_model, GeneratedAtTextRef,
};

pub async fn build_enforcement_supported_adapter_runtime_proof_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let generated_at: String = timestamp_now();
    let read_model =
        v08_supported_adapter_runtime_proof_read_model(GeneratedAtTextRef(&generated_at));
    let audit_read_model = v08_enforcement_integrity_runtime_audit_read_model(
        IntegrityGeneratedAtTextRef(&generated_at),
    );
    build_event(
        constants::event_id::ENFORCEMENT_SUPPORTED_ADAPTER_RUNTIME_PROOF_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentEnforcementSupportedAdapterRuntimeProofReported,
        LogLevel::Info,
        enforcement_supported_adapter_runtime_proof_payload(&read_model, &audit_read_model),
        None,
    )
}

fn enforcement_supported_adapter_runtime_proof_payload(
    read_model: &V08SupportedAdapterRuntimeProofReadModel,
    audit_read_model: &V08EnforcementIntegrityRuntimeAuditReadModel,
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
            LogFieldValue::String(serialize_json_string(read_model).0),
        ),
        (
            constants::field::ENFORCEMENT_INTEGRITY_RUNTIME_AUDIT_READ_MODEL,
            LogFieldValue::String(serialize_json_string(audit_read_model).0),
        ),
    ])
}
