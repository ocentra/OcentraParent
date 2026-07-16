use ocentra_parent_agent_protocol::constants;

use crate::{
    local_ai_runtime_config::LocalAiRuntimeConfigSnapshot,
    local_ai_runtime_config_values::{LocalAiRuntimePath, LocalAiRuntimeText},
    local_ai_runtime_status::local_ai_runtime_status_for_model_from_config,
    local_ai_runtime_status_tests::{remove_temp_file, write_temp_file},
};

#[test]
fn requested_unsupported_model_reports_capacity_unavailable_without_runtime_probe() {
    let binary = write_temp_file(constants::local_ai_runtime::PROVIDER_ID_LOCAL_LLAMA_CLI);
    let model = write_temp_file(constants::local_ai_runtime::MODEL_ID_DEFAULT_GEMMA_4);
    let config = LocalAiRuntimeConfigSnapshot::from_parts_with_execution(
        Some(LocalAiRuntimePath(binary.clone())),
        Some(LocalAiRuntimePath(model.clone())),
        None,
        None,
        true,
        constants::local_ai_runtime::DEFAULT_GENERATION_TIMEOUT_MS,
        constants::local_ai_runtime::DEFAULT_GENERATION_MAX_TOKENS,
    );

    let (status, probe, _cache) = local_ai_runtime_status_for_model_from_config(
        constants::local_ai_runtime::TEST_CHECKED_AT.to_string(),
        &config,
        Some(LocalAiRuntimeText(
            constants::local_ai_runtime::TEST_UNSUPPORTED_MODEL_ID.to_string(),
        )),
    );

    assert_eq!(
        status.model_id,
        constants::local_ai_runtime::TEST_UNSUPPORTED_MODEL_ID
    );
    assert_eq!(
        status.model_reference,
        constants::local_ai_runtime::MODEL_REFERENCE_UNCONFIGURED
    );
    assert_eq!(
        status.unavailable_reason,
        Some(constants::local_ai_runtime::UNAVAILABLE_REASON_MODEL_UNSUPPORTED.to_string())
    );
    assert_eq!(
        probe.unavailable_reason,
        Some(constants::local_ai_runtime::UNAVAILABLE_REASON_MODEL_UNSUPPORTED.to_string())
    );
    assert!(!probe.execution_allowed);

    remove_temp_file(binary);
    remove_temp_file(model);
}
