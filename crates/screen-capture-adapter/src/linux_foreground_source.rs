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
#[path = "linux_socket_connect.rs"]
pub(crate) mod linux_socket_connect;
#[cfg(all(target_os = "linux", not(target_env = "ohos")))]
#[path = "linux_socket_security.rs"]
pub(crate) mod linux_socket_security;

/// Classifies the trusted local Linux display environment found by preflight.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinuxDisplayEnvironment {
    Wslg,
    Native,
    Unknown,
    Unavailable,
}

/// Reports whether the configured Linux display passed local readiness checks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinuxDisplayReadiness {
    Ready,
    Missing,
    Invalid,
    Unavailable,
}

/// Reports whether a validated local Linux display socket accepted a connection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinuxSocketReadiness {
    Ready,
    Missing,
    Unavailable,
}

/// Records the result of a trusted foreground-tool probe.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinuxToolProbe {
    Succeeded,
    Failed,
    Unavailable,
}

/// Records whether a trusted source observed the active Linux window.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinuxActiveWindowObservation {
    Observed,
    NotObserved,
}

/// Reports whether the Linux foreground source crossed its attestation boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinuxSourceReadiness {
    Ready,
    Unavailable,
}

/// Captures fail-closed Linux display, socket, tool, and source observations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LinuxForegroundSourcePreflight {
    pub display_environment: LinuxDisplayEnvironment,
    pub display: LinuxDisplayReadiness,
    pub x11_socket: LinuxSocketReadiness,
    pub wayland_socket: LinuxSocketReadiness,
    pub xprop: LinuxToolProbe,
    pub xdotool: LinuxToolProbe,
    pub active_window: LinuxActiveWindowObservation,
    // BRAND-INVARIANT: only an owner-controlled OS custody boundary may set
    // this attestation; unavailable/default construction keeps it false.
    source_attested: bool,
}

impl LinuxForegroundSourcePreflight {
    /// Returns a preflight with every Linux source capability unavailable.
    pub fn unavailable() -> Self {
        Self {
            display_environment: LinuxDisplayEnvironment::Unavailable,
            display: LinuxDisplayReadiness::Unavailable,
            x11_socket: LinuxSocketReadiness::Unavailable,
            wayland_socket: LinuxSocketReadiness::Unavailable,
            xprop: LinuxToolProbe::Unavailable,
            xdotool: LinuxToolProbe::Unavailable,
            active_window: LinuxActiveWindowObservation::NotObserved,
            source_attested: false,
        }
    }

    /// Returns the validated display readiness recorded by this preflight.
    pub fn display_ready(self) -> LinuxDisplayReadiness {
        self.display
    }

    /// Aggregates the validated X11 and Wayland socket readiness states.
    pub fn socket_ready(self) -> LinuxSocketReadiness {
        if matches!(self.x11_socket, LinuxSocketReadiness::Ready)
            || matches!(self.wayland_socket, LinuxSocketReadiness::Ready)
        {
            LinuxSocketReadiness::Ready
        } else if matches!(self.x11_socket, LinuxSocketReadiness::Unavailable)
            && matches!(self.wayland_socket, LinuxSocketReadiness::Unavailable)
        {
            LinuxSocketReadiness::Unavailable
        } else {
            LinuxSocketReadiness::Missing
        }
    }

    /// Returns ready only when a trusted X11 source attested the observation.
    pub fn source_ready(self) -> LinuxSourceReadiness {
        if self.source_attested
            && matches!(self.display_ready(), LinuxDisplayReadiness::Ready)
            && matches!(
                self.display_environment,
                LinuxDisplayEnvironment::Native | LinuxDisplayEnvironment::Wslg
            )
            // xprop/xdotool are X11 tools. A Wayland socket or a remote/invalid
            // DISPLAY cannot authorize a foreground-source result.
            && matches!(self.x11_socket, LinuxSocketReadiness::Ready)
            && (matches!(self.xprop, LinuxToolProbe::Succeeded)
                || matches!(self.xdotool, LinuxToolProbe::Succeeded))
        {
            LinuxSourceReadiness::Ready
        } else {
            LinuxSourceReadiness::Unavailable
        }
    }

    /// Returns observed only when both source attestation and observation agree.
    pub fn active_window_observed(self) -> LinuxActiveWindowObservation {
        if matches!(self.source_ready(), LinuxSourceReadiness::Ready)
            && matches!(self.active_window, LinuxActiveWindowObservation::Observed)
        {
            LinuxActiveWindowObservation::Observed
        } else {
            LinuxActiveWindowObservation::NotObserved
        }
    }
}

/// Probes bounded local Linux display readiness without minting source authority.
pub fn foreground_source_preflight() -> LinuxForegroundSourcePreflight {
    #[cfg(all(target_os = "linux", not(target_env = "ohos")))]
    {
        let deadline = Instant::now() + Duration::from_secs(2);
        let remaining = || deadline.saturating_duration_since(Instant::now());
        return foreground_source_preflight_with_budget(&remaining);
    }

    #[cfg(not(all(target_os = "linux", not(target_env = "ohos"))))]
    {
        LinuxForegroundSourcePreflight::unavailable()
    }
}

#[cfg(all(target_os = "linux", not(target_env = "ohos")))]
fn foreground_source_preflight_with_budget(
    remaining: &impl Fn() -> Duration,
) -> LinuxForegroundSourcePreflight {
    let display = linux_display::display_probe(remaining);

    // External xprop/xdotool execution is intentionally disabled. The source
    // phase has no OS primitive that guarantees custody of a process group
    // across setsid/pid-namespace escapes, so a live tool result would not be
    // trustworthy. Keep the typed fields unavailable and never mint an active
    // or probe reference from display/socket readiness alone.
    let xprop = LinuxToolProbe::Unavailable;
    let xdotool = LinuxToolProbe::Unavailable;
    let active_window = LinuxActiveWindowObservation::NotObserved;

    LinuxForegroundSourcePreflight {
        display_environment: display.environment,
        display: display.readiness,
        x11_socket: display.x11_socket,
        wayland_socket: display.wayland_socket,
        xprop,
        xdotool,
        active_window,
        source_attested: false,
    }
}
