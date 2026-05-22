use std::path::{Path, PathBuf};

use ocentra_parent_agent_protocol::constants;

use crate::{
    local_ai_cache_root::local_ai_cache_root,
    local_ai_runtime_distribution_assets::{
        asset_name, asset_suffix, download_url, executable_name,
    },
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
    pub(crate) fn extracted_binary_path(&self, cache_root: &Path) -> PathBuf {
        let mut path = cache_root.to_path_buf();
        path.push(constants::local_ai_runtime::LLAMA_CPP_CACHE_DIR);
        path.push(&self.release_tag);
        path.push(self.executable_name);
        path
    }
}

pub(crate) fn selected_cached_llama_runtime_path(
    release_tag: &str,
    runtime_device: Option<&str>,
    gpu_layers: Option<&str>,
) -> Option<PathBuf> {
    let cache_root = local_ai_cache_root()?;
    let target = LocalAiRuntimeTarget::current();
    select_llama_runtime_distribution(
        target,
        requested_runtime_acceleration(target, runtime_device, gpu_layers),
        release_tag,
    )
    .map(|distribution| distribution.extracted_binary_path(&cache_root))
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
    runtime_device: Option<&str>,
    gpu_layers: Option<&str>,
) -> LlamaRuntimeAcceleration {
    if runtime_device
        .map(|value| value.starts_with(constants::local_ai_runtime::LLAMA_DEVICE_CUDA_PREFIX))
        .unwrap_or(false)
    {
        return LlamaRuntimeAcceleration::Cuda;
    }

    if runtime_device
        .map(|value| value.starts_with(constants::local_ai_runtime::LLAMA_DEVICE_VULKAN_PREFIX))
        .unwrap_or(false)
        || gpu_layers
            .map(gpu_layers_request_acceleration)
            .unwrap_or(false)
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

fn gpu_layers_request_acceleration(value: &str) -> bool {
    value == constants::local_ai_runtime::LLAMA_GPU_LAYERS_ALL
        || value == constants::local_ai_runtime::LLAMA_GPU_LAYERS_AUTO
        || value
            .parse::<u32>()
            .map(|layers| layers > 0)
            .unwrap_or(false)
}
