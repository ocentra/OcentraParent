use ocentra_parent_agent_protocol::constants;

use crate::local_ai_runtime_config_values::{
    env_flag, env_llama_device, env_llama_gpu_layers, env_value, LocalAiRuntimeEnvVar,
    LocalAiRuntimeText,
};

#[derive(Clone, Debug, Default)]
pub(crate) struct LocalAiRuntimeAccelerationConfig {
    pub(crate) runtime_device: Option<String>,
    pub(crate) gpu_layers: Option<String>,
    pub(crate) split_mode: Option<String>,
    pub(crate) tensor_split: Option<String>,
    pub(crate) main_gpu: Option<String>,
    pub(crate) fit: Option<String>,
    pub(crate) fit_target: Option<String>,
    pub(crate) op_offload: Option<bool>,
    pub(crate) cpu_moe: bool,
    pub(crate) cpu_moe_layers: Option<String>,
}

impl LocalAiRuntimeAccelerationConfig {
    pub(crate) fn from_environment() -> Self {
        Self {
            runtime_device: env_llama_device(LocalAiRuntimeEnvVar(
                constants::env_var::LOCAL_AI_RUNTIME_DEVICE,
            ))
            .map(|value| value.0),
            gpu_layers: env_llama_gpu_layers(LocalAiRuntimeEnvVar(
                constants::env_var::LOCAL_AI_GPU_LAYERS,
            ))
            .map(|value| value.0),
            split_mode: env_llama_split_mode(LocalAiRuntimeEnvVar(
                constants::env_var::LOCAL_AI_SPLIT_MODE,
            ))
            .map(|value| value.0),
            tensor_split: env_llama_numeric_list(LocalAiRuntimeEnvVar(
                constants::env_var::LOCAL_AI_TENSOR_SPLIT,
            ))
            .map(|value| value.0),
            main_gpu: env_llama_non_negative_integer(LocalAiRuntimeEnvVar(
                constants::env_var::LOCAL_AI_MAIN_GPU,
            ))
            .map(|value| value.0),
            fit: env_llama_toggle(LocalAiRuntimeEnvVar(constants::env_var::LOCAL_AI_FIT))
                .map(|value| value.0),
            fit_target: env_llama_numeric_list(LocalAiRuntimeEnvVar(
                constants::env_var::LOCAL_AI_FIT_TARGET,
            ))
            .map(|value| value.0),
            op_offload: env_llama_op_offload(LocalAiRuntimeEnvVar(
                constants::env_var::LOCAL_AI_OP_OFFLOAD,
            )),
            cpu_moe: env_flag(LocalAiRuntimeEnvVar(constants::env_var::LOCAL_AI_CPU_MOE)),
            cpu_moe_layers: env_llama_positive_integer(LocalAiRuntimeEnvVar(
                constants::env_var::LOCAL_AI_CPU_MOE_LAYERS,
            ))
            .map(|value| value.0),
        }
    }

    pub(crate) fn uses_gpu_runtime(&self) -> bool {
        if self
            .runtime_device
            .as_deref()
            .map(|value| runtime_device_disables_gpu(&LocalAiRuntimeText(value.to_string())))
            .unwrap_or(false)
        {
            return false;
        }

        self.runtime_device
            .as_deref()
            .map(|value| runtime_device_requests_gpu(&LocalAiRuntimeText(value.to_string())))
            .unwrap_or(false)
            || self
                .gpu_layers
                .as_deref()
                .map(|value| {
                    gpu_layers_request_acceleration(&LocalAiRuntimeText(value.to_string()))
                })
                .unwrap_or(false)
            || self.split_mode.is_some()
            || self.tensor_split.is_some()
            || self.main_gpu.is_some()
            || self.op_offload.unwrap_or(false)
    }
}

pub(crate) fn gpu_layers_request_acceleration(value: &LocalAiRuntimeText) -> bool {
    value.0 == constants::local_ai_runtime::LLAMA_GPU_LAYERS_ALL
        || value.0 == constants::local_ai_runtime::LLAMA_GPU_LAYERS_AUTO
        || value
            .0
            .parse::<u32>()
            .map(|layers| layers > 0)
            .unwrap_or(false)
}

pub(crate) fn runtime_device_requests_gpu(value: &LocalAiRuntimeText) -> bool {
    !runtime_device_disables_gpu(value)
}

fn runtime_device_disables_gpu(value: &LocalAiRuntimeText) -> bool {
    value
        .0
        .eq_ignore_ascii_case(constants::local_ai_runtime::LLAMA_DEVICE_NONE)
}

fn env_llama_split_mode(env_var_name: LocalAiRuntimeEnvVar) -> Option<LocalAiRuntimeText> {
    env_value(env_var_name).filter(|value| {
        value.0 == constants::local_ai_runtime::LLAMA_SPLIT_MODE_NONE
            || value.0 == constants::local_ai_runtime::LLAMA_SPLIT_MODE_LAYER
            || value.0 == constants::local_ai_runtime::LLAMA_SPLIT_MODE_ROW
            || value.0 == constants::local_ai_runtime::LLAMA_SPLIT_MODE_TENSOR
    })
}

fn env_llama_numeric_list(env_var_name: LocalAiRuntimeEnvVar) -> Option<LocalAiRuntimeText> {
    env_value(env_var_name).filter(is_safe_numeric_list)
}

fn env_llama_non_negative_integer(
    env_var_name: LocalAiRuntimeEnvVar,
) -> Option<LocalAiRuntimeText> {
    env_value(env_var_name).filter(|value| value.0.parse::<u32>().is_ok())
}

fn env_llama_positive_integer(env_var_name: LocalAiRuntimeEnvVar) -> Option<LocalAiRuntimeText> {
    env_value(env_var_name).filter(|value| {
        value
            .0
            .parse::<u32>()
            .map(|index| index > 0)
            .unwrap_or(false)
    })
}

fn env_llama_toggle(env_var_name: LocalAiRuntimeEnvVar) -> Option<LocalAiRuntimeText> {
    env_value(env_var_name).filter(|value| {
        value.0 == constants::local_ai_runtime::LLAMA_TOGGLE_ON
            || value.0 == constants::local_ai_runtime::LLAMA_TOGGLE_OFF
    })
}

fn env_llama_op_offload(env_var_name: LocalAiRuntimeEnvVar) -> Option<bool> {
    env_llama_toggle(env_var_name)
        .map(|value| value.0 == constants::local_ai_runtime::LLAMA_TOGGLE_ON)
}

fn is_safe_numeric_list(candidate: &LocalAiRuntimeText) -> bool {
    !candidate.0.is_empty()
        && candidate.0.len() <= 64
        && candidate.0.split(constants::delimiter::LIST).all(|part| {
            !part.is_empty() && part.parse::<u32>().map(|value| value > 0).unwrap_or(false)
        })
}
