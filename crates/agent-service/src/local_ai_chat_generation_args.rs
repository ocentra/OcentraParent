use ocentra_parent_agent_protocol::constants;

use crate::local_ai_runtime_config::LocalAiRuntimeConfigSnapshot;

pub(crate) fn llama_acceleration_args(config: &LocalAiRuntimeConfigSnapshot) -> Vec<String> {
    let acceleration = config.acceleration();
    let mut args = Vec::new();
    append_arg_value(
        &mut args,
        constants::local_ai_runtime::LLAMA_ARG_DEVICE,
        acceleration.runtime_device.as_deref(),
    );
    append_arg_value(
        &mut args,
        constants::local_ai_runtime::LLAMA_ARG_GPU_LAYERS,
        acceleration.gpu_layers.as_deref(),
    );
    append_arg_value(
        &mut args,
        constants::local_ai_runtime::LLAMA_ARG_SPLIT_MODE,
        acceleration.split_mode.as_deref(),
    );
    append_arg_value(
        &mut args,
        constants::local_ai_runtime::LLAMA_ARG_TENSOR_SPLIT,
        acceleration.tensor_split.as_deref(),
    );
    append_arg_value(
        &mut args,
        constants::local_ai_runtime::LLAMA_ARG_MAIN_GPU,
        acceleration.main_gpu.as_deref(),
    );
    append_arg_value(
        &mut args,
        constants::local_ai_runtime::LLAMA_ARG_FIT,
        acceleration.fit.as_deref(),
    );
    append_arg_value(
        &mut args,
        constants::local_ai_runtime::LLAMA_ARG_FIT_TARGET,
        acceleration.fit_target.as_deref(),
    );
    append_op_offload_arg(&mut args, acceleration.op_offload);
    append_cpu_moe_args(&mut args, config);
    args
}

fn append_op_offload_arg(args: &mut Vec<String>, op_offload: Option<bool>) {
    if let Some(op_offload) = op_offload {
        args.push(
            if op_offload {
                constants::local_ai_runtime::LLAMA_ARG_OP_OFFLOAD
            } else {
                constants::local_ai_runtime::LLAMA_ARG_NO_OP_OFFLOAD
            }
            .to_string(),
        );
    }
}

fn append_cpu_moe_args(args: &mut Vec<String>, config: &LocalAiRuntimeConfigSnapshot) {
    let acceleration = config.acceleration();
    if acceleration.cpu_moe {
        args.push(constants::local_ai_runtime::LLAMA_ARG_CPU_MOE.to_string());
    }
    append_arg_value(
        args,
        constants::local_ai_runtime::LLAMA_ARG_CPU_MOE_LAYERS,
        acceleration.cpu_moe_layers.as_deref(),
    );
}

fn append_arg_value(args: &mut Vec<String>, name: &str, value: Option<&str>) {
    if let Some(value) = value {
        args.push(name.to_string());
        args.push(value.to_string());
    }
}
