use ocentra_parent_agent_protocol::constants::v08_supported_adapter_runtime_proof as proof;

use super::{
    app_game_adapter_host_capabilities_paths::ExecutableName,
    app_game_linux_docker_host_preflight_path_security::trusted_docker_candidate,
};

use std::path::PathBuf;

#[cfg(target_os = "linux")]
use std::fs::File;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct TrustedDockerRoot(pub(super) &'static str);

#[derive(Debug)]
pub(super) struct TrustedDockerExecutable {
    pub(super) path: PathBuf,
    pub(super) cwd: PathBuf,
    pub(super) identity: file_id::FileId,
    #[cfg(target_os = "linux")]
    pub(super) executable: File,
}

impl TrustedDockerExecutable {
    pub(super) fn try_clone(&self) -> Option<Self> {
        Some(Self {
            path: self.path.clone(),
            cwd: self.cwd.clone(),
            identity: self.identity,
            #[cfg(target_os = "linux")]
            executable: self.executable.try_clone().ok()?,
        })
    }
}

pub(super) fn resolve_trusted_docker_executable() -> Option<TrustedDockerExecutable> {
    #[cfg(windows)]
    let executable = ExecutableName(proof::WINDOWS_DOCKER_EXECUTABLE);

    #[cfg(unix)]
    let executable = ExecutableName(proof::EXE_DOCKER);

    #[cfg(windows)]
    let roots = proof::WINDOWS_DOCKER_TRUSTED_ROOTS;

    #[cfg(unix)]
    let roots = proof::LINUX_DOCKER_TRUSTED_ROOTS;

    roots
        .into_iter()
        .find_map(|root| trusted_docker_candidate(TrustedDockerRoot(root), executable))
}
