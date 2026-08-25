use std::{
    fs,
    os::unix::fs::{MetadataExt, PermissionsExt},
    path::Path,
};

use nix::unistd::geteuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum DirectoryOwnerPolicy {
    EuidOnly,
    EuidOrRoot,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum DirectoryModePolicy {
    Private,
    AllowX11Sticky,
}

pub(super) fn trusted_directory_chain(
    path: &Path,
    owner_policy: DirectoryOwnerPolicy,
    mode_policy: DirectoryModePolicy,
) -> Option<()> {
    let Ok(canonical_path) = fs::canonicalize(path) else {
        return None;
    };
    let mut current = canonical_path.as_path();
    loop {
        let Ok(metadata) = fs::symlink_metadata(current) else {
            return None;
        };
        if !metadata.file_type().is_dir() || owner_allowed(&metadata, owner_policy).is_none() {
            return None;
        }
        if directory_mode_allowed(current, &metadata, mode_policy).is_none() {
            return None;
        }
        if current == Path::new("/") {
            return Some(());
        }
        let Some(parent) = current.parent() else {
            return None;
        };
        current = parent;
    }
}

pub(super) fn owner_allowed(
    metadata: &fs::Metadata,
    owner_policy: DirectoryOwnerPolicy,
) -> Option<()> {
    let owner = metadata.uid();
    if owner == geteuid().as_raw()
        || (matches!(owner_policy, DirectoryOwnerPolicy::EuidOrRoot) && owner == 0)
    {
        Some(())
    } else {
        None
    }
}

fn directory_mode_allowed(
    path: &Path,
    metadata: &fs::Metadata,
    mode_policy: DirectoryModePolicy,
) -> Option<()> {
    let mode = metadata.permissions().mode();
    let x11_sticky_directory = matches!(mode_policy, DirectoryModePolicy::AllowX11Sticky)
        && (path == Path::new("/tmp") || path == Path::new("/tmp/.X11-unix"));
    if x11_sticky_directory {
        (mode & 0o1000 != 0 && mode & 0o002 != 0).then_some(())
    } else {
        (mode & 0o022 == 0).then_some(())
    }
}
