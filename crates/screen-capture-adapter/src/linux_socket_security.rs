use std::{
    fs,
    os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt},
    path::{Component, Path, PathBuf},
};

#[path = "linux_socket_security_chain.rs"]
mod chain;

use nix::unistd::geteuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum TrustedSocketRoot {
    X11,
    Wslg,
    Native,
}

pub(super) fn validated_socket(path: &Path) -> Option<(PathBuf, TrustedSocketRoot)> {
    let metadata = fs::symlink_metadata(path).ok()?;
    if !metadata.file_type().is_socket() || metadata.file_type().is_symlink() {
        return None;
    }

    let parent = path.parent()?;
    let canonical_parent = fs::canonicalize(parent).ok()?;
    let root = trusted_runtime_root(&canonical_parent)?;
    let canonical_path = fs::canonicalize(path).ok()?;
    if canonical_path.parent() != Some(canonical_parent.as_path()) {
        return None;
    }

    let canonical_metadata = fs::symlink_metadata(&canonical_path).ok()?;
    if !canonical_metadata.file_type().is_socket()
        || canonical_metadata.file_type().is_symlink()
        || !socket_owner_allowed(root, canonical_metadata.uid())
        || !socket_mode_allowed(root, canonical_metadata.permissions().mode())
    {
        return None;
    }

    Some((canonical_path, root))
}

pub(super) fn is_trusted_wslg_runtime(path: &Path) -> bool {
    let Some(canonical_path) = fs::canonicalize(path).ok() else {
        return false;
    };
    matches!(
        trusted_runtime_root(&canonical_path),
        Some(TrustedSocketRoot::Wslg)
    )
}

pub(super) fn is_trusted_wslg_socket(path: &Path) -> bool {
    matches!(validated_socket(path), Some((_, TrustedSocketRoot::Wslg)))
}

fn trusted_runtime_root(path: &Path) -> Option<TrustedSocketRoot> {
    if path == Path::new("/tmp/.X11-unix") && trusted_x11_directory(path) {
        return Some(TrustedSocketRoot::X11);
    }
    if path == Path::new("/mnt/wslg/runtime-dir") && trusted_runtime_directory(path, true) {
        return Some(TrustedSocketRoot::Wslg);
    }
    trusted_native_runtime_directory(path).then_some(TrustedSocketRoot::Native)
}

fn trusted_x11_directory(path: &Path) -> bool {
    chain::trusted_directory_chain(path, true, true)
}

fn trusted_native_runtime_directory(path: &Path) -> bool {
    let Ok(canonical_path) = fs::canonicalize(path) else {
        return false;
    };
    let Some(relative) = canonical_path.strip_prefix("/run/user").ok() else {
        return false;
    };
    let mut components = relative.components();
    let Some(Component::Normal(uid_component)) = components.next() else {
        return false;
    };
    if components.next().is_some() {
        return false;
    }
    let Some(uid_text) = uid_component.to_str() else {
        return false;
    };
    let Ok(uid) = uid_text.parse::<u32>() else {
        return false;
    };
    uid == geteuid().as_raw() && trusted_runtime_directory(&canonical_path, true)
}

fn trusted_runtime_directory(path: &Path, allow_root: bool) -> bool {
    chain::trusted_directory_chain(path, allow_root, false)
}

fn owner_allowed(owner: u32, allow_root: bool) -> bool {
    chain::owner_allowed(owner, allow_root)
}

fn socket_owner_allowed(root: TrustedSocketRoot, owner: u32) -> bool {
    match root {
        TrustedSocketRoot::X11 | TrustedSocketRoot::Wslg => owner_allowed(owner, true),
        TrustedSocketRoot::Native => owner_allowed(owner, false),
    }
}

fn socket_mode_allowed(root: TrustedSocketRoot, mode: u32) -> bool {
    // X11's `/tmp/.X11-unix` sockets are conventionally 0777. Replacement is
    // prevented by the sticky, root/euid-owned parent and the socket owner
    // check above; private WSLg/native runtime sockets remain non-writable by
    // group/other.
    matches!(root, TrustedSocketRoot::X11) || mode & 0o022 == 0
}
