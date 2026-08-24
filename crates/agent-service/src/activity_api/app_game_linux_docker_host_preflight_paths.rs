use ocentra_parent_agent_protocol::constants::v08_supported_adapter_runtime_proof as proof;

use super::{
    app_game_adapter_host_capabilities_paths::{ExecutableName, ResolvedExecutablePath},
    app_game_linux_docker_host_preflight_path_security::trusted_docker_candidate,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct TrustedDockerRoot(pub(super) &'static str);

pub(super) fn resolve_trusted_docker_executable() -> Option<ResolvedExecutablePath> {
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
