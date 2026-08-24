use std::{
    fs,
    path::{Component, Path, PathBuf},
};

use ocentra_parent_agent_protocol::constants::v08_supported_adapter_runtime_proof as proof;

use super::{
    app_game_adapter_host_capabilities_paths::ExecutableName,
    app_game_linux_docker_host_preflight_paths::{TrustedDockerExecutable, TrustedDockerRoot},
};

#[cfg(target_os = "linux")]
pub(super) fn trusted_docker_candidate(
    root: TrustedDockerRoot,
    executable: ExecutableName,
) -> Option<TrustedDockerExecutable> {
    let root_path = PathBuf::from(root.0);
    if !trusted_unix_directory_chain(&root_path)
        || !trusted_unix_directory_chain(Path::new(proof::DOCKER_SERVICE_CONFIG_DIRECTORY))
    {
        return None;
    }

    let candidate = root_path.join(executable.0);
    let candidate_metadata = fs::symlink_metadata(&candidate).ok()?;
    if !trusted_unix_executable(&candidate_metadata) {
        return None;
    }
    let identity = file_id::get_file_id(&candidate).ok()?;
    let canonical_candidate = fs::canonicalize(&candidate).ok()?;
    if canonical_candidate != candidate {
        return None;
    }
    let descriptor = rustix::fs::open(
        &candidate,
        rustix::fs::OFlags::RDONLY | rustix::fs::OFlags::NOFOLLOW | rustix::fs::OFlags::CLOEXEC,
        rustix::fs::Mode::empty(),
    )
    .ok()?;
    let executable: fs::File = descriptor.into();
    let descriptor_path = PathBuf::from(format!(proof::DOCKER_DESCRIPTOR_PATH_FORMAT, {
        use std::os::fd::AsRawFd;
        executable.as_raw_fd()
    }));
    (file_id::get_file_id(&descriptor_path).ok()? == identity).then_some(TrustedDockerExecutable {
        path: candidate,
        cwd: root_path,
        identity,
        executable,
    })
}

#[cfg(all(unix, not(target_os = "linux")))]
pub(super) fn trusted_docker_candidate(
    _root: TrustedDockerRoot,
    _executable: ExecutableName,
) -> Option<TrustedDockerExecutable> {
    // This preflight is Linux-specific. Without a platform-supported
    // descriptor-backed exec path, fail closed instead of falling back to a
    // replaceable Unix pathname.
    None
}

#[cfg(target_os = "linux")]
fn trusted_unix_directory_chain(path: &Path) -> bool {
    use std::os::unix::fs::{MetadataExt, PermissionsExt};

    let mut current = PathBuf::new();
    path.components().all(|component| {
        (!matches!(component, Component::CurDir | Component::ParentDir)) && {
            current.push(component.as_os_str());
            fs::symlink_metadata(&current).is_ok_and(|metadata| {
                metadata.file_type().is_dir()
                    && !metadata.file_type().is_symlink()
                    && metadata.uid() == 0
                    && metadata.permissions().mode() & 0o022 == 0
            })
        }
    })
}

#[cfg(target_os = "linux")]
fn trusted_unix_executable(metadata: &fs::Metadata) -> bool {
    use std::os::unix::fs::{MetadataExt, PermissionsExt};

    metadata.file_type().is_file()
        && !metadata.file_type().is_symlink()
        && metadata.uid() == 0
        && metadata.nlink() == 1
        && metadata.permissions().mode() & 0o022 == 0
        && metadata.permissions().mode() & 0o111 != 0
}

#[cfg(windows)]
pub(super) fn trusted_docker_candidate(
    root: TrustedDockerRoot,
    executable: ExecutableName,
) -> Option<TrustedDockerExecutable> {
    let root_path = PathBuf::from(root.0);
    if !trusted_windows_directory_chain(&root_path)
        || !windows_owner_dacl_write_authority_proven(&root_path)
    {
        return None;
    }
    let candidate = root_path.join(executable.0);
    let candidate_metadata = fs::symlink_metadata(&candidate).ok()?;
    if !trusted_windows_executable(&candidate_metadata) {
        return None;
    }
    let identity = file_id::get_file_id(&candidate).ok()?;
    if !windows_owner_dacl_write_authority_proven(&candidate) {
        return None;
    }
    let canonical_candidate = fs::canonicalize(&candidate).ok()?;
    (canonical_candidate == candidate).then_some(TrustedDockerExecutable {
        path: candidate,
        cwd: root_path,
        identity,
    })
}

