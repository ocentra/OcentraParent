use std::{
    fs,
    os::unix::fs::{MetadataExt, PermissionsExt},
    path::Path,
};

use nix::unistd::geteuid;

pub(super) fn trusted_directory_chain(
    path: &Path,
    allow_root: bool,
    allow_x11_sticky: bool,
) -> bool {
    let Ok(canonical_path) = fs::canonicalize(path) else {
        return false;
    };
    let mut current = canonical_path.as_path();
    loop {
        let Some(metadata) = fs::symlink_metadata(current).ok() else {
            return false;
        };
        if !metadata.file_type().is_dir() || !owner_allowed(metadata.uid(), allow_root) {
            return false;
        }
        let mode = metadata.permissions().mode();
        if !directory_mode_allowed(current, mode, allow_x11_sticky) {
            return false;
        }
        if current == Path::new("/") {
            return true;
        }
        let Some(parent) = current.parent() else {
            return false;
        };
        current = parent;
    }
}

pub(super) fn owner_allowed(owner: u32, allow_root: bool) -> bool {
    owner == geteuid().as_raw() || (allow_root && owner == 0)
}

fn directory_mode_allowed(path: &Path, mode: u32, allow_x11_sticky: bool) -> bool {
    let x11_sticky_directory =
        allow_x11_sticky && (path == Path::new("/tmp") || path == Path::new("/tmp/.X11-unix"));
    if x11_sticky_directory {
        mode & 0o1000 != 0 && mode & 0o002 != 0
    } else {
        mode & 0o022 == 0
    }
}
