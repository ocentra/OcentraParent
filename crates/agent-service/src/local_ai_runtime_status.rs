use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentEventEnvelope, AgentEventName, LocalAiAdapterBoundary,
    LocalAiAdapterProbeState, LocalAiAdapterReadinessState, LocalAiDegradedState,
    LocalAiExecutionState, LocalAiModelCacheStatus, LocalAiModelLoadState,
    LocalAiProviderConfigurationState, LocalAiProviderPrivacyMode, LocalAiProviderSource,
    LocalAiResourceClass, LocalModelRuntimeStatus, LocalProviderAdapterProbe, LogLevel,
};

use crate::{
    event_builder::build_event,
    local_ai_runtime_cache_status::local_ai_model_cache_status_from_config,
    local_ai_runtime_config::LocalAiRuntimeConfigSnapshot,
    local_ai_runtime_configured_status::{
        configured_local_ai_runtime_status, configured_local_provider_adapter_probe,
        executable_local_ai_runtime_status, executable_local_provider_adapter_probe,
    },
    local_ai_runtime_payload::local_ai_runtime_status_payload,
    time::timestamp_now,
};

pub fn unavailable_local_ai_runtime_status(checked_at: String) -> LocalModelRuntimeStatus {
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
        last_checked_at: checked_at,
        unavailable_reason: Some(
            constants::local_ai_runtime::UNAVAILABLE_REASON_UNCONFIGURED.to_string(),
        ),
    }
}

pub fn unavailable_local_provider_adapter_probe(checked_at: String) -> LocalProviderAdapterProbe {
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
        last_checked_at: checked_at,
        unavailable_reason: Some(
            constants::local_ai_runtime::UNAVAILABLE_REASON_UNCONFIGURED.to_string(),
        ),
    }
}

pub fn local_ai_runtime_status_from_config(
    checked_at: String,
    config: &LocalAiRuntimeConfigSnapshot,
) -> (
    LocalModelRuntimeStatus,
    LocalProviderAdapterProbe,
    LocalAiModelCacheStatus,
) {
    let cache = local_ai_model_cache_status_from_config(checked_at.clone(), config);

    if !config.runtime_binary().is_configured() {
        return (
            unavailable_local_ai_runtime_status_with_reason(
                checked_at.clone(),
                constants::local_ai_runtime::UNAVAILABLE_REASON_RUNTIME_BINARY_UNCONFIGURED,
            ),
            unavailable_local_provider_adapter_probe_with_reason(
                checked_at,
                constants::local_ai_runtime::UNAVAILABLE_REASON_RUNTIME_BINARY_UNCONFIGURED,
            ),
            cache,
        );
    }

    if !config.runtime_binary().exists() {
        return (
            unavailable_local_ai_runtime_status_with_reason(
                checked_at.clone(),
                constants::local_ai_runtime::UNAVAILABLE_REASON_RUNTIME_BINARY_MISSING,
            ),
            unavailable_local_provider_adapter_probe_with_reason(
                checked_at,
                constants::local_ai_runtime::UNAVAILABLE_REASON_RUNTIME_BINARY_MISSING,
            ),
            cache,
        );
    }

    if !config.model_file().is_configured() {
        return (
            unavailable_local_ai_runtime_status_with_reason(
                checked_at.clone(),
                constants::local_ai_runtime::UNAVAILABLE_REASON_MODEL_FILE_UNCONFIGURED,
            ),
            unavailable_local_provider_adapter_probe_with_reason(
                checked_at,
                constants::local_ai_runtime::UNAVAILABLE_REASON_MODEL_FILE_UNCONFIGURED,
            ),
            cache,
        );
    }

    if !config.model_file().exists() {
        return (
            unavailable_local_ai_runtime_status_with_reason(
                checked_at.clone(),
                constants::local_ai_runtime::UNAVAILABLE_REASON_MODEL_FILE_MISSING,
            ),
            unavailable_local_provider_adapter_probe_with_reason(
                checked_at,
                constants::local_ai_runtime::UNAVAILABLE_REASON_MODEL_FILE_MISSING,
            ),
            cache,
        );
    }

    if config.execution_enabled() {
        return (
            executable_local_ai_runtime_status(checked_at.clone(), config),
            executable_local_provider_adapter_probe(checked_at, config),
            cache,
        );
    }

    (
        configured_local_ai_runtime_status(checked_at.clone(), config),
        configured_local_provider_adapter_probe(checked_at, config),
        cache,
    )
}

pub fn local_ai_runtime_is_executable(config: &LocalAiRuntimeConfigSnapshot) -> bool {
    config.runtime_binary().exists() && config.model_file().exists() && config.execution_enabled()
}

pub async fn build_local_ai_runtime_status_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let checked_at = timestamp_now();
    let config = tokio::task::spawn_blocking(LocalAiRuntimeConfigSnapshot::from_environment)
        .await
        .unwrap_or_else(|_| LocalAiRuntimeConfigSnapshot::unconfigured());
    let (status, probe, cache) = local_ai_runtime_status_from_config(checked_at, &config);
    build_event(
        constants::event_id::LOCAL_AI_RUNTIME_STATUS_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentLocalAiRuntimeStatusReported,
        LogLevel::Info,
        local_ai_runtime_status_payload(&status, &probe, &cache),
        None,
    )
}

fn unavailable_local_ai_runtime_status_with_reason(
    checked_at: String,
    reason: &'static str,
) -> LocalModelRuntimeStatus {
    let mut status = unavailable_local_ai_runtime_status(checked_at);
    status.unavailable_reason = Some(reason.to_string());
    status
}

fn unavailable_local_provider_adapter_probe_with_reason(
    checked_at: String,
    reason: &'static str,
) -> LocalProviderAdapterProbe {
    let mut probe = unavailable_local_provider_adapter_probe(checked_at);
    probe.unavailable_reason = Some(reason.to_string());
    probe
}
