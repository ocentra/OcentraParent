use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiResourceClass;
use ocentra_parent_agent_protocol::local_ai_runtime::status::LocalModelRuntimeStatus;
use ocentra_parent_agent_protocol::local_ai_runtime::status::LocalProviderAdapterProbe;

use crate::{
    local_ai_runtime_config::LocalAiRuntimeConfigSnapshot,
    local_ai_runtime_config_values::{LocalAiRuntimeText, LocalAiUnavailableReason},
    local_ai_runtime_model_selection::{model_reference_for_request, uses_gpu_resource},
    local_ai_runtime_status::{
        unavailable_local_ai_runtime_status, unavailable_local_provider_adapter_probe,
    },
};

pub(crate) fn unavailable_local_ai_runtime_status_for_model(
    checked_at: impl Into<LocalAiRuntimeText>,
    config: &LocalAiRuntimeConfigSnapshot,
    model_id: &LocalAiRuntimeText,
    reason: LocalAiUnavailableReason,
) -> LocalModelRuntimeStatus {
    let checked_at = checked_at.into();
    let mut status = unavailable_local_ai_runtime_status_with_config(checked_at, config, reason);
    status.model_id =
        if reason.0 == constants::local_ai_runtime::UNAVAILABLE_REASON_MODEL_ID_INVALID {
            constants::local_ai_runtime::MODEL_ID_UNCONFIGURED.to_string()
        } else {
            model_id.0.clone()
        };
    status.model_reference = model_reference_for_request(config, model_id).0;
    status
}

pub(crate) fn unavailable_local_ai_runtime_status_with_config(
    checked_at: impl Into<LocalAiRuntimeText>,
    config: &LocalAiRuntimeConfigSnapshot,
    reason: LocalAiUnavailableReason,
) -> LocalModelRuntimeStatus {
    let mut status = unavailable_local_ai_runtime_status(checked_at.into());
    status.runtime_reference_id =
        constants::local_ai_runtime::RUNTIME_REFERENCE_LOCAL_LLAMA_CLI.to_string();
    status.provider_id = constants::local_ai_runtime::PROVIDER_ID_LOCAL_LLAMA_CLI.to_string();
    status.model_id = config.model_id().0;
    status.model_reference = config.artifact_ref().0;
    status.resource_class = if uses_gpu_resource(config) {
        LocalAiResourceClass::Gpu
    } else {
        LocalAiResourceClass::Cpu
    };
    status.unavailable_reason = Some(reason.0.to_string());
    status
}

pub(crate) fn unavailable_local_provider_adapter_probe_with_reason(
    checked_at: impl Into<LocalAiRuntimeText>,
    reason: LocalAiUnavailableReason,
) -> LocalProviderAdapterProbe {
    let mut probe = unavailable_local_provider_adapter_probe(checked_at.into());
    probe.unavailable_reason = Some(reason.0.to_string());
    probe
}
