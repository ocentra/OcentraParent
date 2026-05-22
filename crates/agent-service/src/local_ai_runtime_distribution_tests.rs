use std::path::PathBuf;

use ocentra_parent_agent_protocol::constants;

use crate::local_ai_runtime_acceleration_config::LocalAiRuntimeAccelerationConfig;
use crate::local_ai_runtime_distribution::{
    requested_runtime_acceleration, select_llama_runtime_distribution, LlamaRuntimeAcceleration,
    LocalAiRuntimeTarget,
};

#[test]
fn current_runtime_target_matches_compiled_platform() {
    let target = LocalAiRuntimeTarget::current();

    assert_eq!(target.os, std::env::consts::OS);
    assert_eq!(target.arch, std::env::consts::ARCH);
}

#[test]
fn windows_x64_gpu_request_selects_vulkan_asset() {
    let target = LocalAiRuntimeTarget {
        os: constants::local_ai_runtime::PLATFORM_OS_WINDOWS,
        arch: constants::local_ai_runtime::PLATFORM_ARCH_X86_64,
    };
    let acceleration = requested_runtime_acceleration(
        target,
        &LocalAiRuntimeAccelerationConfig::basic(
            None,
            Some(constants::local_ai_runtime::LLAMA_GPU_LAYERS_ALL.to_string()),
        ),
    );
    let distribution = select_llama_runtime_distribution(
        target,
        acceleration,
        constants::local_ai_runtime::DEFAULT_LLAMA_CPP_RELEASE_TAG,
    )
    .expect(constants::error::LOCAL_AI_RUNTIME_SPAWNS);

    assert_eq!(acceleration, LlamaRuntimeAcceleration::Vulkan);
    assert_eq!(
        distribution.asset_name,
        expected_asset_name(constants::local_ai_runtime::LLAMA_ASSET_WIN_VULKAN_X64_SUFFIX)
    );
    assert_eq!(
        distribution.download_url,
        expected_download_url(constants::local_ai_runtime::LLAMA_ASSET_WIN_VULKAN_X64_SUFFIX)
    );
    assert_eq!(
        distribution.executable_name,
        constants::local_ai_runtime::LLAMA_CLI_EXECUTABLE_WINDOWS
    );
}

#[test]
fn windows_x64_cuda_device_selects_cuda_asset() {
    let target = LocalAiRuntimeTarget {
        os: constants::local_ai_runtime::PLATFORM_OS_WINDOWS,
        arch: constants::local_ai_runtime::PLATFORM_ARCH_X86_64,
    };
    let acceleration = requested_runtime_acceleration(
        target,
        &LocalAiRuntimeAccelerationConfig::basic(
            Some(constants::local_ai_runtime::TEST_RUNTIME_DEVICE_CUDA0.to_string()),
            None,
        ),
    );
    let distribution = select_llama_runtime_distribution(
        target,
        acceleration,
        constants::local_ai_runtime::DEFAULT_LLAMA_CPP_RELEASE_TAG,
    )
    .expect(constants::error::LOCAL_AI_RUNTIME_SPAWNS);

    assert_eq!(acceleration, LlamaRuntimeAcceleration::Cuda);
    assert_eq!(
        distribution.asset_name,
        expected_asset_name(constants::local_ai_runtime::LLAMA_ASSET_WIN_CUDA_12_4_X64_SUFFIX)
    );
}

#[test]
fn windows_x64_tensor_split_request_selects_vulkan_asset() {
    let target = LocalAiRuntimeTarget {
        os: constants::local_ai_runtime::PLATFORM_OS_WINDOWS,
        arch: constants::local_ai_runtime::PLATFORM_ARCH_X86_64,
    };
    let acceleration = requested_runtime_acceleration(
        target,
        &LocalAiRuntimeAccelerationConfig {
            split_mode: Some(constants::local_ai_runtime::LLAMA_SPLIT_MODE_LAYER.to_string()),
            tensor_split: Some(constants::local_ai_runtime::TEST_TENSOR_SPLIT_DUAL.to_string()),
            main_gpu: Some(constants::local_ai_runtime::TEST_MAIN_GPU_1.to_string()),
            ..LocalAiRuntimeAccelerationConfig::default()
        },
    );
    let distribution = select_llama_runtime_distribution(
        target,
        acceleration,
        constants::local_ai_runtime::DEFAULT_LLAMA_CPP_RELEASE_TAG,
    )
    .expect(constants::error::LOCAL_AI_RUNTIME_SPAWNS);

    assert_eq!(acceleration, LlamaRuntimeAcceleration::Vulkan);
    assert_eq!(
        distribution.asset_name,
        expected_asset_name(constants::local_ai_runtime::LLAMA_ASSET_WIN_VULKAN_X64_SUFFIX)
    );
}

