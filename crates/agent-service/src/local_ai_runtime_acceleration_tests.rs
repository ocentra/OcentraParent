use ocentra_parent_agent_protocol::{constants, LocalAiResourceClass};

use crate::{
    local_ai_runtime_config::LocalAiRuntimeConfigSnapshot,
    local_ai_runtime_status_tests::{remove_temp_file, write_temp_file},
};

#[test]
fn gpu_runtime_configuration_reports_gpu_resource_class() {
    let binary = write_temp_file(constants::local_ai_runtime::PROVIDER_ID_LOCAL_LLAMA_CLI);
    let model = write_temp_file(constants::local_ai_runtime::MODEL_ID_LOCAL_GGUF_CONFIGURED);
    let config = LocalAiRuntimeConfigSnapshot::from_parts_with_acceleration(
        Some(binary.clone()),
        Some(model.clone()),
        None,
        None,
        true,
        Some(constants::local_ai_runtime::RESOURCE_GPU.to_string()),
        Some(constants::local_ai_runtime::LLAMA_GPU_LAYERS_ALL.to_string()),
        constants::local_ai_runtime::DEFAULT_GENERATION_TIMEOUT_MS,
        constants::local_ai_runtime::DEFAULT_GENERATION_MAX_TOKENS,
    );

    let (status, _probe, _cache) =
        crate::local_ai_runtime_status::local_ai_runtime_status_from_config(
            constants::local_ai_runtime::TEST_CHECKED_AT.to_string(),
            &config,
        );

    assert_eq!(status.resource_class, LocalAiResourceClass::Gpu);
    assert!(config.runtime_device().is_some());
    assert_eq!(
        config.gpu_layers(),
        Some(constants::local_ai_runtime::LLAMA_GPU_LAYERS_ALL)
    );

    remove_temp_file(binary);
    remove_temp_file(model);
}
