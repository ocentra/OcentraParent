use ocentra_parent_agent_protocol::app_game_adapter_execution_readiness::APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_DETECTED;
use ocentra_parent_agent_protocol::app_game_authority_classifier::APP_GAME_PARENT_PLATFORM_LINUX;
use ocentra_parent_agent_protocol::app_game_platform_proof_status::{
    APP_GAME_PLATFORM_AUTHORITY_VISIBILITY_ONLY, APP_GAME_PLATFORM_GAP_LINUX_FOREGROUND_CAPTURE,
    APP_GAME_PLATFORM_PROOF_LINUX_HOST_NOT_DETECTED,
};
use ocentra_parent_screen_capture_adapter::linux_foreground_source::{
    LinuxActiveWindowObservation, LinuxDisplayEnvironment, LinuxDisplayReadiness,
    LinuxForegroundSourcePreflight, LinuxSocketReadiness, LinuxToolProbe,
};

use crate::app_game_adapter_host_capabilities::{CapabilityState, HostCapabilitySignals};
use crate::app_game_linux_docker_host_preflight::unavailable_linux_docker_host_preflight;
use crate::app_game_platform_proof_status_payload::app_game_platform_proof_status_read_model_from_preflights;
use crate::app_game_platform_proof_status_payload::PlatformProofGeneratedAtText;

fn unavailable_host_signals() -> HostCapabilitySignals {
    HostCapabilitySignals {
        android_adb: false,
        android_adb_path: false,
        android_adb_sdk: false,
    }
}

fn caller_minted_observations() -> LinuxForegroundSourcePreflight {
    let mut preflight = LinuxForegroundSourcePreflight::unavailable();
    preflight.display_environment = LinuxDisplayEnvironment::Wslg;
    preflight.display = LinuxDisplayReadiness::Ready;
    preflight.x11_socket = LinuxSocketReadiness::Ready;
    preflight.wayland_socket = LinuxSocketReadiness::Ready;
    preflight.xprop = LinuxToolProbe::Succeeded;
    preflight.xdotool = LinuxToolProbe::Succeeded;
    preflight.active_window = LinuxActiveWindowObservation::Observed;
    preflight
}

#[test]
fn host_capability_route_keeps_caller_observations_not_detected() {
    let signals = unavailable_host_signals();
    let preflight = caller_minted_observations();

    let CapabilityState(state) = signals.linux_state_for(&preflight);

    assert_eq!(state, APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_DETECTED);
    assert_eq!(
        preflight.source_ready(),
        ocentra_parent_screen_capture_adapter::linux_foreground_source::LinuxSourceReadiness::Unavailable
    );
    assert_eq!(
        preflight.active_window_observed(),
        LinuxActiveWindowObservation::NotObserved
    );
}

#[test]
fn platform_status_route_preserves_linux_fail_closed_gaps_and_claims() -> Result<(), &'static str> {
    let signals = unavailable_host_signals();
    let preflight = caller_minted_observations();
    let docker = unavailable_linux_docker_host_preflight();
    let model = app_game_platform_proof_status_read_model_from_preflights(
        PlatformProofGeneratedAtText("2026-08-28T00:00:00Z".to_string()),
        &signals,
        &preflight,
        &docker,
    );
    let linux = model
        .rows
        .iter()
        .find(|row| row.platform == APP_GAME_PARENT_PLATFORM_LINUX)
        .ok_or("the typed platform status model must include Linux")?;

    assert_eq!(
        linux.host_capability_state,
        APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_DETECTED
    );
    assert_eq!(
        linux.proof_state,
        APP_GAME_PLATFORM_PROOF_LINUX_HOST_NOT_DETECTED
    );
    assert_eq!(
        linux.authority_state,
        APP_GAME_PLATFORM_AUTHORITY_VISIBILITY_ONLY
    );
    assert_eq!(linux.host_capability_evidence_refs, Vec::<String>::new());
    assert_eq!(linux.host_capability_probe_refs, Vec::<String>::new());
    assert!(linux
        .open_gaps
        .contains(&APP_GAME_PLATFORM_GAP_LINUX_FOREGROUND_CAPTURE.to_string()));
    assert!(!linux.adapter_dispatch_claimed);
    assert!(!linux.platform_enforcement_claimed);
    assert!(!linux.provider_delivery_claimed);
    assert!(!linux.child_device_delivery_claimed);
    assert!(!linux.private_diagnostics_claimed);
    Ok(())
}
