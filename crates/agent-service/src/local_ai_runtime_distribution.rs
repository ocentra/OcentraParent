use std::path::{Path, PathBuf};

use ocentra_parent_agent_protocol::constants;

use crate::local_ai_runtime_acceleration_config::LocalAiRuntimeAccelerationConfig;
use crate::local_ai_runtime_distribution_assets::{
    asset_name, asset_suffix, download_url, executable_name,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum LlamaRuntimeAcceleration {
    Cpu,
    Cuda,
    Vulkan,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct LocalAiRuntimeTarget {
    pub(crate) os: &'static str,
    pub(crate) arch: &'static str,
}

impl LocalAiRuntimeTarget {
    pub(crate) fn current() -> Self {
        Self {
            os: std::env::consts::OS,
            arch: std::env::consts::ARCH,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LlamaRuntimeDistribution {
    pub(crate) target: LocalAiRuntimeTarget,
    pub(crate) acceleration: LlamaRuntimeAcceleration,
    pub(crate) release_tag: String,
    pub(crate) asset_name: String,
    pub(crate) download_url: String,
    pub(crate) executable_name: &'static str,
}

impl LlamaRuntimeDistribution {
    pub(crate) fn archive_path(&self, cache_root: &Path) -> PathBuf {
        let mut path = cache_root.to_path_buf();
        path.push(constants::local_ai_runtime::LLAMA_CPP_CACHE_DIR);
        path.push(&self.asset_name);
        path
    }

    pub(crate) fn extracted_binary_path(&self, cache_root: &Path) -> PathBuf {
        let mut path = cache_root.to_path_buf();
        path.push(constants::local_ai_runtime::LLAMA_CPP_CACHE_DIR);
        path.push(&self.release_tag);
        path.push(self.executable_name);
        path
    }
}

pub(crate) fn select_llama_runtime_distribution(
    target: LocalAiRuntimeTarget,
    acceleration: LlamaRuntimeAcceleration,
    release_tag: &str,
) -> Option<LlamaRuntimeDistribution> {
    let suffix = asset_suffix(target, acceleration)?;
    Some(LlamaRuntimeDistribution {
        target,
        acceleration,
        release_tag: release_tag.to_string(),
        asset_name: asset_name(release_tag, suffix),
        download_url: download_url(release_tag, &asset_name(release_tag, suffix)),
        executable_name: executable_name(target),
    })
}

pub(crate) fn requested_runtime_acceleration(
    target: LocalAiRuntimeTarget,
    acceleration_config: &LocalAiRuntimeAccelerationConfig,
) -> LlamaRuntimeAcceleration {
    if acceleration_config
        .runtime_device
        .as_deref()
        .map(|value| value.starts_with(constants::local_ai_runtime::LLAMA_DEVICE_CUDA_PREFIX))
        .unwrap_or(false)
    {
        return LlamaRuntimeAcceleration::Cuda;
    }

    if acceleration_config
        .runtime_device
        .as_deref()
        .map(|value| value.starts_with(constants::local_ai_runtime::LLAMA_DEVICE_VULKAN_PREFIX))
        .unwrap_or(false)
        || acceleration_config.uses_gpu_runtime()
    {
        return preferred_gpu_acceleration(target);
    }

    LlamaRuntimeAcceleration::Cpu
}

fn preferred_gpu_acceleration(target: LocalAiRuntimeTarget) -> LlamaRuntimeAcceleration {
    if target.os == constants::local_ai_runtime::PLATFORM_OS_WINDOWS
        || target.os == constants::local_ai_runtime::PLATFORM_OS_LINUX
    {
        LlamaRuntimeAcceleration::Vulkan
    } else {
        LlamaRuntimeAcceleration::Cpu
    }
}
