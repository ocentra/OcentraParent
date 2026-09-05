#![forbid(unsafe_code)]

use ocentra_parent_screen_capture_adapter::linux_foreground_source::{
    foreground_source_preflight, LinuxActiveWindowObservation, LinuxDisplayEnvironment,
    LinuxDisplayReadiness, LinuxForegroundSourcePreflight, LinuxSocketReadiness,
    LinuxSourceReadiness, LinuxToolProbe,
};

#[test]
fn unavailable_preflight_is_fully_fail_closed() {
    let preflight = LinuxForegroundSourcePreflight::unavailable();

    assert_eq!(
        preflight.display_environment,
        LinuxDisplayEnvironment::Unavailable
    );
    assert_eq!(
        preflight.display_ready(),
        LinuxDisplayReadiness::Unavailable
    );
    assert_eq!(preflight.x11_socket, LinuxSocketReadiness::Unavailable);
    assert_eq!(preflight.wayland_socket, LinuxSocketReadiness::Unavailable);
    assert_eq!(preflight.socket_ready(), LinuxSocketReadiness::Unavailable);
    assert_eq!(preflight.xprop, LinuxToolProbe::Unavailable);
    assert_eq!(preflight.xdotool, LinuxToolProbe::Unavailable);
    assert_eq!(
        preflight.active_window,
        LinuxActiveWindowObservation::NotObserved
    );
    assert_eq!(preflight.source_ready(), LinuxSourceReadiness::Unavailable);
    assert_eq!(
        preflight.active_window_observed(),
        LinuxActiveWindowObservation::NotObserved
    );
}

#[test]
fn caller_mutable_observations_cannot_mint_source_attestation() {
    let mut preflight = LinuxForegroundSourcePreflight::unavailable();
    preflight.display_environment = LinuxDisplayEnvironment::Native;
    preflight.display = LinuxDisplayReadiness::Ready;
    preflight.x11_socket = LinuxSocketReadiness::Ready;
    preflight.wayland_socket = LinuxSocketReadiness::Ready;
    preflight.xprop = LinuxToolProbe::Succeeded;
    preflight.xdotool = LinuxToolProbe::Succeeded;
    preflight.active_window = LinuxActiveWindowObservation::Observed;

    assert_eq!(preflight.socket_ready(), LinuxSocketReadiness::Ready);
    assert_eq!(preflight.source_ready(), LinuxSourceReadiness::Unavailable);
    assert_eq!(
        preflight.active_window_observed(),
        LinuxActiveWindowObservation::NotObserved
    );
}

#[test]
fn wayland_only_or_invalid_display_observations_remain_unavailable() {
    let mut wayland = LinuxForegroundSourcePreflight::unavailable();
    wayland.display_environment = LinuxDisplayEnvironment::Native;
    wayland.display = LinuxDisplayReadiness::Ready;
    wayland.x11_socket = LinuxSocketReadiness::Missing;
    wayland.wayland_socket = LinuxSocketReadiness::Ready;
    wayland.xprop = LinuxToolProbe::Succeeded;

    assert_eq!(wayland.socket_ready(), LinuxSocketReadiness::Ready);
    assert_eq!(wayland.source_ready(), LinuxSourceReadiness::Unavailable);

    let mut invalid_display = wayland;
    invalid_display.x11_socket = LinuxSocketReadiness::Ready;
    invalid_display.display = LinuxDisplayReadiness::Invalid;

    assert_eq!(
        invalid_display.source_ready(),
        LinuxSourceReadiness::Unavailable
    );
    assert_eq!(
        invalid_display.active_window_observed(),
        LinuxActiveWindowObservation::NotObserved
    );
}

#[cfg(all(target_os = "linux", not(target_env = "ohos")))]
#[test]
fn live_linux_preflight_does_not_promote_display_readiness_to_foreground_authority() {
    let preflight = foreground_source_preflight();

    assert_eq!(preflight.xprop, LinuxToolProbe::Unavailable);
    assert_eq!(preflight.xdotool, LinuxToolProbe::Unavailable);
    assert_eq!(
        preflight.active_window,
        LinuxActiveWindowObservation::NotObserved
    );
    assert_eq!(preflight.source_ready(), LinuxSourceReadiness::Unavailable);
    assert_eq!(
        preflight.active_window_observed(),
        LinuxActiveWindowObservation::NotObserved
    );
}
