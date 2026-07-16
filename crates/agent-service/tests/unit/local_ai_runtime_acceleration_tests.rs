use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiResourceClass;
use std::ffi::OsString as TestOsString;
use std::path::PathBuf as TestPathBuf;
use std::primitive::str as TestStr;

use crate::{
    local_ai_chat_generation_args::llama_acceleration_args,
    local_ai_runtime_acceleration_config::LocalAiRuntimeAccelerationConfig,
    local_ai_runtime_config::LocalAiRuntimeConfigSnapshot,
    local_ai_runtime_config_values::{LocalAiRuntimePath, LocalAiRuntimeText},
    local_ai_runtime_status::local_ai_runtime_status_from_config,
    local_ai_runtime_status_tests::{remove_temp_file, write_temp_file},
};

#[test]
fn gpu_runtime_configuration_reports_gpu_resource_class() {
    let binary = write_temp_file(constants::local_ai_runtime::PROVIDER_ID_LOCAL_LLAMA_CLI);
    let model = write_temp_file(constants::local_ai_runtime::MODEL_ID_LOCAL_GGUF_CONFIGURED);
    let config = LocalAiRuntimeConfigSnapshot::from_parts_with_execution(
        Some(LocalAiRuntimePath(binary.clone())),
        Some(LocalAiRuntimePath(model.clone())),
        None,
        None,
        true,
        constants::local_ai_runtime::DEFAULT_GENERATION_TIMEOUT_MS,
        constants::local_ai_runtime::DEFAULT_GENERATION_MAX_TOKENS,
    )
    .with_acceleration_config(LocalAiRuntimeAccelerationConfig {
        runtime_device: Some(constants::local_ai_runtime::TEST_RUNTIME_DEVICE_VULKAN0.to_string()),
        gpu_layers: Some(constants::local_ai_runtime::LLAMA_GPU_LAYERS_ALL.to_string()),
        ..LocalAiRuntimeAccelerationConfig::default()
    });

    let (status, _probe, _cache) = local_ai_runtime_status_from_config(
        constants::local_ai_runtime::TEST_CHECKED_AT.to_string(),
        &config,
    );

    assert_eq!(status.resource_class, LocalAiResourceClass::Gpu);
    assert_eq!(
        config.acceleration().runtime_device.as_deref(),
        Some(constants::local_ai_runtime::TEST_RUNTIME_DEVICE_VULKAN0)
    );
    assert_eq!(
        config.acceleration().gpu_layers.as_deref(),
        Some(constants::local_ai_runtime::LLAMA_GPU_LAYERS_ALL)
    );

    remove_temp_file(binary);
    remove_temp_file(model);
}

#[test]
fn split_runtime_configuration_reports_gpu_resource_class() {
    let binary = write_temp_file(constants::local_ai_runtime::PROVIDER_ID_LOCAL_LLAMA_CLI);
    let model = write_temp_file(constants::local_ai_runtime::MODEL_ID_LOCAL_GGUF_CONFIGURED);
    let config = execution_config(binary.clone(), model.clone()).with_acceleration_config(
        LocalAiRuntimeAccelerationConfig {
            gpu_layers: Some(constants::local_ai_runtime::TEST_GPU_LAYERS_12.to_string()),
            split_mode: Some(constants::local_ai_runtime::LLAMA_SPLIT_MODE_LAYER.to_string()),
            tensor_split: Some(constants::local_ai_runtime::TEST_TENSOR_SPLIT_DUAL.to_string()),
            main_gpu: Some(constants::local_ai_runtime::TEST_MAIN_GPU_1.to_string()),
            ..LocalAiRuntimeAccelerationConfig::default()
        },
    );

    let (status, _probe, _cache) = local_ai_runtime_status_from_config(
        constants::local_ai_runtime::TEST_CHECKED_AT.to_string(),
        &config,
    );

    assert_eq!(status.resource_class, LocalAiResourceClass::Gpu);
    assert_eq!(
        config.acceleration().tensor_split.as_deref(),
        Some(constants::local_ai_runtime::TEST_TENSOR_SPLIT_DUAL)
    );

    remove_temp_file(binary);
    remove_temp_file(model);
}

