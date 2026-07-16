use ocentra_parent_agent_protocol::constants;

use crate::local_ai_runtime_config_values::{env_path, LocalAiRuntimeEnvVar, LocalAiRuntimePath};

pub(crate) fn local_ai_cache_root() -> Option<LocalAiRuntimePath> {
    env_path(LocalAiRuntimeEnvVar(
        constants::env_var::LOCAL_AI_RUNTIME_CACHE_DIR,
    ))
    .or_else(|| {
        let mut path = env_path(LocalAiRuntimeEnvVar(constants::env_var::HOME))
            .or_else(|| env_path(LocalAiRuntimeEnvVar(constants::env_var::USERPROFILE)))?
            .0;
        path.push(constants::local_ai_runtime::USER_CACHE_DIR);
        path.push(constants::local_ai_runtime::OCENTRA_PARENT_CACHE_DIR);
        Some(LocalAiRuntimePath(path))
    })
}
