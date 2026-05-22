use ocentra_parent_agent_protocol::constants;

use crate::{
    local_ai_model_registry::known_model_for_id,
    local_ai_runtime_config::LocalAiRuntimeConfigSnapshot,
    local_ai_runtime_config_parts::{LocalAiRuntimeConfigParts, LocalAiRuntimeModelConfig},
    local_ai_runtime_config_values::{
        env_flag, env_llama_device, env_llama_gpu_layers, env_llama_release_tag,
        env_local_ai_model_id, env_path, env_u32, env_u64, env_value,
    },
    local_ai_runtime_install_plan::{
        default_install_plan_from_environment, LocalAiRequiredArtifactStatus,
    },
};

pub(crate) fn runtime_config_from_environment() -> LocalAiRuntimeConfigSnapshot {
    let release_tag = env_llama_release_tag(constants::env_var::LOCAL_AI_LLAMA_CPP_RELEASE_TAG);
    let model_id = env_local_ai_model_id(constants::env_var::LOCAL_AI_MODEL_ID);
    let runtime_device = env_llama_device(constants::env_var::LOCAL_AI_RUNTIME_DEVICE);
    let gpu_layers = env_llama_gpu_layers(constants::env_var::LOCAL_AI_GPU_LAYERS);
    let execution_enabled = env_flag(constants::env_var::LOCAL_AI_EXECUTION_ENABLED);
    let install_plan = default_install_plan_from_environment(
        &release_tag,
        runtime_device.as_deref(),
        gpu_layers.as_deref(),
    );
    if let Some(plan) = install_plan.as_ref() {
        if plan.runtime_status != LocalAiRequiredArtifactStatus::Unsupported {
            let _ = plan.ensure_cache_directories();
        }
    }
    let runtime_binary = env_path(constants::env_var::LOCAL_AI_RUNTIME_BINARY).or_else(|| {
        install_plan
            .as_ref()
            .and_then(|plan| plan.runtime.as_ref())
            .map(|runtime| runtime.binary_path.clone())
    });
    let known_model = known_model_for_id(&model_id);
    let model_file = env_path(constants::env_var::LOCAL_AI_MODEL_FILE).or_else(|| {
        if execution_enabled && known_model.is_some() {
            install_plan
                .as_ref()
                .map(|plan| plan.default_model.model_path.clone())
        } else {
            None
        }
    });
    LocalAiRuntimeConfigSnapshot::from_config_parts(LocalAiRuntimeConfigParts {
        runtime_binary,
        model: runtime_model_config(model_id, model_file),
        execution_enabled,
        generation_timeout_ms: env_u64(
            constants::env_var::LOCAL_AI_GENERATION_TIMEOUT_MS,
            constants::local_ai_runtime::DEFAULT_GENERATION_TIMEOUT_MS,
        ),
        generation_max_tokens: env_u32(
            constants::env_var::LOCAL_AI_GENERATION_MAX_TOKENS,
            constants::local_ai_runtime::DEFAULT_GENERATION_MAX_TOKENS,
        ),
    })
    .with_acceleration(runtime_device, gpu_layers)
}

fn runtime_model_config(
    model_id: String,
    model_file: Option<std::path::PathBuf>,
) -> LocalAiRuntimeModelConfig {
    let known_model = known_model_for_id(&model_id);
    LocalAiRuntimeModelConfig {
        model_id,
        model_file,
        artifact_ref: env_value(constants::env_var::LOCAL_AI_MODEL_ARTIFACT_REF),
        manifest_ref: env_value(constants::env_var::LOCAL_AI_MODEL_MANIFEST_REF),
        default_artifact_ref: known_model
            .map(|model| model.artifact_ref)
            .unwrap_or(constants::local_ai_runtime::MODEL_REFERENCE_LOCAL_GGUF_CONFIGURED),
        default_manifest_ref: known_model
            .map(|model| model.manifest_ref)
            .unwrap_or(constants::local_ai_runtime::MODEL_MANIFEST_REFERENCE_LOCAL_GGUF_CONFIGURED),
    }
}
