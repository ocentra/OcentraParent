use std::path::Path;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::{
    LocalAiCapabilityFlag, LocalAiDegradedState, LocalAiModelLoadState, LocalAiResourceClass,
};
use ocentra_parent_agent_protocol::local_ai_runtime::status::LocalModelRuntimeStatus;
use ocentra_parent_agent_protocol::local_ai_runtime_boundary::{
    LocalAiAdapterBoundary, LocalAiExecutionState, LocalAiProviderPrivacyMode,
    LocalAiProviderSource,
};
use ocentra_parent_agent_protocol::screen_evidence::{
    SCREEN_SERVICE_ANALYSIS_MODEL_ID, SCREEN_SERVICE_ANALYSIS_MODEL_REFERENCE,
    SCREEN_SERVICE_ANALYSIS_PROVIDER_ID, SCREEN_SERVICE_ANALYSIS_RUNTIME_REF,
};

#[derive(Clone, Copy)]
pub(super) struct AdapterRuntimeCommand<'a>(pub(super) Option<&'a Path>);
#[derive(Clone, Copy)]
pub(super) struct AdapterRuntimeTimestamp<'a>(pub(super) &'a str);

pub(super) fn runtime_status(
    command: AdapterRuntimeCommand<'_>,
    timestamp: AdapterRuntimeTimestamp<'_>,
) -> LocalModelRuntimeStatus {
    let available = command.0.is_some_and(Path::is_file);
    LocalModelRuntimeStatus {
        runtime_reference_id: SCREEN_SERVICE_ANALYSIS_RUNTIME_REF.to_string(),
        provider_id: SCREEN_SERVICE_ANALYSIS_PROVIDER_ID.to_string(),
        model_id: SCREEN_SERVICE_ANALYSIS_MODEL_ID.to_string(),
        model_reference: SCREEN_SERVICE_ANALYSIS_MODEL_REFERENCE.to_string(),
        privacy_mode: LocalAiProviderPrivacyMode::LocalOnly,
        adapter_boundary: if available {
            LocalAiAdapterBoundary::LocalAdapterReady
        } else {
            LocalAiAdapterBoundary::LocalAdapterUnavailable
        },
        execution_state: if available {
            LocalAiExecutionState::DryRunReady
        } else {
            LocalAiExecutionState::Disabled
        },
        provider_source: if available {
            LocalAiProviderSource::LocalConfig
        } else {
            LocalAiProviderSource::Unavailable
        },
        load_state: if available {
            LocalAiModelLoadState::Loaded
        } else {
            LocalAiModelLoadState::Unavailable
        },
        capability_flags: if available {
            vec![
                LocalAiCapabilityFlag::Classification,
                LocalAiCapabilityFlag::SafetyDecision,
            ]
        } else {
            Vec::new()
        },
        resource_class: if available {
            LocalAiResourceClass::Cpu
        } else {
            LocalAiResourceClass::RemoteUnavailable
        },
        degraded_state: if available {
            LocalAiDegradedState::None
        } else {
            LocalAiDegradedState::ProviderUnavailable
        },
        last_checked_at: timestamp.0.to_string(),
        unavailable_reason: if available {
            None
        } else {
            Some(
                constants::local_ai_runtime::UNAVAILABLE_REASON_RUNTIME_BINARY_UNCONFIGURED
                    .to_string(),
            )
        },
    }
}
