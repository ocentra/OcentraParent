use ocentra_parent_agent_protocol::constants;

use crate::{
    local_ai_model_registry::known_model_for_id,
    local_ai_runtime_acceleration_config::LocalAiRuntimeAccelerationConfig,
    local_ai_runtime_config::LocalAiRuntimeConfigSnapshot,
    local_ai_runtime_config_parts::{LocalAiRuntimeConfigParts, LocalAiRuntimeModelConfig},
    local_ai_runtime_config_values::{
        env_flag, env_llama_release_tag, env_local_ai_model_id, env_path, env_u32, env_u64,
        env_value, LocalAiRuntimeEnvVar, LocalAiRuntimePath, LocalAiRuntimeText,
    },
    local_ai_runtime_install_plan::{
        default_install_plan_from_environment, LocalAiRequiredArtifactStatus,
    },
};

pub(crate) fn runtime_config_from_environment() -> LocalAiRuntimeConfigSnapshot {
    let release_tag = env_llama_release_tag(LocalAiRuntimeEnvVar(
        constants::env_var::LOCAL_AI_LLAMA_CPP_RELEASE_TAG,
    ));
    let model_id =
        env_local_ai_model_id(LocalAiRuntimeEnvVar(constants::env_var::LOCAL_AI_MODEL_ID));
    let acceleration = LocalAiRuntimeAccelerationConfig::from_environment();
    let execution_enabled = env_flag(LocalAiRuntimeEnvVar(
        constants::env_var::LOCAL_AI_EXECUTION_ENABLED,
    ));
    let install_plan = default_install_plan_from_environment(release_tag.0.as_str(), &acceleration);
    if let Some(plan) = install_plan.as_ref() {
        if plan.runtime_status != LocalAiRequiredArtifactStatus::Unsupported {
            let _ = plan.ensure_cache_directories();
        }
    }
    let runtime_binary = env_path(LocalAiRuntimeEnvVar(
        constants::env_var::LOCAL_AI_RUNTIME_BINARY,
    ))
    .map(|path| path.0)
    .or_else(|| {
        install_plan
            .as_ref()
            .and_then(|plan| plan.runtime.as_ref())
            .map(|runtime| runtime.binary_path.clone())
    });
    let known_model = known_model_for_id(&model_id);
    let model_file = env_path(LocalAiRuntimeEnvVar(
        constants::env_var::LOCAL_AI_MODEL_FILE,
    ))
    .map(|path| path.0)
    .or_else(|| {
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
        model: runtime_model_config(model_id, model_file.map(LocalAiRuntimePath)),
        execution_enabled,
        generation_timeout_ms: env_u64(
            LocalAiRuntimeEnvVar(constants::env_var::LOCAL_AI_GENERATION_TIMEOUT_MS),
            constants::local_ai_runtime::DEFAULT_GENERATION_TIMEOUT_MS,
        ),
        generation_max_tokens: env_u32(
            LocalAiRuntimeEnvVar(constants::env_var::LOCAL_AI_GENERATION_MAX_TOKENS),
            constants::local_ai_runtime::DEFAULT_GENERATION_MAX_TOKENS,
        ),
    })
    .with_acceleration_config(acceleration)
}

fn runtime_model_config(
    model_id: LocalAiRuntimeText,
    model_file: Option<LocalAiRuntimePath>,
) -> LocalAiRuntimeModelConfig {
    let known_model = known_model_for_id(&model_id);
    LocalAiRuntimeModelConfig {
        model_id: model_id.0,
        model_file: model_file.map(|path| path.0),
        artifact_ref: env_value(LocalAiRuntimeEnvVar(
            constants::env_var::LOCAL_AI_MODEL_ARTIFACT_REF,
        ))
        .map(|value| value.0),
        manifest_ref: env_value(LocalAiRuntimeEnvVar(
            constants::env_var::LOCAL_AI_MODEL_MANIFEST_REF,
        ))
        .map(|value| value.0),
        default_artifact_ref: known_model
            .map(|model| model.artifact_ref)
            .unwrap_or(constants::local_ai_runtime::MODEL_REFERENCE_LOCAL_GGUF_CONFIGURED),
        default_manifest_ref: known_model
            .map(|model| model.manifest_ref)
            .unwrap_or(constants::local_ai_runtime::MODEL_MANIFEST_REFERENCE_LOCAL_GGUF_CONFIGURED),
    }
}
