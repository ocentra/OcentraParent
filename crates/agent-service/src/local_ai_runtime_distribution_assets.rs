use ocentra_parent_agent_protocol::constants;

use crate::local_ai_runtime_distribution::{LlamaRuntimeAcceleration, LocalAiRuntimeTarget};

pub(crate) fn asset_suffix(
    target: LocalAiRuntimeTarget,
    acceleration: LlamaRuntimeAcceleration,
) -> Option<&'static str> {
    match (target.os, target.arch, acceleration) {
        (
            constants::local_ai_runtime::PLATFORM_OS_WINDOWS,
            constants::local_ai_runtime::PLATFORM_ARCH_X86_64,
            LlamaRuntimeAcceleration::Vulkan,
        ) => Some(constants::local_ai_runtime::LLAMA_ASSET_WIN_VULKAN_X64_SUFFIX),
        (
            constants::local_ai_runtime::PLATFORM_OS_WINDOWS,
            constants::local_ai_runtime::PLATFORM_ARCH_X86_64,
            LlamaRuntimeAcceleration::Cuda,
        ) => Some(constants::local_ai_runtime::LLAMA_ASSET_WIN_CUDA_12_4_X64_SUFFIX),
        (
            constants::local_ai_runtime::PLATFORM_OS_WINDOWS,
            constants::local_ai_runtime::PLATFORM_ARCH_X86_64,
            LlamaRuntimeAcceleration::Cpu,
        ) => Some(constants::local_ai_runtime::LLAMA_ASSET_WIN_CPU_X64_SUFFIX),
        (
            constants::local_ai_runtime::PLATFORM_OS_WINDOWS,
            constants::local_ai_runtime::PLATFORM_ARCH_AARCH64,
            _,
        ) => Some(constants::local_ai_runtime::LLAMA_ASSET_WIN_CPU_ARM64_SUFFIX),
        (
            constants::local_ai_runtime::PLATFORM_OS_MACOS,
            constants::local_ai_runtime::PLATFORM_ARCH_AARCH64,
            _,
        ) => Some(constants::local_ai_runtime::LLAMA_ASSET_MACOS_ARM64_SUFFIX),
        (
            constants::local_ai_runtime::PLATFORM_OS_MACOS,
            constants::local_ai_runtime::PLATFORM_ARCH_X86_64,
            _,
        ) => Some(constants::local_ai_runtime::LLAMA_ASSET_MACOS_X64_SUFFIX),
        (
            constants::local_ai_runtime::PLATFORM_OS_LINUX,
            constants::local_ai_runtime::PLATFORM_ARCH_X86_64,
            LlamaRuntimeAcceleration::Vulkan,
        ) => Some(constants::local_ai_runtime::LLAMA_ASSET_UBUNTU_VULKAN_X64_SUFFIX),
        (
            constants::local_ai_runtime::PLATFORM_OS_LINUX,
            constants::local_ai_runtime::PLATFORM_ARCH_AARCH64,
            LlamaRuntimeAcceleration::Vulkan,
        ) => Some(constants::local_ai_runtime::LLAMA_ASSET_UBUNTU_VULKAN_ARM64_SUFFIX),
        (
            constants::local_ai_runtime::PLATFORM_OS_LINUX,
            constants::local_ai_runtime::PLATFORM_ARCH_X86_64,
            _,
        ) => Some(constants::local_ai_runtime::LLAMA_ASSET_UBUNTU_X64_SUFFIX),
        (
            constants::local_ai_runtime::PLATFORM_OS_LINUX,
            constants::local_ai_runtime::PLATFORM_ARCH_AARCH64,
            _,
        ) => Some(constants::local_ai_runtime::LLAMA_ASSET_UBUNTU_ARM64_SUFFIX),
        (
            constants::local_ai_runtime::PLATFORM_OS_ANDROID,
            constants::local_ai_runtime::PLATFORM_ARCH_AARCH64,
            _,
        ) => Some(constants::local_ai_runtime::LLAMA_ASSET_ANDROID_ARM64_SUFFIX),
        _ => None,
    }
}

pub(crate) fn executable_name(target: LocalAiRuntimeTarget) -> &'static str {
    if target.os == constants::local_ai_runtime::PLATFORM_OS_WINDOWS {
        constants::local_ai_runtime::LLAMA_CLI_EXECUTABLE_WINDOWS
    } else {
        constants::local_ai_runtime::LLAMA_CLI_EXECUTABLE_UNIX
    }
}

pub(crate) fn asset_name(release_tag: &str, suffix: &str) -> String {
    let mut name = constants::local_ai_runtime::LLAMA_ASSET_PREFIX.to_string();
    name.push_str(release_tag);
    name.push_str(suffix);
    name
}

pub(crate) fn download_url(release_tag: &str, asset_name: &str) -> String {
    let mut url = constants::local_ai_runtime::LLAMA_CPP_RELEASE_DOWNLOAD_BASE_URL.to_string();
    url.push(constants::delimiter::SLASH);
    url.push_str(release_tag);
    url.push(constants::delimiter::SLASH);
    url.push_str(asset_name);
    url
}
