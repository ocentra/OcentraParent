use std::path::PathBuf;

use ocentra_parent_agent_protocol::constants;

use crate::{
    local_ai_runtime_config_path::ConfiguredLocalPath,
    local_ai_runtime_config_values::{
        env_flag, env_llama_device, env_llama_gpu_layers, env_path, env_u32, env_u64, env_value,
        safe_ref_or_default,
    },
};

#[derive(Clone, Debug)]
pub struct LocalAiRuntimeConfigSnapshot {
    runtime_binary: ConfiguredLocalPath,
    model_file: ConfiguredLocalPath,
    artifact_ref: String,
    manifest_ref: Option<String>,
    execution_enabled: bool,
    generation_timeout_ms: u64,
    generation_max_tokens: u32,
    runtime_device: Option<String>,
    gpu_layers: Option<String>,
}

impl LocalAiRuntimeConfigSnapshot {
    pub fn from_environment() -> Self {
        Self::from_parts_with_execution(
            env_path(constants::env_var::LOCAL_AI_RUNTIME_BINARY),
            env_path(constants::env_var::LOCAL_AI_MODEL_FILE),
            env_value(constants::env_var::LOCAL_AI_MODEL_ARTIFACT_REF),
            env_value(constants::env_var::LOCAL_AI_MODEL_MANIFEST_REF),
            env_flag(constants::env_var::LOCAL_AI_EXECUTION_ENABLED),
            env_u64(
                constants::env_var::LOCAL_AI_GENERATION_TIMEOUT_MS,
                constants::local_ai_runtime::DEFAULT_GENERATION_TIMEOUT_MS,
            ),
            env_u32(
                constants::env_var::LOCAL_AI_GENERATION_MAX_TOKENS,
                constants::local_ai_runtime::DEFAULT_GENERATION_MAX_TOKENS,
            ),
        )
        .with_acceleration(
            env_llama_device(constants::env_var::LOCAL_AI_RUNTIME_DEVICE),
            env_llama_gpu_layers(constants::env_var::LOCAL_AI_GPU_LAYERS),
        )
    }

    pub fn from_parts(
        runtime_binary: Option<PathBuf>,
        model_file: Option<PathBuf>,
        artifact_ref: Option<String>,
        manifest_ref: Option<String>,
    ) -> Self {
        Self::from_parts_with_execution(
            runtime_binary,
            model_file,
            artifact_ref,
            manifest_ref,
            false,
            constants::local_ai_runtime::DEFAULT_GENERATION_TIMEOUT_MS,
            constants::local_ai_runtime::DEFAULT_GENERATION_MAX_TOKENS,
        )
    }

    pub fn from_parts_with_execution(
        runtime_binary: Option<PathBuf>,
        model_file: Option<PathBuf>,
        artifact_ref: Option<String>,
        manifest_ref: Option<String>,
        execution_enabled: bool,
        generation_timeout_ms: u64,
        generation_max_tokens: u32,
    ) -> Self {
        Self {
            runtime_binary: ConfiguredLocalPath::from_path(runtime_binary.as_deref()),
            model_file: ConfiguredLocalPath::from_path(model_file.as_deref()),
            artifact_ref: safe_ref_or_default(
                artifact_ref,
                constants::local_ai_runtime::MODEL_ARTIFACT_REF_PREFIX,
                constants::local_ai_runtime::MODEL_REFERENCE_LOCAL_GGUF_CONFIGURED,
            ),
            manifest_ref: Some(safe_ref_or_default(
                manifest_ref,
                constants::local_ai_runtime::MODEL_MANIFEST_REF_PREFIX,
                constants::local_ai_runtime::MODEL_MANIFEST_REFERENCE_LOCAL_GGUF_CONFIGURED,
            )),
            execution_enabled,
            generation_timeout_ms,
            generation_max_tokens,
            runtime_device: None,
            gpu_layers: None,
        }
    }

    pub fn with_acceleration(
        mut self,
        runtime_device: Option<String>,
        gpu_layers: Option<String>,
    ) -> Self {
        self.runtime_device = runtime_device;
        self.gpu_layers = gpu_layers;
        self
    }

    pub fn unconfigured() -> Self {
        Self::from_parts(None, None, None, None)
    }

    pub fn runtime_binary(&self) -> &ConfiguredLocalPath {
        &self.runtime_binary
    }

    pub fn model_file(&self) -> &ConfiguredLocalPath {
        &self.model_file
    }

    pub fn artifact_ref(&self) -> &str {
        &self.artifact_ref
    }

    pub fn manifest_ref(&self) -> Option<String> {
        self.manifest_ref.clone()
    }

    pub fn execution_enabled(&self) -> bool {
        self.execution_enabled
    }

    pub fn generation_timeout_ms(&self) -> u64 {
        self.generation_timeout_ms
    }

    pub fn generation_max_tokens(&self) -> u32 {
        self.generation_max_tokens
    }

    pub fn runtime_device(&self) -> Option<&str> {
        self.runtime_device.as_deref()
    }

    pub fn gpu_layers(&self) -> Option<&str> {
        self.gpu_layers.as_deref()
    }

    pub fn uses_gpu_resource(&self) -> bool {
        self.runtime_device().is_some()
            || self
                .gpu_layers()
                .map(gpu_layers_request_acceleration)
                .unwrap_or(false)
    }
}

fn gpu_layers_request_acceleration(value: &str) -> bool {
    value == constants::local_ai_runtime::LLAMA_GPU_LAYERS_ALL
        || value == constants::local_ai_runtime::LLAMA_GPU_LAYERS_AUTO
        || value
            .parse::<u32>()
            .map(|layers| layers > 0)
            .unwrap_or(true)
}
