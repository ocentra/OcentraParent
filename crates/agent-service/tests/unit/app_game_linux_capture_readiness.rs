#![cfg(target_os = "linux")]

use crate::app_game_platform_proof_status_payload::app_game_platform_proof_status_read_model_with_linux_preflight;
use crate::app_game_adapter_execution_readiness_payload::GeneratedAtText;
use ocentra_parent_agent_protocol::app_game_authority_classifier::APP_GAME_PARENT_PLATFORM_LINUX;
use ocentra_parent_agent_protocol::app_game_platform_proof_status::{APP_GAME_PLATFORM_AUTHORITY_VISIBILITY_ONLY, APP_GAME_PLATFORM_GAP_LINUX_FOREGROUND_CAPTURE};
use ocentra_parent_screen_capture_adapter::linux_foreground_source::{LinuxActiveWindowObservation, LinuxForegroundSourcePreflight, LinuxSourceReadiness};

#[test]
fn service_runtime_has_no_capture_readiness_authority_without_preflight_owner() {
    let preflight = LinuxForegroundSourcePreflight::unavailable();
    assert_eq!(preflight.source_ready(), LinuxSourceReadiness::Unavailable);
    assert_eq!(preflight.active_window_observed(), LinuxActiveWindowObservation::NotObserved);
    let generated = GeneratedAtText("test-generated-at".to_owned());
    let model = app_game_platform_proof_status_read_model_with_linux_preflight(generated, preflight);
    let row = model.rows.iter().find(|row| row.platform == APP_GAME_PARENT_PLATFORM_LINUX).expect("Linux row");
    assert_eq!(row.authority_state, APP_GAME_PLATFORM_AUTHORITY_VISIBILITY_ONLY);
    assert!(row.open_gaps.iter().any(|gap| gap == APP_GAME_PLATFORM_GAP_LINUX_FOREGROUND_CAPTURE));
    assert!(!row.adapter_dispatch_claimed && !row.platform_enforcement_claimed && !row.provider_delivery_claimed && !row.child_device_delivery_claimed && !row.private_diagnostics_claimed);
    assert!(!model.adapter_dispatch_claimed && !model.platform_enforcement_claimed && !model.provider_delivery_claimed && !model.child_device_delivery_claimed && !model.private_diagnostics_claimed);
}
