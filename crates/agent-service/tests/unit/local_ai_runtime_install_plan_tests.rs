use std::ffi::OsString as TestOsString;
use std::path::PathBuf as TestPathBuf;
use std::primitive::str as TestStr;
use std::{
    fs,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use ocentra_parent_agent_protocol::constants;

use crate::local_ai_runtime_acceleration_config::LocalAiRuntimeAccelerationConfig;
use crate::local_ai_runtime_distribution::LocalAiRuntimeTarget;
use crate::local_ai_runtime_install_plan::{
    default_install_plan_for_target, default_install_plan_from_environment,
    LocalAiRequiredArtifactStatus,
};
use crate::test_require_ok::require_ok;
use crate::test_require_some::require_some;

#[test]
fn install_plan_reports_missing_required_runtime_and_default_model() {
    let root = unique_cache_root();
    let target = windows_x64_target();
    let plan = default_install_plan_for_target(
        &root,
        target,
        constants::local_ai_runtime::DEFAULT_LLAMA_CPP_RELEASE_TAG,
        &gpu_acceleration_config(),
    );
    let runtime = require_some(
        plan.runtime.as_ref(),
        constants::error::LOCAL_AI_RUNTIME_SPAWNS,
    );

    assert_eq!(plan.runtime_status, LocalAiRequiredArtifactStatus::Missing);
    assert_eq!(
        plan.default_model.status,
        LocalAiRequiredArtifactStatus::Missing
    );
    assert!(runtime
        .archive_path
        .ends_with(&runtime.distribution.asset_name));
    assert!(runtime.binary_path.ends_with(runtime_binary_suffix()));
    assert!(plan
        .default_model
        .model_path
        .ends_with(default_model_suffix()));
    assert_eq!(runtime.status, LocalAiRequiredArtifactStatus::Missing);
}

#[test]
fn install_plan_reports_installed_after_cache_artifacts_exist() {
    let root = unique_cache_root();
    let target = windows_x64_target();
    let initial = default_install_plan_for_target(
        &root,
        target,
        constants::local_ai_runtime::DEFAULT_LLAMA_CPP_RELEASE_TAG,
        &gpu_acceleration_config(),
    );
    write_required_files(&initial);

    let installed = default_install_plan_for_target(
        &root,
        target,
        constants::local_ai_runtime::DEFAULT_LLAMA_CPP_RELEASE_TAG,
        &gpu_acceleration_config(),
    );

    assert_eq!(
        installed.runtime_status,
        LocalAiRequiredArtifactStatus::Installed
    );
    assert_eq!(
        installed.default_model.status,
        LocalAiRequiredArtifactStatus::Installed
    );
    assert_eq!(
        installed.runtime.as_ref().map(|runtime| runtime.status),
        Some(LocalAiRequiredArtifactStatus::Installed)
    );
    let _ = fs::remove_dir_all(root);
}

#[test]
fn unsupported_runtime_target_does_not_guess_binary() {
    let root = unique_cache_root();
    let target = LocalAiRuntimeTarget {
        os: constants::local_ai_runtime::PLATFORM_OS_ANDROID,
        arch: constants::local_ai_runtime::PLATFORM_ARCH_X86_64,
    };
    let plan = default_install_plan_for_target(
        &root,
        target,
        constants::local_ai_runtime::DEFAULT_LLAMA_CPP_RELEASE_TAG,
        &LocalAiRuntimeAccelerationConfig::default(),
    );

    assert_eq!(plan.runtime, None);
    assert_eq!(
        plan.runtime_status,
        LocalAiRequiredArtifactStatus::Unsupported
    );
    assert_eq!(
        plan.default_model.status,
        LocalAiRequiredArtifactStatus::Missing
    );
}

#[test]
fn install_plan_creates_managed_cache_directories() {
    let root = unique_cache_root();
    let plan = default_install_plan_for_target(
        &root,
        windows_x64_target(),
        constants::local_ai_runtime::DEFAULT_LLAMA_CPP_RELEASE_TAG,
        &gpu_acceleration_config(),
    );

    require_ok(
        plan.ensure_cache_directories(),
        constants::error::LOCAL_AI_RUNTIME_SPAWNS,
    );
    assert!(AsRef::<Path>::as_ref(&plan.runtime_cache_dir()).is_dir());
    assert!(AsRef::<Path>::as_ref(&plan.model_cache_dir()).is_dir());
    let _ = fs::remove_dir_all(root);
}

#[test]
fn install_plan_from_environment_uses_configured_cache_root() {
    let previous_root = std::env::var_os(constants::env_var::LOCAL_AI_RUNTIME_CACHE_DIR);
    let configured_root = unique_cache_root().join("configured");
    std::env::set_var(
        constants::env_var::LOCAL_AI_RUNTIME_CACHE_DIR,
        &configured_root,
    );

    let plan = default_install_plan_from_environment(
        constants::local_ai_runtime::DEFAULT_LLAMA_CPP_RELEASE_TAG,
        &gpu_acceleration_config(),
    );

    restore_env_var(
        constants::env_var::LOCAL_AI_RUNTIME_CACHE_DIR,
        previous_root,
    );
    assert_eq!(
        plan.as_ref().map(|plan| plan.cache_root.clone()),
        Some(configured_root)
    );
}

fn write_required_files(plan: &crate::local_ai_runtime_install_plan::LocalAiRuntimeInstallPlan) {
    let runtime = require_some(
        plan.runtime.as_ref(),
        constants::error::LOCAL_AI_RUNTIME_SPAWNS,
    );
    write_file(
        &runtime.binary_path,
        constants::local_ai_runtime::REQUIRED_ARTIFACT_RUNTIME,
    );
    write_file(
        &plan.default_model.model_path,
        constants::local_ai_runtime::REQUIRED_ARTIFACT_DEFAULT_MODEL,
    );
}

fn write_file(path: &Path, contents: &TestStr) {
    if let Some(parent) = path.parent() {
        require_ok(
            fs::create_dir_all(parent),
            constants::error::LOCAL_AI_RUNTIME_SPAWNS,
        );
    }
    require_ok(
        fs::write(path, contents),
        constants::error::LOCAL_AI_RUNTIME_SPAWNS,
    );
}

fn windows_x64_target() -> LocalAiRuntimeTarget {
    LocalAiRuntimeTarget {
        os: constants::local_ai_runtime::PLATFORM_OS_WINDOWS,
        arch: constants::local_ai_runtime::PLATFORM_ARCH_X86_64,
    }
}

fn gpu_acceleration_config() -> LocalAiRuntimeAccelerationConfig {
    LocalAiRuntimeAccelerationConfig {
        gpu_layers: Some(constants::local_ai_runtime::LLAMA_GPU_LAYERS_ALL.to_string()),
        ..LocalAiRuntimeAccelerationConfig::default()
    }
}

fn runtime_binary_suffix() -> TestPathBuf {
    Path::new(constants::local_ai_runtime::DEFAULT_LLAMA_CPP_RELEASE_TAG)
        .join(constants::local_ai_runtime::LLAMA_CLI_EXECUTABLE_WINDOWS)
}

fn default_model_suffix() -> TestPathBuf {
    Path::new(constants::local_ai_runtime::LOCAL_AI_MODELS_CACHE_DIR)
        .join(constants::local_ai_runtime::DEFAULT_GEMMA_4_MODEL_FILE_NAME)
}

fn unique_cache_root() -> TestPathBuf {
    let mut path = std::env::temp_dir();
    path.push(constants::local_ai_runtime::OCENTRA_PARENT_CACHE_DIR);
    path.push(std::process::id().to_string());
    path.push(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
            .to_string(),
    );
    path
}

fn restore_env_var(env_var_name: &TestStr, value: Option<TestOsString>) {
    match value {
        Some(previous) => std::env::set_var(env_var_name, previous),
        None => std::env::remove_var(env_var_name),
    }
}