#[cfg(windows)]
fn trusted_windows_directory_chain(path: &Path) -> bool {
    use std::os::windows::fs::MetadataExt;
    use windows_sys::Win32::Storage::FileSystem::FILE_ATTRIBUTE_REPARSE_POINT;

    let protected = Path::new(proof::WINDOWS_DOCKER_PROTECTED_ANCESTOR);
    if !path.starts_with(protected) {
        return false;
    }
    let mut current = protected.to_path_buf();
    loop {
        let Some(metadata) = fs::symlink_metadata(&current).ok() else {
            return false;
        };
        if !metadata.file_type().is_dir()
            || metadata.file_type().is_symlink()
            || metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
        {
            return false;
        }
        if current == path {
            return true;
        }
        let Some(relative) = path.strip_prefix(&current).ok() else {
            return false;
        };
        let Some(next) = relative.components().next() else {
            return false;
        };
        if !matches!(next, Component::Normal(_)) {
            return false;
        }
        current.push(next.as_os_str());
    }
}

#[cfg(windows)]
fn trusted_windows_executable(metadata: &fs::Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;
    use windows_sys::Win32::Storage::FileSystem::FILE_ATTRIBUTE_REPARSE_POINT;

    metadata.file_type().is_file()
        && !metadata.file_type().is_symlink()
        && metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT == 0
}

#[cfg(windows)]
fn windows_owner_dacl_write_authority_proven(_path: &Path) -> bool {
    // The current dependency set has no safe owner/DACL inspection API. A
    // path-only or reparse-only check would violate the custody invariant, so
    // Windows Docker probes remain unavailable until a reviewed safe ACL port
    // is added. This is an intentional fail-closed boundary.
    false
}

#[cfg(target_os = "linux")]
pub(super) fn revalidate_trusted_docker_candidate(candidate: &TrustedDockerExecutable) -> bool {
    let Some(metadata) = fs::symlink_metadata(&candidate.path).ok() else {
        return false;
    };
    if !trusted_unix_executable(&metadata) {
        return false;
    }
    let descriptor_path = {
        use std::os::fd::AsRawFd;
        PathBuf::from(format!(
            proof::DOCKER_DESCRIPTOR_PATH_FORMAT,
            candidate.executable.as_raw_fd()
        ))
    };
    file_id::get_file_id(&candidate.path).is_ok_and(|identity| identity == candidate.identity)
        && file_id::get_file_id(&descriptor_path)
            .is_ok_and(|identity| identity == candidate.identity)
        && candidate.path.parent() == Some(candidate.cwd.as_path())
        && trusted_unix_directory_chain(&candidate.cwd)
        && trusted_unix_directory_chain(Path::new(proof::DOCKER_SERVICE_CONFIG_DIRECTORY))
        && fs::canonicalize(&candidate.path).is_ok_and(|path| path == candidate.path)
}

#[cfg(all(unix, not(target_os = "linux")))]
pub(super) fn revalidate_trusted_docker_candidate(_candidate: &TrustedDockerExecutable) -> bool {
    false
}

#[cfg(windows)]
pub(super) fn revalidate_trusted_docker_candidate(candidate: &TrustedDockerExecutable) -> bool {
    trusted_windows_directory_chain(&candidate.cwd)
        && windows_owner_dacl_write_authority_proven(&candidate.cwd)
        && candidate.path.parent() == Some(candidate.cwd.as_path())
        && fs::symlink_metadata(&candidate.path)
            .ok()
            .is_some_and(|metadata| {
                trusted_windows_executable(&metadata)
                    && file_id::get_file_id(&candidate.path)
                        .is_ok_and(|identity| identity == candidate.identity)
            })
        && windows_owner_dacl_write_authority_proven(&candidate.path)
        && fs::canonicalize(&candidate.path).is_ok_and(|path| path == candidate.path)
}
