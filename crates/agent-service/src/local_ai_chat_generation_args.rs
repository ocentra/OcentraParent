use ocentra_parent_agent_protocol::constants;

use crate::{
    local_ai_runtime_config::LocalAiRuntimeConfigSnapshot,
    local_ai_runtime_config_values::LocalAiRuntimeText,
};

#[derive(Clone, Debug)]
pub(crate) struct LocalAiCommandArgs(Vec<LocalAiRuntimeText>);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct LocalAiArgFlag(&'static str);

impl LocalAiCommandArgs {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn push(&mut self, value: impl Into<LocalAiRuntimeText>) {
        self.0.push(value.into());
    }
}

impl IntoIterator for LocalAiCommandArgs {
    type Item = LocalAiRuntimeText;
    type IntoIter = std::vec::IntoIter<LocalAiRuntimeText>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub(crate) fn llama_acceleration_args(config: &LocalAiRuntimeConfigSnapshot) -> LocalAiCommandArgs {
    let acceleration = config.acceleration();
    let mut args = LocalAiCommandArgs::new();
    append_arg_value(
        &mut args,
        LocalAiArgFlag(constants::local_ai_runtime::LLAMA_ARG_DEVICE),
        acceleration
            .runtime_device
            .as_deref()
            .map(LocalAiRuntimeText::from),
    );
    append_arg_value(
        &mut args,
        LocalAiArgFlag(constants::local_ai_runtime::LLAMA_ARG_GPU_LAYERS),
        acceleration
            .gpu_layers
            .as_deref()
            .map(LocalAiRuntimeText::from),
    );
    append_arg_value(
        &mut args,
        LocalAiArgFlag(constants::local_ai_runtime::LLAMA_ARG_SPLIT_MODE),
        acceleration
            .split_mode
            .as_deref()
            .map(LocalAiRuntimeText::from),
    );
    append_arg_value(
        &mut args,
        LocalAiArgFlag(constants::local_ai_runtime::LLAMA_ARG_TENSOR_SPLIT),
        acceleration
            .tensor_split
            .as_deref()
            .map(LocalAiRuntimeText::from),
    );
    append_arg_value(
        &mut args,
        LocalAiArgFlag(constants::local_ai_runtime::LLAMA_ARG_MAIN_GPU),
        acceleration
            .main_gpu
            .as_deref()
            .map(LocalAiRuntimeText::from),
    );
    append_arg_value(
        &mut args,
        LocalAiArgFlag(constants::local_ai_runtime::LLAMA_ARG_FIT),
        acceleration.fit.as_deref().map(LocalAiRuntimeText::from),
    );
    append_arg_value(
        &mut args,
        LocalAiArgFlag(constants::local_ai_runtime::LLAMA_ARG_FIT_TARGET),
        acceleration
            .fit_target
            .as_deref()
            .map(LocalAiRuntimeText::from),
    );
    append_op_offload_arg(&mut args, acceleration.op_offload);
    append_cpu_moe_args(&mut args, config);
    args
}

fn append_op_offload_arg(args: &mut LocalAiCommandArgs, op_offload: Option<bool>) {
    if let Some(op_offload) = op_offload {
        args.push(if op_offload {
            constants::local_ai_runtime::LLAMA_ARG_OP_OFFLOAD
        } else {
            constants::local_ai_runtime::LLAMA_ARG_NO_OP_OFFLOAD
        });
    }
}

fn append_cpu_moe_args(args: &mut LocalAiCommandArgs, config: &LocalAiRuntimeConfigSnapshot) {
    let acceleration = config.acceleration();
    if acceleration.cpu_moe {
        args.push(constants::local_ai_runtime::LLAMA_ARG_CPU_MOE);
    }
    append_arg_value(
        args,
        LocalAiArgFlag(constants::local_ai_runtime::LLAMA_ARG_CPU_MOE_LAYERS),
        acceleration
            .cpu_moe_layers
            .as_deref()
            .map(LocalAiRuntimeText::from),
    );
}

fn append_arg_value(
    args: &mut LocalAiCommandArgs,
    arg_flag: LocalAiArgFlag,
    value: Option<LocalAiRuntimeText>,
) {
    if let Some(value) = value {
        args.push(arg_flag.0);
        args.push(value);
    }
}
