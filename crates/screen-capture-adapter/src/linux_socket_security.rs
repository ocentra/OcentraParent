use std::{
    fs,
    os::unix::fs::{FileTypeExt, PermissionsExt},
    path::{Component, Path, PathBuf},
};

#[path = "linux_socket_security_chain.rs"]
mod chain;

use nix::unistd::geteuid;

use chain::{DirectoryModePolicy, DirectoryOwnerPolicy};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum TrustedSocketRoot {
    X11,
    Wslg,
    Native,
}

pub(super) fn validated_socket(path: &Path) -> Option<(PathBuf, TrustedSocketRoot)> {
    let Ok(metadata) = fs::symlink_metadata(path) else {
        return None;
    };
    if !metadata.file_type().is_socket() || metadata.file_type().is_symlink() {
        return None;
    }

    let parent = path.parent()?;
    let Ok(canonical_parent) = fs::canonicalize(parent) else {
        return None;
    };
    let root = trusted_runtime_root(&canonical_parent)?;
    let Ok(canonical_path) = fs::canonicalize(path) else {
        return None;
    };
    if canonical_path.parent() != Some(canonical_parent.as_path()) {
        return None;
    }

    let Ok(canonical_metadata) = fs::symlink_metadata(&canonical_path) else {
        return None;
    };
    if !canonical_metadata.file_type().is_socket()
        || canonical_metadata.file_type().is_symlink()
        || socket_owner_allowed(root, &canonical_metadata).is_none()
        || socket_mode_allowed(root, &canonical_metadata).is_none()
    {
        return None;
    }

    Some((canonical_path, root))
}

pub(super) fn is_trusted_wslg_runtime(path: &Path) -> Option<()> {
    let Ok(canonical_path) = fs::canonicalize(path) else {
        return None;
    };
    matches!(
        trusted_runtime_root(&canonical_path),
        Some(TrustedSocketRoot::Wslg)
    )
    .then_some(())
}

pub(super) fn is_trusted_wslg_socket(path: &Path) -> Option<()> {
    matches!(validated_socket(path), Some((_, TrustedSocketRoot::Wslg))).then_some(())
}

fn trusted_runtime_root(path: &Path) -> Option<TrustedSocketRoot> {
    if path == Path::new("/tmp/.X11-unix") && trusted_x11_directory(path).is_some() {
        return Some(TrustedSocketRoot::X11);
    }
    if path == Path::new("/mnt/wslg/runtime-dir")
        && trusted_runtime_directory(path, DirectoryOwnerPolicy::EuidOrRoot).is_some()
    {
        return Some(TrustedSocketRoot::Wslg);
    }
    trusted_native_runtime_directory(path).map(|()| TrustedSocketRoot::Native)
}

fn trusted_x11_directory(path: &Path) -> Option<()> {
    chain::trusted_directory_chain(
        path,
        DirectoryOwnerPolicy::EuidOrRoot,
        DirectoryModePolicy::AllowX11Sticky,
    )
}

fn trusted_native_runtime_directory(path: &Path) -> Option<()> {
    let Ok(canonical_path) = fs::canonicalize(path) else {
        return None;
    };
    let Ok(relative) = canonical_path.strip_prefix("/run/user") else {
        return None;
    };
    let mut components = relative.components();
    let Some(Component::Normal(uid_component)) = components.next() else {
        return None;
    };
    if components.next().is_some() {
        return None;
    }
    let Some(uid_text) = uid_component.to_str() else {
        return None;
    };
    let Ok(uid) = uid_text.parse::<u32>() else {
        return None;
    };
    if uid != geteuid().as_raw() {
        return None;
    }
    trusted_runtime_directory(&canonical_path, DirectoryOwnerPolicy::EuidOrRoot)
}

fn trusted_runtime_directory(path: &Path, owner_policy: DirectoryOwnerPolicy) -> Option<()> {
    chain::trusted_directory_chain(path, owner_policy, DirectoryModePolicy::Private)
}

fn socket_owner_allowed(root: TrustedSocketRoot, metadata: &fs::Metadata) -> Option<()> {
    match root {
        TrustedSocketRoot::X11 | TrustedSocketRoot::Wslg => {
            chain::owner_allowed(metadata, DirectoryOwnerPolicy::EuidOrRoot)
        }
        TrustedSocketRoot::Native => chain::owner_allowed(metadata, DirectoryOwnerPolicy::EuidOnly),
    }
}

fn socket_mode_allowed(root: TrustedSocketRoot, metadata: &fs::Metadata) -> Option<()> {
    // X11's `/tmp/.X11-unix` sockets are conventionally 0777. Replacement is
    // prevented by the sticky, root/euid-owned parent and the socket owner
    // check above; private WSLg/native runtime sockets remain non-writable by
    // group/other.
    if matches!(root, TrustedSocketRoot::X11) || metadata.permissions().mode() & 0o022 == 0 {
        Some(())
    } else {
        None
    }
}
