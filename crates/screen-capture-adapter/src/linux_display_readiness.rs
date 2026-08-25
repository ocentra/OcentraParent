use std::{path::Path, time::Duration};

use super::{
    linux_socket_connect::socket_ready,
    linux_socket_security::{is_trusted_wslg_runtime, is_trusted_wslg_socket},
    LinuxDisplayReadiness, LinuxSocketReadiness,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum DisplayConfiguration {
    Configured,
    Unconfigured,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum DisplayConfigurationValidity {
    Valid,
    Invalid,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum LinuxWslEnvironmentHint {
    Present,
    Absent,
}

pub(super) fn display_readiness(
    configured: DisplayConfiguration,
    valid_configuration: DisplayConfigurationValidity,
    socket_ready: LinuxSocketReadiness,
) -> LinuxDisplayReadiness {
    match (
        configured,
        valid_configuration,
        matches!(socket_ready, LinuxSocketReadiness::Ready),
    ) {
        (DisplayConfiguration::Unconfigured, _, _)
        | (DisplayConfiguration::Configured, DisplayConfigurationValidity::Valid, false) => {
            LinuxDisplayReadiness::Missing
        }
        (DisplayConfiguration::Configured, _, true) => LinuxDisplayReadiness::Ready,
        (DisplayConfiguration::Configured, DisplayConfigurationValidity::Invalid, false) => {
            LinuxDisplayReadiness::Invalid
        }
    }
}

pub(super) fn socket_readiness(
    path: Option<&Path>,
    remaining: &impl Fn() -> Duration,
) -> LinuxSocketReadiness {
    match path {
        Some(path) if socket_ready(path, remaining).is_some() => LinuxSocketReadiness::Ready,
        Some(_) => LinuxSocketReadiness::Missing,
        None => LinuxSocketReadiness::Unavailable,
    }
}

pub(super) fn wslg_environment_hint(
    runtime_dir: Option<&Path>,
    wayland_path: Option<&Path>,
    wayland_socket: LinuxSocketReadiness,
) -> LinuxWslEnvironmentHint {
    if !matches!(wayland_socket, LinuxSocketReadiness::Ready) {
        return LinuxWslEnvironmentHint::Absent;
    }
    let Some(runtime_dir) = runtime_dir else {
        return LinuxWslEnvironmentHint::Absent;
    };
    let Some(wayland_path) = wayland_path else {
        return LinuxWslEnvironmentHint::Absent;
    };
    if is_trusted_wslg_runtime(runtime_dir).is_none()
        || is_trusted_wslg_socket(wayland_path).is_none()
        || !matches!(wsl_environment_hint(), LinuxWslEnvironmentHint::Present)
    {
        LinuxWslEnvironmentHint::Absent
    } else {
        LinuxWslEnvironmentHint::Present
    }
}

pub(super) fn wsl_environment_hint() -> LinuxWslEnvironmentHint {
    let interop_present = std::env::var_os("WSL_INTEROP").is_some_and(|value| !value.is_empty());
    let distro_present = std::env::var_os("WSL_DISTRO_NAME").is_some_and(|value| !value.is_empty());
    if interop_present || distro_present {
        LinuxWslEnvironmentHint::Present
    } else {
        LinuxWslEnvironmentHint::Absent
    }
}
