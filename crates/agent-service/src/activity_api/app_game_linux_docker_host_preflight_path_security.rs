use std::{fs, path::PathBuf};

use ocentra_parent_agent_protocol::constants::v08_supported_adapter_runtime_proof as proof;

use super::{
    app_game_adapter_host_capabilities_paths::{ExecutableName, ResolvedExecutablePath},
    app_game_linux_docker_host_preflight_paths::TrustedDockerRoot,
};

#[cfg(unix)]
pub(super) fn trusted_docker_candidate(
    root: TrustedDockerRoot,
    executable: ExecutableName,
) -> Option<ResolvedExecutablePath> {
    let canonical_root = fs::canonicalize(PathBuf::from(root.0)).ok()?;
    if !trusted_unix_directory_chain(&canonical_root) {
        return None;
    }

    let candidate = canonical_root.join(executable.0);
    let candidate_metadata = fs::symlink_metadata(&candidate).ok()?;
    if !trusted_unix_executable(&candidate_metadata) {
        return None;
    }

    let canonical_candidate = fs::canonicalize(candidate).ok()?;
    let canonical_metadata = fs::symlink_metadata(&canonical_candidate).ok()?;
    (canonical_candidate.parent() == Some(canonical_root.as_path())
        && trusted_unix_executable(&canonical_metadata))
    .then_some(ResolvedExecutablePath(canonical_candidate))
}

#[cfg(unix)]
fn trusted_unix_directory_chain(path: &std::path::Path) -> bool {
    use std::os::unix::fs::{MetadataExt, PermissionsExt};

    path.ancestors().all(|ancestor| {
        fs::symlink_metadata(ancestor).is_ok_and(|metadata| {
            metadata.file_type().is_dir()
                && metadata.uid() == 0
                && metadata.permissions().mode() & 0o022 == 0
        })
    })
}

#[cfg(unix)]
fn trusted_unix_executable(metadata: &fs::Metadata) -> bool {
    use std::os::unix::fs::{MetadataExt, PermissionsExt};

    metadata.file_type().is_file()
        && !metadata.file_type().is_symlink()
        && metadata.uid() == 0
        && metadata.permissions().mode() & 0o022 == 0
        && metadata.permissions().mode() & 0o111 != 0
}

#[cfg(windows)]
pub(super) fn trusted_docker_candidate(
    root: TrustedDockerRoot,
    executable: ExecutableName,
) -> Option<ResolvedExecutablePath> {
    let protected_ancestor = fs::canonicalize(proof::WINDOWS_DOCKER_PROTECTED_ANCESTOR).ok()?;
    let canonical_root = fs::canonicalize(PathBuf::from(root.0)).ok()?;
    let root_metadata = fs::symlink_metadata(&canonical_root).ok()?;
    if !canonical_root.starts_with(&protected_ancestor)
        || !root_metadata.file_type().is_dir()
        || root_metadata.file_type().is_symlink()
    {
        return None;
    }

    let candidate = canonical_root.join(executable.0);
    let candidate_metadata = fs::symlink_metadata(&candidate).ok()?;
    if !candidate_metadata.file_type().is_file() || candidate_metadata.file_type().is_symlink() {
        return None;
    }

    let canonical_candidate = fs::canonicalize(candidate).ok()?;
    (canonical_candidate.parent() == Some(canonical_root.as_path()))
        .then_some(ResolvedExecutablePath(canonical_candidate))
}