#[test]
fn llama_acceleration_args_include_split_and_offload_controls() {
    let config = LocalAiRuntimeConfigSnapshot::unconfigured().with_acceleration_config(
        LocalAiRuntimeAccelerationConfig {
            runtime_device: Some(
                constants::local_ai_runtime::TEST_RUNTIME_DEVICE_VULKAN0.to_string(),
            ),
            gpu_layers: Some(constants::local_ai_runtime::TEST_GPU_LAYERS_12.to_string()),
            split_mode: Some(constants::local_ai_runtime::LLAMA_SPLIT_MODE_LAYER.to_string()),
            tensor_split: Some(constants::local_ai_runtime::TEST_TENSOR_SPLIT_SINGLE.to_string()),
            main_gpu: Some(constants::local_ai_runtime::TEST_MAIN_GPU_0.to_string()),
            fit: Some(constants::local_ai_runtime::LLAMA_TOGGLE_ON.to_string()),
            fit_target: Some(constants::local_ai_runtime::TEST_FIT_TARGET_512.to_string()),
            op_offload: Some(true),
            cpu_moe: true,
            cpu_moe_layers: Some(constants::local_ai_runtime::TEST_CPU_MOE_LAYERS_2.to_string()),
        },
    );

    let args = llama_acceleration_args(&config);

    assert_eq!(
        args.into_iter().collect::<Vec<_>>(),
        [
            constants::local_ai_runtime::LLAMA_ARG_DEVICE,
            constants::local_ai_runtime::TEST_RUNTIME_DEVICE_VULKAN0,
            constants::local_ai_runtime::LLAMA_ARG_GPU_LAYERS,
            constants::local_ai_runtime::TEST_GPU_LAYERS_12,
            constants::local_ai_runtime::LLAMA_ARG_SPLIT_MODE,
            constants::local_ai_runtime::LLAMA_SPLIT_MODE_LAYER,
            constants::local_ai_runtime::LLAMA_ARG_TENSOR_SPLIT,
            constants::local_ai_runtime::TEST_TENSOR_SPLIT_SINGLE,
            constants::local_ai_runtime::LLAMA_ARG_MAIN_GPU,
            constants::local_ai_runtime::TEST_MAIN_GPU_0,
            constants::local_ai_runtime::LLAMA_ARG_FIT,
            constants::local_ai_runtime::LLAMA_TOGGLE_ON,
            constants::local_ai_runtime::LLAMA_ARG_FIT_TARGET,
            constants::local_ai_runtime::TEST_FIT_TARGET_512,
            constants::local_ai_runtime::LLAMA_ARG_OP_OFFLOAD,
            constants::local_ai_runtime::LLAMA_ARG_CPU_MOE,
            constants::local_ai_runtime::LLAMA_ARG_CPU_MOE_LAYERS,
            constants::local_ai_runtime::TEST_CPU_MOE_LAYERS_2,
        ]
        .map(LocalAiRuntimeText::from)
    );
}

#[test]
fn acceleration_config_from_environment_parses_safe_gpu_controls() {
    let previous_device = std::env::var_os(constants::env_var::LOCAL_AI_RUNTIME_DEVICE);
    let previous_layers = std::env::var_os(constants::env_var::LOCAL_AI_GPU_LAYERS);
    std::env::set_var(
        constants::env_var::LOCAL_AI_RUNTIME_DEVICE,
        constants::local_ai_runtime::LLAMA_DEVICE_NONE,
    );
    std::env::set_var(
        constants::env_var::LOCAL_AI_GPU_LAYERS,
        constants::local_ai_runtime::LLAMA_GPU_LAYERS_ALL,
    );

    let config = LocalAiRuntimeAccelerationConfig::from_environment();

    restore_env_var(constants::env_var::LOCAL_AI_RUNTIME_DEVICE, previous_device);
    restore_env_var(constants::env_var::LOCAL_AI_GPU_LAYERS, previous_layers);

    assert_eq!(
        config.runtime_device.as_deref(),
        Some(constants::local_ai_runtime::LLAMA_DEVICE_NONE)
    );
    assert_eq!(
        config.gpu_layers.as_deref(),
        Some(constants::local_ai_runtime::LLAMA_GPU_LAYERS_ALL)
    );
    assert!(!config.uses_gpu_runtime());
}

fn execution_config(binary: TestPathBuf, model: TestPathBuf) -> LocalAiRuntimeConfigSnapshot {
    LocalAiRuntimeConfigSnapshot::from_parts_with_execution(
        Some(LocalAiRuntimePath(binary)),
        Some(LocalAiRuntimePath(model)),
        None,
        None,
        true,
        constants::local_ai_runtime::DEFAULT_GENERATION_TIMEOUT_MS,
        constants::local_ai_runtime::DEFAULT_GENERATION_MAX_TOKENS,
    )
}

fn restore_env_var(env_var_name: &TestStr, value: Option<TestOsString>) {
    match value {
        Some(previous) => std::env::set_var(env_var_name, previous),
        None => std::env::remove_var(env_var_name),
    }
}
