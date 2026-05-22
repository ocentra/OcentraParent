use ocentra_parent_agent_protocol::{
    constants, LocalAiAdapterBoundary, LocalAiAdapterProbeState, LocalAiAdapterReadinessState,
    LocalAiCapabilityFlag, LocalAiDegradedState, LocalAiExecutionState, LocalAiModelLoadState,
    LocalAiProviderConfigurationState, LocalAiProviderPrivacyMode, LocalAiProviderSource,
    LocalAiResourceClass, LocalModelRuntimeStatus, LocalProviderAdapterProbe,
};

use crate::local_ai_runtime_config::LocalAiRuntimeConfigSnapshot;

pub(crate) fn configured_local_ai_runtime_status(
    checked_at: String,
    config: &LocalAiRuntimeConfigSnapshot,
) -> LocalModelRuntimeStatus {
    LocalModelRuntimeStatus {
        runtime_reference_id: constants::local_ai_runtime::RUNTIME_REFERENCE_LOCAL_LLAMA_CLI
            .to_string(),
        provider_id: constants::local_ai_runtime::PROVIDER_ID_LOCAL_LLAMA_CLI.to_string(),
        model_id: constants::local_ai_runtime::MODEL_ID_LOCAL_GGUF_CONFIGURED.to_string(),
        model_reference: config.artifact_ref().to_string(),
        privacy_mode: LocalAiProviderPrivacyMode::LocalOnly,
        adapter_boundary: LocalAiAdapterBoundary::StatusOnly,
        execution_state: LocalAiExecutionState::Disabled,
        provider_source: LocalAiProviderSource::LocalConfig,
        load_state: LocalAiModelLoadState::Unavailable,
        capability_flags: vec![],
        resource_class: LocalAiResourceClass::Cpu,
        degraded_state: LocalAiDegradedState::None,
        last_checked_at: checked_at,
        unavailable_reason: Some(
            constants::local_ai_runtime::UNAVAILABLE_REASON_EXECUTION_DISABLED.to_string(),
        ),
    }
}

pub(crate) fn configured_local_provider_adapter_probe(
    checked_at: String,
    _config: &LocalAiRuntimeConfigSnapshot,
) -> LocalProviderAdapterProbe {
    LocalProviderAdapterProbe {
        provider_id: constants::local_ai_runtime::PROVIDER_ID_LOCAL_LLAMA_CLI.to_string(),
        privacy_mode: LocalAiProviderPrivacyMode::LocalOnly,
        adapter_boundary: LocalAiAdapterBoundary::StatusOnly,
        execution_state: LocalAiExecutionState::Disabled,
        provider_source: LocalAiProviderSource::LocalConfig,
        probe_state: LocalAiAdapterProbeState::ProbeReady,
        configuration_state: LocalAiProviderConfigurationState::LocalProviderConfigured,
        readiness_state: LocalAiAdapterReadinessState::AdapterNotReady,
        execution_allowed: false,
        last_checked_at: checked_at,
        unavailable_reason: Some(
            constants::local_ai_runtime::UNAVAILABLE_REASON_EXECUTION_DISABLED.to_string(),
        ),
    }
}

pub(crate) fn executable_local_ai_runtime_status(
    checked_at: String,
    config: &LocalAiRuntimeConfigSnapshot,
) -> LocalModelRuntimeStatus {
    LocalModelRuntimeStatus {
        runtime_reference_id: constants::local_ai_runtime::RUNTIME_REFERENCE_LOCAL_LLAMA_CLI
            .to_string(),
        provider_id: constants::local_ai_runtime::PROVIDER_ID_LOCAL_LLAMA_CLI.to_string(),
        model_id: constants::local_ai_runtime::MODEL_ID_LOCAL_GGUF_CONFIGURED.to_string(),
        model_reference: config.artifact_ref().to_string(),
        privacy_mode: LocalAiProviderPrivacyMode::LocalOnly,
        adapter_boundary: LocalAiAdapterBoundary::LocalAdapterReady,
        execution_state: LocalAiExecutionState::DryRunReady,
        provider_source: LocalAiProviderSource::LocalConfig,
        load_state: LocalAiModelLoadState::Loaded,
        capability_flags: vec![
            LocalAiCapabilityFlag::ChatCompletion,
            LocalAiCapabilityFlag::Summarization,
            LocalAiCapabilityFlag::SafetyDecision,
        ],
        resource_class: LocalAiResourceClass::Cpu,
        degraded_state: LocalAiDegradedState::None,
        last_checked_at: checked_at,
        unavailable_reason: None,
    }
}

pub(crate) fn executable_local_provider_adapter_probe(
    checked_at: String,
    _config: &LocalAiRuntimeConfigSnapshot,
) -> LocalProviderAdapterProbe {
    LocalProviderAdapterProbe {
        provider_id: constants::local_ai_runtime::PROVIDER_ID_LOCAL_LLAMA_CLI.to_string(),
        privacy_mode: LocalAiProviderPrivacyMode::LocalOnly,
        adapter_boundary: LocalAiAdapterBoundary::LocalAdapterReady,
        execution_state: LocalAiExecutionState::DryRunReady,
        provider_source: LocalAiProviderSource::LocalConfig,
        probe_state: LocalAiAdapterProbeState::ProbeReady,
        configuration_state: LocalAiProviderConfigurationState::LocalProviderConfigured,
        readiness_state: LocalAiAdapterReadinessState::AdapterReady,
        execution_allowed: true,
        last_checked_at: checked_at,
        unavailable_reason: None,
    }
}
