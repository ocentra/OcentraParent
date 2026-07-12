use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiCapabilityFlag;
use ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiDegradedState;
use ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiModelLoadState;
use ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiResourceClass;
use ocentra_parent_agent_protocol::local_ai_runtime::status::LocalModelRuntimeStatus;
use ocentra_parent_agent_protocol::local_ai_runtime::status::LocalProviderAdapterProbe;
use ocentra_parent_agent_protocol::local_ai_runtime_boundary::LocalAiAdapterBoundary;
use ocentra_parent_agent_protocol::local_ai_runtime_boundary::LocalAiAdapterProbeState;
use ocentra_parent_agent_protocol::local_ai_runtime_boundary::LocalAiAdapterReadinessState;
use ocentra_parent_agent_protocol::local_ai_runtime_boundary::LocalAiExecutionState;
use ocentra_parent_agent_protocol::local_ai_runtime_boundary::LocalAiProviderConfigurationState;
use ocentra_parent_agent_protocol::local_ai_runtime_boundary::LocalAiProviderPrivacyMode;
use ocentra_parent_agent_protocol::local_ai_runtime_boundary::LocalAiProviderSource;

use crate::{
    local_ai_runtime_config::LocalAiRuntimeConfigSnapshot,
    local_ai_runtime_config_values::LocalAiRuntimeText,
    local_ai_runtime_model_selection::uses_gpu_resource,
};

pub(crate) fn configured_local_ai_runtime_status(
    checked_at: impl Into<LocalAiRuntimeText>,
    config: &LocalAiRuntimeConfigSnapshot,
) -> LocalModelRuntimeStatus {
    let checked_at = checked_at.into();
    LocalModelRuntimeStatus {
        runtime_reference_id: constants::local_ai_runtime::RUNTIME_REFERENCE_LOCAL_LLAMA_CLI
            .to_string(),
        provider_id: constants::local_ai_runtime::PROVIDER_ID_LOCAL_LLAMA_CLI.to_string(),
        model_id: config.model_id().0,
        model_reference: config.artifact_ref().0,
        privacy_mode: LocalAiProviderPrivacyMode::LocalOnly,
        adapter_boundary: LocalAiAdapterBoundary::StatusOnly,
        execution_state: LocalAiExecutionState::Disabled,
        provider_source: LocalAiProviderSource::LocalConfig,
        load_state: LocalAiModelLoadState::Unavailable,
        capability_flags: vec![],
        resource_class: local_ai_resource_class(config),
        degraded_state: LocalAiDegradedState::None,
        last_checked_at: checked_at.0,
        unavailable_reason: Some(
            constants::local_ai_runtime::UNAVAILABLE_REASON_EXECUTION_DISABLED.to_string(),
        ),
    }
}

pub(crate) fn configured_local_provider_adapter_probe(
    checked_at: impl Into<LocalAiRuntimeText>,
    _config: &LocalAiRuntimeConfigSnapshot,
) -> LocalProviderAdapterProbe {
    let checked_at = checked_at.into();
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
        last_checked_at: checked_at.0,
        unavailable_reason: Some(
            constants::local_ai_runtime::UNAVAILABLE_REASON_EXECUTION_DISABLED.to_string(),
        ),
    }
}

pub(crate) fn executable_local_ai_runtime_status(
    checked_at: impl Into<LocalAiRuntimeText>,
    config: &LocalAiRuntimeConfigSnapshot,
) -> LocalModelRuntimeStatus {
    let checked_at = checked_at.into();
    LocalModelRuntimeStatus {
        runtime_reference_id: constants::local_ai_runtime::RUNTIME_REFERENCE_LOCAL_LLAMA_CLI
            .to_string(),
        provider_id: constants::local_ai_runtime::PROVIDER_ID_LOCAL_LLAMA_CLI.to_string(),
        model_id: config.model_id().0,
        model_reference: config.artifact_ref().0,
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
        resource_class: local_ai_resource_class(config),
        degraded_state: LocalAiDegradedState::None,
        last_checked_at: checked_at.0,
        unavailable_reason: None,
    }
}

fn local_ai_resource_class(config: &LocalAiRuntimeConfigSnapshot) -> LocalAiResourceClass {
    if uses_gpu_resource(config) {
        LocalAiResourceClass::Gpu
    } else {
        LocalAiResourceClass::Cpu
    }
}

pub(crate) fn executable_local_provider_adapter_probe(
    checked_at: impl Into<LocalAiRuntimeText>,
    _config: &LocalAiRuntimeConfigSnapshot,
) -> LocalProviderAdapterProbe {
    let checked_at = checked_at.into();
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
        last_checked_at: checked_at.0,
        unavailable_reason: None,
    }
}
