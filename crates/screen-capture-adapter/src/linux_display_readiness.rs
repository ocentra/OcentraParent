use std::{fs, os::unix::fs::FileTypeExt, path::Path};

use super::{LinuxDisplayReadiness, LinuxSocketReadiness};

pub(super) fn display_readiness(
    configured: bool,
    valid_configuration: bool,
    socket_ready: bool,
) -> LinuxDisplayReadiness {
    match (configured, valid_configuration, socket_ready) {
        (false, _, _) | (true, true, false) => LinuxDisplayReadiness::Missing,
        (true, _, true) => LinuxDisplayReadiness::Ready,
        (true, false, false) => LinuxDisplayReadiness::Invalid,
    }
}

pub(super) fn socket_readiness(path: Option<&Path>) -> LinuxSocketReadiness {
    match path {
        Some(path) if unix_socket_ready(path) => LinuxSocketReadiness::Ready,
        Some(_) => LinuxSocketReadiness::Missing,
        None => LinuxSocketReadiness::Unavailable,
    }
}

pub(super) fn wslg_environment_hint() -> bool {
    ["WSL_INTEROP", "WSL_DISTRO_NAME"]
        .into_iter()
        .filter_map(std::env::var_os)
        .any(|value| !value.is_empty())
        || std::env::var_os("XDG_RUNTIME_DIR")
            .and_then(|value| value.into_string().ok())
            .map(|value| value.to_ascii_lowercase().contains("wslg"))
            .unwrap_or(false)
}

fn unix_socket_ready(path: &Path) -> bool {
    fs::metadata(path)
        .map(|metadata| metadata.file_type().is_socket())
        .unwrap_or(false)
}
