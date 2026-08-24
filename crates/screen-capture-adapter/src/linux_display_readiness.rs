use std::{path::Path, time::Instant};

use super::{
    linux_socket_connect::socket_ready,
    linux_socket_security::{is_trusted_wslg_runtime, is_trusted_wslg_socket},
    LinuxDisplayReadiness, LinuxSocketReadiness,
};

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

pub(super) fn socket_readiness(path: Option<&Path>, deadline: Instant) -> LinuxSocketReadiness {
    match path {
        Some(path) if socket_ready(path, deadline) => LinuxSocketReadiness::Ready,
        Some(_) => LinuxSocketReadiness::Missing,
        None => LinuxSocketReadiness::Unavailable,
    }
}

pub(super) fn wslg_environment_hint(
    runtime_dir: Option<&Path>,
    wayland_path: Option<&Path>,
    wayland_socket: LinuxSocketReadiness,
) -> bool {
    matches!(wayland_socket, LinuxSocketReadiness::Ready)
        && runtime_dir.is_some_and(is_trusted_wslg_runtime)
        && wayland_path.is_some_and(is_trusted_wslg_socket)
        && wsl_environment_hint()
}

pub(super) fn wsl_environment_hint() -> bool {
    ["WSL_INTEROP", "WSL_DISTRO_NAME"]
        .into_iter()
        .filter_map(std::env::var_os)
        .any(|value| !value.is_empty())
}
