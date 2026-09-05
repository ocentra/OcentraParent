use ocentra_parent_agent_protocol::app_game_authority_classifier::APP_GAME_PARENT_PLATFORM_LINUX;
use ocentra_parent_agent_protocol::app_game_platform_proof_status::{
    APP_GAME_LINUX_DOCKER_PREFLIGHT_READY, APP_GAME_PLATFORM_GAP_CHILD_DELIVERY,
    APP_GAME_PLATFORM_GAP_LINUX_FOREGROUND_CAPTURE, APP_GAME_PLATFORM_GAP_LINUX_NATIVE_SERVICE,
    APP_GAME_PLATFORM_GAP_LINUX_ROLLBACK, APP_GAME_PLATFORM_GAP_PLATFORM_ENFORCEMENT,
};
use ocentra_parent_agent_protocol::constants::value::APP_GAME_TEST_PLATFORM_PROOF_STATUS_GENERATED_AT;
use ocentra_parent_screen_capture_adapter::linux_foreground_source::LinuxForegroundSourcePreflight;

use super::app_game_adapter_host_capabilities::HostCapabilitySignals;
use super::app_game_linux_docker_host_preflight_state::{build_preflight, DockerPreflightState};
use super::app_game_platform_proof_status_payload::app_game_platform_proof_status_read_model_from_preflights;
use super::app_game_platform_proof_status_payload::PlatformProofGeneratedAtText;
use crate::test_require_some::require_some;

#[test]
fn ready_docker_preflight_route_carries_counts_and_redaction_only() {
    let host_capabilities = HostCapabilitySignals {
        android_adb: false,
        android_adb_path: false,
        android_adb_sdk: false,
    };
    let linux_foreground_preflight = LinuxForegroundSourcePreflight::unavailable();
    let docker_preflight = build_preflight(
        DockerPreflightState::READY,
        true,
        true,
        Some(2),
        Some((3, 1)),
    );

    let read_model = app_game_platform_proof_status_read_model_from_preflights(
        PlatformProofGeneratedAtText(APP_GAME_TEST_PLATFORM_PROOF_STATUS_GENERATED_AT.to_string()),
        &host_capabilities,
        &linux_foreground_preflight,
        &docker_preflight,
    );

    assert_eq!(
        (
            read_model.adapter_dispatch_claimed,
            read_model.broad_installed_app_blocking_claimed,
            read_model.platform_enforcement_claimed,
            read_model.provider_delivery_claimed,
            read_model.child_device_delivery_claimed,
            read_model.private_diagnostics_claimed,
        ),
        (false, false, false, false, false, false)
    );

    let linux_row = require_some(
        read_model
            .rows
            .iter()
            .find(|row| row.platform == APP_GAME_PARENT_PLATFORM_LINUX),
        "the platform proof route must include a Linux row",
    );
    let docker = require_some(
        linux_row.linux_docker_host_preflight.as_ref(),
        "the Linux row must carry the injected Docker preflight",
    );

    assert_eq!(docker.state, APP_GAME_LINUX_DOCKER_PREFLIGHT_READY);
    assert!(docker.cli_visible);
    assert!(docker.daemon_visible);
    assert_eq!(
        (
            docker.context_inventory_visible,
            docker.context_count,
            docker.image_inventory_visible,
            docker.image_count,
            docker.container_inventory_visible,
            docker.container_count,
        ),
        (true, 2, true, 3, true, 1)
    );
    assert!(docker.identifiers_redacted);
    assert_eq!(docker.proof_refs, Vec::<String>::new());
    assert_eq!(docker.open_gaps, Vec::<String>::new());
    assert_eq!(
        (
            docker.adapter_dispatch_claimed,
            docker.platform_enforcement_claimed,
            docker.provider_delivery_claimed,
            docker.child_device_delivery_claimed,
            docker.private_diagnostics_claimed,
        ),
        (false, false, false, false, false)
    );

    assert_eq!(
        linux_row.open_gaps,
        vec![
            APP_GAME_PLATFORM_GAP_LINUX_NATIVE_SERVICE.to_string(),
            APP_GAME_PLATFORM_GAP_LINUX_FOREGROUND_CAPTURE.to_string(),
            APP_GAME_PLATFORM_GAP_LINUX_ROLLBACK.to_string(),
            APP_GAME_PLATFORM_GAP_PLATFORM_ENFORCEMENT.to_string(),
            APP_GAME_PLATFORM_GAP_CHILD_DELIVERY.to_string(),
        ]
    );
    assert_eq!(
        (
            linux_row.adapter_dispatch_claimed,
            linux_row.broad_installed_app_blocking_claimed,
            linux_row.platform_enforcement_claimed,
            linux_row.provider_delivery_claimed,
            linux_row.child_device_delivery_claimed,
            linux_row.private_diagnostics_claimed,
        ),
        (false, false, false, false, false, false)
    );
}
