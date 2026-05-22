use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use ocentra_parent_agent_protocol::constants;

use crate::local_ai_runtime_distribution::LocalAiRuntimeTarget;
use crate::local_ai_runtime_install_plan::{
    default_install_plan_for_target, LocalAiRequiredArtifactStatus,
};

#[test]
fn install_plan_reports_missing_required_runtime_and_default_model() {
    let root = unique_cache_root();
    let target = windows_x64_target();
    let plan = default_install_plan_for_target(
        &root,
        target,
        constants::local_ai_runtime::DEFAULT_LLAMA_CPP_RELEASE_TAG,
        None,
        Some(constants::local_ai_runtime::LLAMA_GPU_LAYERS_ALL),
    );
    let runtime = plan
        .runtime
        .as_ref()
        .expect(constants::error::LOCAL_AI_RUNTIME_SPAWNS);

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
        None,
        Some(constants::local_ai_runtime::LLAMA_GPU_LAYERS_ALL),
    );
    write_required_files(&initial);

    let installed = default_install_plan_for_target(
        &root,
        target,
        constants::local_ai_runtime::DEFAULT_LLAMA_CPP_RELEASE_TAG,
        None,
        Some(constants::local_ai_runtime::LLAMA_GPU_LAYERS_ALL),
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
        None,
        None,
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
        None,
        Some(constants::local_ai_runtime::LLAMA_GPU_LAYERS_ALL),
    );

    assert!(plan.ensure_cache_directories().is_ok());
    assert!(plan.runtime_cache_dir().is_dir());
    assert!(plan.model_cache_dir().is_dir());
    let _ = fs::remove_dir_all(root);
}

fn write_required_files(plan: &crate::local_ai_runtime_install_plan::LocalAiRuntimeInstallPlan) {
    let runtime = plan
        .runtime
        .as_ref()
        .expect(constants::error::LOCAL_AI_RUNTIME_SPAWNS);
    write_file(
        &runtime.binary_path,
        constants::local_ai_runtime::REQUIRED_ARTIFACT_RUNTIME,
    );
    write_file(
        &plan.default_model.model_path,
        constants::local_ai_runtime::REQUIRED_ARTIFACT_DEFAULT_MODEL,
    );
}

fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        assert!(fs::create_dir_all(parent).is_ok());
    }
    assert!(fs::write(path, contents).is_ok());
}

fn windows_x64_target() -> LocalAiRuntimeTarget {
    LocalAiRuntimeTarget {
        os: constants::local_ai_runtime::PLATFORM_OS_WINDOWS,
        arch: constants::local_ai_runtime::PLATFORM_ARCH_X86_64,
    }
}

fn runtime_binary_suffix() -> PathBuf {
    Path::new(constants::local_ai_runtime::DEFAULT_LLAMA_CPP_RELEASE_TAG)
        .join(constants::local_ai_runtime::LLAMA_CLI_EXECUTABLE_WINDOWS)
}

fn default_model_suffix() -> PathBuf {
    Path::new(constants::local_ai_runtime::LOCAL_AI_MODELS_CACHE_DIR)
        .join(constants::local_ai_runtime::DEFAULT_GEMMA_4_MODEL_FILE_NAME)
}

fn unique_cache_root() -> PathBuf {
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
