use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiDegradedState;
use ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiModelLoadState;
use ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiResourceClass;
use ocentra_parent_agent_protocol::local_ai_runtime::status::LocalAiModelCacheStatus;
use ocentra_parent_agent_protocol::local_ai_runtime::status::LocalModelRuntimeStatus;
use ocentra_parent_agent_protocol::local_ai_runtime::status::LocalProviderAdapterProbe;
use ocentra_parent_agent_protocol::local_ai_runtime_boundary::LocalAiAdapterBoundary;
use ocentra_parent_agent_protocol::local_ai_runtime_boundary::LocalAiAdapterProbeState;
use ocentra_parent_agent_protocol::local_ai_runtime_boundary::LocalAiAdapterReadinessState;
use ocentra_parent_agent_protocol::local_ai_runtime_boundary::LocalAiExecutionState;
use ocentra_parent_agent_protocol::local_ai_runtime_boundary::LocalAiProviderConfigurationState;
use ocentra_parent_agent_protocol::local_ai_runtime_boundary::LocalAiProviderPrivacyMode;
use ocentra_parent_agent_protocol::local_ai_runtime_boundary::LocalAiProviderSource;
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;

use crate::{
    event_builder::build_event,
    local_ai_provider_scheduler::local_ai_provider_scheduler,
    local_ai_runtime_cache_status::local_ai_model_cache_status_from_config,
    local_ai_runtime_config::LocalAiRuntimeConfigSnapshot,
    local_ai_runtime_config_values::LocalAiRuntimeText,
    local_ai_runtime_configured_status::{
        configured_local_ai_runtime_status, configured_local_provider_adapter_probe,
        executable_local_ai_runtime_status, executable_local_provider_adapter_probe,
    },
    local_ai_runtime_model_selection::requested_model_unavailable_reason,
    local_ai_runtime_payload::local_ai_runtime_status_payload,
    local_ai_runtime_provider_proof_read_model::local_ai_runtime_provider_proof_read_model,
    local_ai_runtime_readiness::runtime_configuration_unavailable_reason,
    local_ai_runtime_status_unavailable::{
        unavailable_local_ai_runtime_status_for_model,
        unavailable_local_ai_runtime_status_with_config,
        unavailable_local_provider_adapter_probe_with_reason,
    },
    time::timestamp_now,
};

pub fn unavailable_local_ai_runtime_status(
    checked_at: impl Into<LocalAiRuntimeText>,
) -> LocalModelRuntimeStatus {
    let checked_at = checked_at.into();
    LocalModelRuntimeStatus {
        runtime_reference_id: constants::local_ai_runtime::RUNTIME_REFERENCE_DEV_UNCONFIGURED
            .to_string(),
        provider_id: constants::local_ai_runtime::PROVIDER_ID_UNCONFIGURED.to_string(),
        model_id: constants::local_ai_runtime::MODEL_ID_UNCONFIGURED.to_string(),
        model_reference: constants::local_ai_runtime::MODEL_REFERENCE_UNCONFIGURED.to_string(),
        privacy_mode: LocalAiProviderPrivacyMode::LocalOnly,
        adapter_boundary: LocalAiAdapterBoundary::LocalAdapterUnavailable,
        execution_state: LocalAiExecutionState::Disabled,
        provider_source: LocalAiProviderSource::Unavailable,
        load_state: LocalAiModelLoadState::Unavailable,
        capability_flags: vec![],
        resource_class: LocalAiResourceClass::Cpu,
        degraded_state: LocalAiDegradedState::ProviderUnavailable,
        last_checked_at: checked_at.0,
        unavailable_reason: Some(
            constants::local_ai_runtime::UNAVAILABLE_REASON_UNCONFIGURED.to_string(),
        ),
    }
}

pub fn unavailable_local_provider_adapter_probe(
    checked_at: impl Into<LocalAiRuntimeText>,
) -> LocalProviderAdapterProbe {
    let checked_at = checked_at.into();
    LocalProviderAdapterProbe {
        provider_id: constants::local_ai_runtime::PROVIDER_ID_UNCONFIGURED.to_string(),
        privacy_mode: LocalAiProviderPrivacyMode::LocalOnly,
        adapter_boundary: LocalAiAdapterBoundary::StatusOnly,
        execution_state: LocalAiExecutionState::Disabled,
        provider_source: LocalAiProviderSource::Unavailable,
        probe_state: LocalAiAdapterProbeState::ProbeUnavailable,
        configuration_state: LocalAiProviderConfigurationState::LocalProviderUnconfigured,
        readiness_state: LocalAiAdapterReadinessState::AdapterNotReady,
        execution_allowed: false,
        last_checked_at: checked_at.0,
        unavailable_reason: Some(
            constants::local_ai_runtime::UNAVAILABLE_REASON_UNCONFIGURED.to_string(),
        ),
    }
}

pub fn local_ai_runtime_status_from_config(
    checked_at: impl Into<LocalAiRuntimeText>,
    config: &LocalAiRuntimeConfigSnapshot,
) -> (
    LocalModelRuntimeStatus,
    LocalProviderAdapterProbe,
    LocalAiModelCacheStatus,
) {
    local_ai_runtime_status_for_model_from_config(checked_at.into(), config, None)
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
    let checked_at = checked_at.into();
    let cache = local_ai_model_cache_status_from_config(checked_at.0.clone(), config);
    if let Some(model_id) = requested_model_id {
        if let Some(reason) = requested_model_unavailable_reason(config, &model_id) {
            return (
                unavailable_local_ai_runtime_status_for_model(
                    checked_at.clone(),
                    config,
                    &model_id,
                    reason,
                ),
                unavailable_local_provider_adapter_probe_with_reason(checked_at, reason),
                cache,
            );
        }
    }

    if let Some(reason) = runtime_configuration_unavailable_reason(config) {
        return (
            unavailable_local_ai_runtime_status_with_config(checked_at.clone(), config, reason),
            unavailable_local_provider_adapter_probe_with_reason(checked_at, reason),
            cache,
        );
    }

    if config.execution_enabled() {
        return (
            executable_local_ai_runtime_status(checked_at.0.clone(), config),
            executable_local_provider_adapter_probe(checked_at.0, config),
            cache,
        );
    }

    (
        configured_local_ai_runtime_status(checked_at.0.clone(), config),
        configured_local_provider_adapter_probe(checked_at.0, config),
        cache,
    )
}

pub fn local_ai_runtime_is_executable(config: &LocalAiRuntimeConfigSnapshot) -> bool {
    config.runtime_binary().exists() && config.model_file().exists() && config.execution_enabled()
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
