use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::local_ai_runtime::status::{
    LocalAiModelCacheStatus, LocalModelRuntimeStatus, LocalProviderAdapterProbe,
};
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentEventEnvelope, AgentEventName,
};

#[path = "local_ai_runtime_status/evaluation.rs"]
mod evaluation;

use crate::event_builder::build_event;
use crate::local_ai_provider_scheduler::local_ai_provider_scheduler;
use crate::local_ai_runtime_config::LocalAiRuntimeConfigSnapshot;
use crate::local_ai_runtime_config_values::LocalAiRuntimeText;
use crate::local_ai_runtime_payload::local_ai_runtime_status_payload;
use crate::local_ai_runtime_provider_proof_read_model::local_ai_runtime_provider_proof_read_model;
use crate::time::timestamp_now;

pub fn unavailable_local_ai_runtime_status(
    checked_at: impl Into<LocalAiRuntimeText>,
) -> LocalModelRuntimeStatus {
    evaluation::unavailable_local_ai_runtime_status(checked_at)
}

pub fn unavailable_local_provider_adapter_probe(
    checked_at: impl Into<LocalAiRuntimeText>,
) -> LocalProviderAdapterProbe {
    evaluation::unavailable_local_provider_adapter_probe(checked_at)
}

pub fn local_ai_runtime_status_from_config(
    checked_at: impl Into<LocalAiRuntimeText>,
    config: &LocalAiRuntimeConfigSnapshot,
) -> (
    LocalModelRuntimeStatus,
    LocalProviderAdapterProbe,
    LocalAiModelCacheStatus,
) {
    evaluation::local_ai_runtime_status_from_config(checked_at, config)
}

pub fn local_ai_runtime_status_for_model_from_config(
    checked_at: impl Into<LocalAiRuntimeText>,
    config: &LocalAiRuntimeConfigSnapshot,
    requested_model_id: Option<LocalAiRuntimeText>,
) -> (
    LocalModelRuntimeStatus,
    LocalProviderAdapterProbe,
    LocalAiModelCacheStatus,
) {
    evaluation::local_ai_runtime_status_for_model_from_config(
        checked_at,
        config,
        requested_model_id,
    )
}

pub fn local_ai_runtime_is_executable(config: &LocalAiRuntimeConfigSnapshot) -> bool {
    evaluation::local_ai_runtime_is_executable(config)
}

pub async fn build_local_ai_runtime_status_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let checked_at: String = timestamp_now();
    let config = tokio::task::spawn_blocking(LocalAiRuntimeConfigSnapshot::from_environment)
        .await
        .unwrap_or_else(|_| LocalAiRuntimeConfigSnapshot::unconfigured());
    let requested_model_id = requested_model_id_from_command(&command);
    let (status, probe, cache) = local_ai_runtime_status_for_model_from_config(
        checked_at.clone(),
        &config,
        requested_model_id,
    );
    let provider_proof = local_ai_runtime_provider_proof_read_model(
        &checked_at,
        &local_ai_provider_scheduler().status_snapshot(),
    );
    build_event(
        constants::event_id::LOCAL_AI_RUNTIME_STATUS_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentLocalAiRuntimeStatusReported,
        LogLevel::Info,
        local_ai_runtime_status_payload(&status, &probe, &cache, &provider_proof),
        None,
    )
}

fn requested_model_id_from_command(command: &AgentCommandEnvelope) -> Option<LocalAiRuntimeText> {
    match command.payload.get(constants::field::LOCAL_AI_MODEL_ID) {
        Some(ocentra_parent_agent_protocol::logging::LogFieldValue::String(value)) => {
            Some(LocalAiRuntimeText(value.trim().to_string()))
        }
        _ => None,
    }
}
