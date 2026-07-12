use ocentra_parent_agent_protocol::constants;

use crate::{
    local_ai_runtime_config::LocalAiRuntimeConfigSnapshot,
    local_ai_runtime_config_values::LocalAiUnavailableReason,
};

pub(crate) fn runtime_configuration_unavailable_reason(
    config: &LocalAiRuntimeConfigSnapshot,
) -> Option<LocalAiUnavailableReason> {
    if !config.runtime_binary().is_configured() {
        return Some(LocalAiUnavailableReason(
            constants::local_ai_runtime::UNAVAILABLE_REASON_RUNTIME_BINARY_UNCONFIGURED,
        ));
    }

    if !config.runtime_binary().exists() {
        return Some(LocalAiUnavailableReason(
            constants::local_ai_runtime::UNAVAILABLE_REASON_RUNTIME_BINARY_MISSING,
        ));
    }

    if !config.model_file().is_configured() {
        return Some(LocalAiUnavailableReason(
            constants::local_ai_runtime::UNAVAILABLE_REASON_MODEL_FILE_UNCONFIGURED,
        ));
    }

    if !config.model_file().exists() {
        return Some(LocalAiUnavailableReason(
            constants::local_ai_runtime::UNAVAILABLE_REASON_MODEL_FILE_MISSING,
        ));
    }

    None
}
