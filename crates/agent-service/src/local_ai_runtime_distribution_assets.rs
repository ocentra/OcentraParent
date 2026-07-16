use ocentra_parent_agent_protocol::constants;

use crate::local_ai_runtime_config_values::LocalAiRuntimeText;
use crate::local_ai_runtime_distribution::{LlamaRuntimeAcceleration, LocalAiRuntimeTarget};

#[derive(Clone, Copy, Debug)]
struct AssetSuffixMapping {
    os: &'static str,
    arch: &'static str,
    acceleration: Option<LlamaRuntimeAcceleration>,
    suffix: &'static str,
}

impl AssetSuffixMapping {
    fn matches(
        &self,
        target: LocalAiRuntimeTarget,
        acceleration: LlamaRuntimeAcceleration,
    ) -> bool {
        self.os == target.os
            && self.arch == target.arch
            && match self.acceleration {
                Some(candidate) => candidate == acceleration,
                None => true,
            }
    }

    fn suffix_text(&self) -> LocalAiRuntimeText {
        LocalAiRuntimeText(self.suffix.to_string())
    }
}

const ASSET_SUFFIX_MAPPINGS: &[AssetSuffixMapping] = &[
    AssetSuffixMapping {
        os: constants::local_ai_runtime::PLATFORM_OS_WINDOWS,
        arch: constants::local_ai_runtime::PLATFORM_ARCH_X86_64,
        acceleration: Some(LlamaRuntimeAcceleration::Vulkan),
        suffix: constants::local_ai_runtime::LLAMA_ASSET_WIN_VULKAN_X64_SUFFIX,
    },
    AssetSuffixMapping {
        os: constants::local_ai_runtime::PLATFORM_OS_WINDOWS,
        arch: constants::local_ai_runtime::PLATFORM_ARCH_X86_64,
        acceleration: Some(LlamaRuntimeAcceleration::Cuda),
        suffix: constants::local_ai_runtime::LLAMA_ASSET_WIN_CUDA_12_4_X64_SUFFIX,
    },
    AssetSuffixMapping {
        os: constants::local_ai_runtime::PLATFORM_OS_WINDOWS,
        arch: constants::local_ai_runtime::PLATFORM_ARCH_X86_64,
        acceleration: Some(LlamaRuntimeAcceleration::Cpu),
        suffix: constants::local_ai_runtime::LLAMA_ASSET_WIN_CPU_X64_SUFFIX,
    },
    AssetSuffixMapping {
        os: constants::local_ai_runtime::PLATFORM_OS_WINDOWS,
        arch: constants::local_ai_runtime::PLATFORM_ARCH_AARCH64,
        acceleration: None,
        suffix: constants::local_ai_runtime::LLAMA_ASSET_WIN_CPU_ARM64_SUFFIX,
    },
    AssetSuffixMapping {
        os: constants::local_ai_runtime::PLATFORM_OS_MACOS,
        arch: constants::local_ai_runtime::PLATFORM_ARCH_AARCH64,
        acceleration: None,
        suffix: constants::local_ai_runtime::LLAMA_ASSET_MACOS_ARM64_SUFFIX,
    },
    AssetSuffixMapping {
        os: constants::local_ai_runtime::PLATFORM_OS_MACOS,
        arch: constants::local_ai_runtime::PLATFORM_ARCH_X86_64,
        acceleration: None,
        suffix: constants::local_ai_runtime::LLAMA_ASSET_MACOS_X64_SUFFIX,
    },
    AssetSuffixMapping {
        os: constants::local_ai_runtime::PLATFORM_OS_LINUX,
        arch: constants::local_ai_runtime::PLATFORM_ARCH_X86_64,
        acceleration: Some(LlamaRuntimeAcceleration::Vulkan),
        suffix: constants::local_ai_runtime::LLAMA_ASSET_UBUNTU_VULKAN_X64_SUFFIX,
    },
    AssetSuffixMapping {
        os: constants::local_ai_runtime::PLATFORM_OS_LINUX,
        arch: constants::local_ai_runtime::PLATFORM_ARCH_AARCH64,
        acceleration: Some(LlamaRuntimeAcceleration::Vulkan),
        suffix: constants::local_ai_runtime::LLAMA_ASSET_UBUNTU_VULKAN_ARM64_SUFFIX,
    },
    AssetSuffixMapping {
        os: constants::local_ai_runtime::PLATFORM_OS_LINUX,
        arch: constants::local_ai_runtime::PLATFORM_ARCH_X86_64,
        acceleration: None,
        suffix: constants::local_ai_runtime::LLAMA_ASSET_UBUNTU_X64_SUFFIX,
    },
    AssetSuffixMapping {
        os: constants::local_ai_runtime::PLATFORM_OS_LINUX,
        arch: constants::local_ai_runtime::PLATFORM_ARCH_AARCH64,
        acceleration: None,
        suffix: constants::local_ai_runtime::LLAMA_ASSET_UBUNTU_ARM64_SUFFIX,
    },
    AssetSuffixMapping {
        os: constants::local_ai_runtime::PLATFORM_OS_ANDROID,
        arch: constants::local_ai_runtime::PLATFORM_ARCH_AARCH64,
        acceleration: None,
        suffix: constants::local_ai_runtime::LLAMA_ASSET_ANDROID_ARM64_SUFFIX,
    },
];

pub(crate) fn asset_suffix(
    target: LocalAiRuntimeTarget,
    acceleration: LlamaRuntimeAcceleration,
) -> Option<LocalAiRuntimeText> {
    ASSET_SUFFIX_MAPPINGS
        .iter()
        .find(|mapping| mapping.matches(target, acceleration))
        .map(AssetSuffixMapping::suffix_text)
}

pub(crate) fn executable_name(target: LocalAiRuntimeTarget) -> LocalAiRuntimeText {
    if target.os == constants::local_ai_runtime::PLATFORM_OS_WINDOWS {
        LocalAiRuntimeText(constants::local_ai_runtime::LLAMA_CLI_EXECUTABLE_WINDOWS.to_string())
    } else {
        LocalAiRuntimeText(constants::local_ai_runtime::LLAMA_CLI_EXECUTABLE_UNIX.to_string())
    }
}

pub(crate) fn asset_name(
    release_tag: &LocalAiRuntimeText,
    suffix: &LocalAiRuntimeText,
) -> LocalAiRuntimeText {
    let mut name = constants::local_ai_runtime::LLAMA_ASSET_PREFIX.to_string();
    name.push_str(&release_tag.0);
    name.push_str(&suffix.0);
    LocalAiRuntimeText(name)
}

pub(crate) fn download_url(
    release_tag: &LocalAiRuntimeText,
    asset_name: &LocalAiRuntimeText,
) -> LocalAiRuntimeText {
    let mut url = constants::local_ai_runtime::LLAMA_CPP_RELEASE_DOWNLOAD_BASE_URL.to_string();
    url.push(constants::delimiter::SLASH);
    url.push_str(&release_tag.0);
    url.push(constants::delimiter::SLASH);
    url.push_str(&asset_name.0);
    LocalAiRuntimeText(url)
}
