#[cfg(all(target_os = "linux", not(target_env = "ohos")))]
use std::time::{Duration, Instant};

#[cfg(all(target_os = "linux", not(target_env = "ohos")))]
#[path = "linux_display.rs"]
pub(crate) mod linux_display;
#[cfg(all(target_os = "linux", not(target_env = "ohos")))]
#[path = "linux_display_paths.rs"]
pub(crate) mod linux_display_paths;
#[cfg(all(target_os = "linux", not(target_env = "ohos")))]
#[path = "linux_display_readiness.rs"]
pub(crate) mod linux_display_readiness;
#[cfg(all(target_os = "linux", not(target_env = "ohos")))]
#[path = "linux_process.rs"]
pub(crate) mod linux_process;
#[cfg(all(target_os = "linux", not(target_env = "ohos")))]
#[path = "linux_socket_connect.rs"]
pub(crate) mod linux_socket_connect;
#[cfg(all(target_os = "linux", not(target_env = "ohos")))]
#[path = "linux_socket_security.rs"]
pub(crate) mod linux_socket_security;
#[cfg(all(target_os = "linux", not(target_env = "ohos")))]
#[path = "linux_tools.rs"]
pub(crate) mod linux_tools;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinuxDisplayEnvironment {
    Wslg,
    Native,
    Unknown,
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinuxDisplayReadiness {
    Ready,
    Missing,
    Invalid,
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinuxSocketReadiness {
    Ready,
    Missing,
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinuxToolProbe {
    Succeeded,
    Failed,
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinuxActiveWindowObservation {
    Observed,
    NotObserved,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LinuxForegroundSourcePreflight {
    pub display_environment: LinuxDisplayEnvironment,
    pub display: LinuxDisplayReadiness,
    pub x11_socket: LinuxSocketReadiness,
    pub wayland_socket: LinuxSocketReadiness,
    pub xprop: LinuxToolProbe,
    pub xdotool: LinuxToolProbe,
    pub active_window: LinuxActiveWindowObservation,
}

impl LinuxForegroundSourcePreflight {
    pub fn unavailable() -> Self {
        Self {
            display_environment: LinuxDisplayEnvironment::Unavailable,
            display: LinuxDisplayReadiness::Unavailable,
            x11_socket: LinuxSocketReadiness::Unavailable,
            wayland_socket: LinuxSocketReadiness::Unavailable,
            xprop: LinuxToolProbe::Unavailable,
            xdotool: LinuxToolProbe::Unavailable,
            active_window: LinuxActiveWindowObservation::NotObserved,
        }
    }

    pub fn display_ready(self) -> bool {
        matches!(self.display, LinuxDisplayReadiness::Ready)
    }

    pub fn socket_ready(self) -> bool {
        matches!(self.x11_socket, LinuxSocketReadiness::Ready)
            || matches!(self.wayland_socket, LinuxSocketReadiness::Ready)
    }

    pub fn source_ready(self) -> bool {
        self.display_ready()
            && self.socket_ready()
            && (matches!(self.xprop, LinuxToolProbe::Succeeded)
                || matches!(self.xdotool, LinuxToolProbe::Succeeded))
    }

    pub fn active_window_observed(self) -> bool {
        matches!(self.active_window, LinuxActiveWindowObservation::Observed)
    }
}

pub fn foreground_source_preflight() -> LinuxForegroundSourcePreflight {
    #[cfg(all(target_os = "linux", not(target_env = "ohos")))]
    {
        return foreground_source_preflight_with_deadline(Instant::now() + Duration::from_secs(2));
    }

    #[cfg(not(all(target_os = "linux", not(target_env = "ohos"))))]
    {
        LinuxForegroundSourcePreflight::unavailable()
    }
}

#[cfg(all(target_os = "linux", not(target_env = "ohos")))]
pub(crate) fn foreground_source_preflight_with_deadline(
    deadline: Instant,
) -> LinuxForegroundSourcePreflight {
    let display = linux_display::display_probe(deadline);
    let (xprop, xprop_observation) = linux_tools::probe_xprop(deadline);
    let (xdotool, xdotool_observation) = linux_tools::probe_xdotool(deadline);
    let active_window = if matches!(xprop_observation, LinuxActiveWindowObservation::Observed)
        || matches!(xdotool_observation, LinuxActiveWindowObservation::Observed)
    {
        LinuxActiveWindowObservation::Observed
    } else {
        LinuxActiveWindowObservation::NotObserved
    };

    LinuxForegroundSourcePreflight {
        display_environment: display.environment,
        display: display.readiness,
        x11_socket: display.x11_socket,
        wayland_socket: display.wayland_socket,
        xprop,
        xdotool,
        active_window,
    }
}
