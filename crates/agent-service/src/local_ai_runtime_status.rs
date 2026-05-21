use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentEventEnvelope, AgentEventName, LocalAiAdapterBoundary,
    LocalAiAdapterProbeState, LocalAiAdapterReadinessState, LocalAiDegradedState,
    LocalAiExecutionState, LocalAiModelLoadState, LocalAiProviderConfigurationState,
    LocalAiProviderPrivacyMode, LocalAiProviderSource, LocalAiResourceClass,
    LocalModelRuntimeStatus, LocalProviderAdapterProbe, LogLevel,
};

use crate::{
    event_builder::build_event, local_ai_runtime_payload::local_ai_runtime_status_payload,
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

pub fn build_local_ai_runtime_status_report(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    let checked_at = timestamp_now();
    let status = unavailable_local_ai_runtime_status(checked_at.clone());
    let probe = unavailable_local_provider_adapter_probe(checked_at);
    build_event(
        constants::event_id::LOCAL_AI_RUNTIME_STATUS_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentLocalAiRuntimeStatusReported,
        LogLevel::Info,
        local_ai_runtime_status_payload(&status, &probe),
        None,
    )
}
