use ocentra_parent_agent_protocol::constants;

use crate::{
    local_ai_runtime_config::LocalAiRuntimeConfigSnapshot,
    local_ai_runtime_config_values::is_safe_local_ai_model_id,
};

pub(crate) fn requested_model_unavailable_reason(
    config: &LocalAiRuntimeConfigSnapshot,
    requested_model_id: &str,
) -> Option<&'static str> {
    if !is_safe_local_ai_model_id(requested_model_id) {
        return Some(constants::local_ai_runtime::UNAVAILABLE_REASON_MODEL_ID_INVALID);
    }

    if requested_model_id != config.model_id() {
        return Some(constants::local_ai_runtime::UNAVAILABLE_REASON_MODEL_UNSUPPORTED);
    }

    None
}

pub(crate) fn model_reference_for_request<'a>(
    config: &'a LocalAiRuntimeConfigSnapshot,
    requested_model_id: &str,
) -> &'a str {
    if requested_model_id == config.model_id() {
        config.artifact_ref()
    } else {
        constants::local_ai_runtime::MODEL_REFERENCE_UNCONFIGURED
    }
}

pub(crate) fn uses_gpu_resource(config: &LocalAiRuntimeConfigSnapshot) -> bool {
    config.runtime_device().is_some()
        || config
            .gpu_layers()
            .map(gpu_layers_request_acceleration)
            .unwrap_or(false)
}

fn gpu_layers_request_acceleration(value: &str) -> bool {
    value == constants::local_ai_runtime::LLAMA_GPU_LAYERS_ALL
        || value == constants::local_ai_runtime::LLAMA_GPU_LAYERS_AUTO
        || value
            .parse::<u32>()
            .map(|layers| layers > 0)
            .unwrap_or(true)
}
