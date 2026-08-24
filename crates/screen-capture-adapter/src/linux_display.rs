use super::{
    linux_display_paths::{wayland_socket_path, x11_socket_path},
    linux_display_readiness::{display_readiness, socket_readiness, wslg_environment_hint},
    LinuxDisplayEnvironment,
};

pub(super) struct DisplayProbe {
    pub(super) environment: LinuxDisplayEnvironment,
    pub(super) readiness: LinuxDisplayReadiness,
    pub(super) x11_socket: LinuxSocketReadiness,
    pub(super) wayland_socket: LinuxSocketReadiness,
}

pub(super) fn display_probe() -> DisplayProbe {
    let display = std::env::var_os("DISPLAY");
    let wayland_display = std::env::var_os("WAYLAND_DISPLAY");
    let runtime_dir = std::env::var_os("XDG_RUNTIME_DIR");
    let x11_path = display.as_deref().and_then(x11_socket_path);
    let wayland_path = wayland_socket_path(wayland_display.as_deref(), runtime_dir.as_deref());
    let x11_socket = socket_readiness(x11_path.as_deref());
    let wayland_socket = socket_readiness(wayland_path.as_deref());
    let configured = display.is_some() || wayland_display.is_some();
    let valid_configuration = x11_path.is_some() || wayland_path.is_some();
    let socket_ready = matches!(x11_socket, LinuxSocketReadiness::Ready)
        || matches!(wayland_socket, LinuxSocketReadiness::Ready);
    let readiness = display_readiness(configured, valid_configuration, socket_ready);
    let environment = match readiness {
        super::LinuxDisplayReadiness::Ready if wslg_environment_hint() => {
            LinuxDisplayEnvironment::Wslg
        }
        super::LinuxDisplayReadiness::Ready => LinuxDisplayEnvironment::Native,
        _ => LinuxDisplayEnvironment::Unknown,
    };

    DisplayProbe {
        environment,
        readiness,
        x11_socket,
        wayland_socket,
    }
}
