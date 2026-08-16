use ocentra_parent_agent_protocol::constants;

use crate::{
    local_ai_runtime_config::LocalAiRuntimeConfigSnapshot,
    local_ai_runtime_config_values::validation::is_safe_local_ai_model_id,
    local_ai_runtime_config_values::{LocalAiRuntimeText, LocalAiUnavailableReason},
};

pub(crate) fn requested_model_unavailable_reason(
    config: &LocalAiRuntimeConfigSnapshot,
    requested_model_id: &LocalAiRuntimeText,
) -> Option<LocalAiUnavailableReason> {
    if !is_safe_local_ai_model_id(requested_model_id) {
        return Some(LocalAiUnavailableReason(
            constants::local_ai_runtime::UNAVAILABLE_REASON_MODEL_ID_INVALID,
        ));
    }

    if requested_model_id.0 != config.model_id().0 {
        return Some(LocalAiUnavailableReason(
            constants::local_ai_runtime::UNAVAILABLE_REASON_MODEL_UNSUPPORTED,
        ));
    }

    None
}

pub(crate) fn model_reference_for_request(
    config: &LocalAiRuntimeConfigSnapshot,
    requested_model_id: &LocalAiRuntimeText,
) -> LocalAiRuntimeText {
    if requested_model_id.0 == config.model_id().0 {
        config.artifact_ref()
    } else {
        LocalAiRuntimeText(constants::local_ai_runtime::MODEL_REFERENCE_UNCONFIGURED.to_string())
    }
}

pub(crate) fn uses_gpu_resource(config: &LocalAiRuntimeConfigSnapshot) -> bool {
    config.acceleration().uses_gpu_runtime()
}
