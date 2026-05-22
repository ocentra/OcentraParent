use std::{
    fs, io,
    path::{Path, PathBuf},
};

use ocentra_parent_agent_protocol::constants;

use crate::{
    local_ai_cache_root::local_ai_cache_root,
    local_ai_model_registry::{default_local_ai_model, LocalAiKnownModel},
    local_ai_runtime_acceleration_config::LocalAiRuntimeAccelerationConfig,
    local_ai_runtime_distribution::{
        requested_runtime_acceleration, select_llama_runtime_distribution,
        LlamaRuntimeDistribution, LocalAiRuntimeTarget,
    },
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum LocalAiRequiredArtifactStatus {
    Installed,
    Missing,
    Unsupported,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LocalAiRuntimeInstallRequirement {
    pub(crate) distribution: LlamaRuntimeDistribution,
    pub(crate) archive_path: PathBuf,
    pub(crate) binary_path: PathBuf,
    pub(crate) status: LocalAiRequiredArtifactStatus,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LocalAiModelInstallRequirement {
    pub(crate) model: LocalAiKnownModel,
    pub(crate) model_path: PathBuf,
    pub(crate) status: LocalAiRequiredArtifactStatus,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LocalAiRuntimeInstallPlan {
    pub(crate) cache_root: PathBuf,
    pub(crate) runtime_status: LocalAiRequiredArtifactStatus,
    pub(crate) runtime: Option<LocalAiRuntimeInstallRequirement>,
    pub(crate) default_model: LocalAiModelInstallRequirement,
}

impl LocalAiRuntimeInstallPlan {
    pub(crate) fn ensure_cache_directories(&self) -> io::Result<()> {
        fs::create_dir_all(self.runtime_cache_dir())?;
        fs::create_dir_all(self.model_cache_dir())
    }

    pub(crate) fn runtime_cache_dir(&self) -> PathBuf {
        let mut path = self.cache_root.clone();
        path.push(constants::local_ai_runtime::LLAMA_CPP_CACHE_DIR);
        path
    }

    pub(crate) fn model_cache_dir(&self) -> PathBuf {
        let mut path = self.cache_root.clone();
        path.push(constants::local_ai_runtime::LOCAL_AI_MODELS_CACHE_DIR);
        path
    }
}

pub(crate) fn default_install_plan_from_environment(
    release_tag: &str,
    acceleration_config: &LocalAiRuntimeAccelerationConfig,
) -> Option<LocalAiRuntimeInstallPlan> {
    let cache_root = local_ai_cache_root()?;
    Some(default_install_plan_for_target(
        &cache_root,
        LocalAiRuntimeTarget::current(),
        release_tag,
        acceleration_config,
    ))
}

pub(crate) fn default_install_plan_for_target(
    cache_root: &Path,
    target: LocalAiRuntimeTarget,
    release_tag: &str,
    acceleration_config: &LocalAiRuntimeAccelerationConfig,
) -> LocalAiRuntimeInstallPlan {
    let acceleration = requested_runtime_acceleration(target, acceleration_config);
    let runtime = select_llama_runtime_distribution(target, acceleration, release_tag)
        .map(|distribution| runtime_requirement(cache_root, distribution));
    let runtime_status = runtime
        .as_ref()
        .map(|requirement| requirement.status)
        .unwrap_or(LocalAiRequiredArtifactStatus::Unsupported);
    LocalAiRuntimeInstallPlan {
        cache_root: cache_root.to_path_buf(),
        runtime_status,
        runtime,
        default_model: model_requirement(cache_root, default_local_ai_model()),
    }
}

fn runtime_requirement(
    cache_root: &Path,
    distribution: LlamaRuntimeDistribution,
) -> LocalAiRuntimeInstallRequirement {
    let binary_path = distribution.extracted_binary_path(cache_root);
    LocalAiRuntimeInstallRequirement {
        archive_path: distribution.archive_path(cache_root),
        binary_path: binary_path.clone(),
        distribution,
        status: installed_or_missing(&binary_path),
    }
}

fn model_requirement(
    cache_root: &Path,
    model: LocalAiKnownModel,
) -> LocalAiModelInstallRequirement {
    let model_path = model.cache_path(cache_root.to_path_buf());
    LocalAiModelInstallRequirement {
        model,
        status: installed_or_missing(&model_path),
        model_path,
    }
}

fn installed_or_missing(path: &Path) -> LocalAiRequiredArtifactStatus {
    if path.exists() {
        LocalAiRequiredArtifactStatus::Installed
    } else {
        LocalAiRequiredArtifactStatus::Missing
    }
}