#[test]
fn linux_x64_cpu_request_selects_ubuntu_cpu_asset() {
    let target = LocalAiRuntimeTarget {
        os: constants::local_ai_runtime::PLATFORM_OS_LINUX,
        arch: constants::local_ai_runtime::PLATFORM_ARCH_X86_64,
    };
    let distribution = select_llama_runtime_distribution(
        target,
        LlamaRuntimeAcceleration::Cpu,
        constants::local_ai_runtime::DEFAULT_LLAMA_CPP_RELEASE_TAG,
    )
    .expect(constants::error::LOCAL_AI_RUNTIME_SPAWNS);

    assert_eq!(
        distribution.asset_name,
        expected_asset_name(constants::local_ai_runtime::LLAMA_ASSET_UBUNTU_X64_SUFFIX)
    );
    assert_eq!(
        distribution.executable_name,
        constants::local_ai_runtime::LLAMA_CLI_EXECUTABLE_UNIX
    );
}

#[test]
fn macos_arm64_request_selects_macos_asset() {
    let target = LocalAiRuntimeTarget {
        os: constants::local_ai_runtime::PLATFORM_OS_MACOS,
        arch: constants::local_ai_runtime::PLATFORM_ARCH_AARCH64,
    };
    let distribution = select_llama_runtime_distribution(
        target,
        LlamaRuntimeAcceleration::Cpu,
        constants::local_ai_runtime::DEFAULT_LLAMA_CPP_RELEASE_TAG,
    )
    .expect(constants::error::LOCAL_AI_RUNTIME_SPAWNS);

    assert_eq!(
        distribution.asset_name,
        expected_asset_name(constants::local_ai_runtime::LLAMA_ASSET_MACOS_ARM64_SUFFIX)
    );
}

#[test]
fn extracted_runtime_path_uses_release_cache_directory() {
    let target = LocalAiRuntimeTarget {
        os: constants::local_ai_runtime::PLATFORM_OS_WINDOWS,
        arch: constants::local_ai_runtime::PLATFORM_ARCH_X86_64,
    };
    let distribution = select_llama_runtime_distribution(
        target,
        LlamaRuntimeAcceleration::Vulkan,
        constants::local_ai_runtime::DEFAULT_LLAMA_CPP_RELEASE_TAG,
    )
    .expect(constants::error::LOCAL_AI_RUNTIME_SPAWNS);
    let mut cache_root = PathBuf::new();
    cache_root.push(constants::local_ai_runtime::OCENTRA_PARENT_CACHE_DIR);

    let mut expected = cache_root.clone();
    expected.push(constants::local_ai_runtime::LLAMA_CPP_CACHE_DIR);
    expected.push(constants::local_ai_runtime::DEFAULT_LLAMA_CPP_RELEASE_TAG);
    expected.push(constants::local_ai_runtime::LLAMA_CLI_EXECUTABLE_WINDOWS);

    assert_eq!(distribution.extracted_binary_path(&cache_root), expected);
}

#[test]
fn runtime_archive_path_uses_asset_name_under_runtime_cache() {
    let target = LocalAiRuntimeTarget {
        os: constants::local_ai_runtime::PLATFORM_OS_WINDOWS,
        arch: constants::local_ai_runtime::PLATFORM_ARCH_X86_64,
    };
    let distribution = select_llama_runtime_distribution(
        target,
        LlamaRuntimeAcceleration::Vulkan,
        constants::local_ai_runtime::DEFAULT_LLAMA_CPP_RELEASE_TAG,
    )
    .expect(constants::error::LOCAL_AI_RUNTIME_SPAWNS);
    let mut cache_root = PathBuf::new();
    cache_root.push(constants::local_ai_runtime::OCENTRA_PARENT_CACHE_DIR);
    let mut expected = cache_root.clone();
    expected.push(constants::local_ai_runtime::LLAMA_CPP_CACHE_DIR);
    expected.push(&distribution.asset_name);

    assert_eq!(distribution.archive_path(&cache_root), expected);
}

fn expected_asset_name(suffix: &str) -> String {
    let mut name = constants::local_ai_runtime::LLAMA_ASSET_PREFIX.to_string();
    name.push_str(constants::local_ai_runtime::DEFAULT_LLAMA_CPP_RELEASE_TAG);
    name.push_str(suffix);
    name
}

fn expected_download_url(suffix: &str) -> String {
    let asset_name = expected_asset_name(suffix);
    let mut url = constants::local_ai_runtime::LLAMA_CPP_RELEASE_DOWNLOAD_BASE_URL.to_string();
    url.push(constants::delimiter::SLASH);
    url.push_str(constants::local_ai_runtime::DEFAULT_LLAMA_CPP_RELEASE_TAG);
    url.push(constants::delimiter::SLASH);
    url.push_str(&asset_name);
    url
}
