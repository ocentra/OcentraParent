use std::{
    env,
    path::{Path, PathBuf},
};

use ocentra_parent_agent_protocol::constants;

#[path = "local_ai_runtime_config_values/validation.rs"]
pub(crate) mod validation;

#[derive(Clone, Copy, Debug)]
pub(crate) struct LocalAiRuntimeEnvVar(pub(crate) &'static str);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LocalAiRuntimeText(pub(crate) String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LocalAiRuntimePath(pub(crate) PathBuf);

#[derive(Clone, Copy, Debug)]
pub(crate) struct LocalAiUnavailableReason(pub(crate) &'static str);

impl<T> From<T> for LocalAiRuntimeText
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl AsRef<Path> for LocalAiRuntimePath {
    fn as_ref(&self) -> &Path {
        self.0.as_path()
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct LocalAiRuntimeRefPrefix(pub(crate) &'static str);

pub(crate) fn env_path(env_var_name: LocalAiRuntimeEnvVar) -> Option<LocalAiRuntimePath> {
    env_value(env_var_name)
        .map(|value| PathBuf::from(value.0))
        .map(LocalAiRuntimePath)
}

pub(crate) fn env_value(env_var_name: LocalAiRuntimeEnvVar) -> Option<LocalAiRuntimeText> {
    env::var(env_var_name.0)
        .ok()
        .filter(|value| !value.trim().is_empty())
        .map(LocalAiRuntimeText)
}

pub(crate) fn env_flag(env_var_name: LocalAiRuntimeEnvVar) -> bool {
    env_value(env_var_name)
        .map(|value| value.0.eq_ignore_ascii_case(constants::value::TRUE))
        .unwrap_or(false)
}

pub(crate) fn env_u64(env_var_name: LocalAiRuntimeEnvVar, fallback: u64) -> u64 {
    env_value(env_var_name)
        .and_then(|value| value.0.parse::<u64>().ok())
        .unwrap_or(fallback)
}

pub(crate) fn env_u32(env_var_name: LocalAiRuntimeEnvVar, fallback: u32) -> u32 {
    env_value(env_var_name)
        .and_then(|value| value.0.parse::<u32>().ok())
        .unwrap_or(fallback)
}

pub(crate) fn env_llama_device(env_var_name: LocalAiRuntimeEnvVar) -> Option<LocalAiRuntimeText> {
    env_value(env_var_name).filter(validation::is_safe_llama_selector)
}

pub(crate) fn env_llama_gpu_layers(
    env_var_name: LocalAiRuntimeEnvVar,
) -> Option<LocalAiRuntimeText> {
    env_value(env_var_name).filter(|value| {
        value.0 == constants::local_ai_runtime::LLAMA_GPU_LAYERS_ALL
            || value.0 == constants::local_ai_runtime::LLAMA_GPU_LAYERS_AUTO
            || value.0.parse::<u32>().is_ok()
    })
}

pub(crate) fn env_llama_release_tag(env_var_name: LocalAiRuntimeEnvVar) -> LocalAiRuntimeText {
    env_value(env_var_name)
        .filter(validation::is_safe_llama_release_tag)
        .unwrap_or_else(|| {
            LocalAiRuntimeText(
                constants::local_ai_runtime::DEFAULT_LLAMA_CPP_RELEASE_TAG.to_string(),
            )
        })
}

pub(crate) fn env_local_ai_model_id(env_var_name: LocalAiRuntimeEnvVar) -> LocalAiRuntimeText {
    env_value(env_var_name)
        .filter(validation::is_safe_local_ai_model_id)
        .unwrap_or_else(|| {
            LocalAiRuntimeText(constants::local_ai_runtime::MODEL_ID_DEFAULT_GEMMA_4.to_string())
        })
}
