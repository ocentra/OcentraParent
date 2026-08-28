use ocentra_parent_agent_protocol::{
    app_game::APP_GAME_SCHEMA_VERSION,
    app_game_platform_proof_status::{
        AppGameLinuxDockerHostPreflight, APP_GAME_LINUX_DOCKER_PREFLIGHT_PROBE_UNAVAILABLE,
        APP_GAME_PLATFORM_GAP_LINUX_DOCKER_CLI,
        APP_GAME_PLATFORM_GAP_LINUX_DOCKER_CONTEXT_INVENTORY,
        APP_GAME_PLATFORM_GAP_LINUX_DOCKER_DAEMON,
        APP_GAME_PLATFORM_GAP_LINUX_DOCKER_OBJECT_INVENTORY,
    },
};

use super::app_game_linux_docker_host_preflight::{
    detect_linux_docker_host_preflight, unavailable_linux_docker_host_preflight,
};
use super::app_game_linux_docker_host_preflight_cleanup_owner::CleanupWorkerRegistry;

#[test]
fn unavailable_preflight_is_fully_fail_closed() {
    let preflight = unavailable_linux_docker_host_preflight();

    assert_fully_probe_unavailable(&preflight);
}

#[test]
fn degraded_cleanup_registry_forces_probe_unavailable_before_docker_access() {
    let cleanup_workers = CleanupWorkerRegistry::new();
    cleanup_workers.mark_degraded();

    let preflight = detect_linux_docker_host_preflight(cleanup_workers);

    assert_fully_probe_unavailable(&preflight);
}

fn assert_fully_probe_unavailable(preflight: &AppGameLinuxDockerHostPreflight) {
    assert_eq!(preflight.schema_version, APP_GAME_SCHEMA_VERSION);
    assert_eq!(
        preflight.state,
        APP_GAME_LINUX_DOCKER_PREFLIGHT_PROBE_UNAVAILABLE
    );
    assert_eq!(preflight.cli_visible, false);
    assert_eq!(preflight.daemon_visible, false);
    assert_eq!(preflight.context_inventory_visible, false);
    assert_eq!(preflight.context_count, 0);
    assert_eq!(preflight.image_inventory_visible, false);
    assert_eq!(preflight.image_count, 0);
    assert_eq!(preflight.container_inventory_visible, false);
    assert_eq!(preflight.container_count, 0);
    assert_eq!(preflight.identifiers_redacted, true);
    assert_eq!(preflight.proof_refs, Vec::<String>::new());
    assert_eq!(
        preflight.open_gaps,
        vec![
            APP_GAME_PLATFORM_GAP_LINUX_DOCKER_CLI.to_string(),
            APP_GAME_PLATFORM_GAP_LINUX_DOCKER_DAEMON.to_string(),
            APP_GAME_PLATFORM_GAP_LINUX_DOCKER_CONTEXT_INVENTORY.to_string(),
            APP_GAME_PLATFORM_GAP_LINUX_DOCKER_OBJECT_INVENTORY.to_string(),
        ]
    );
    assert_eq!(
        (
            preflight.adapter_dispatch_claimed,
            preflight.platform_enforcement_claimed,
            preflight.provider_delivery_claimed,
            preflight.child_device_delivery_claimed,
            preflight.private_diagnostics_claimed,
        ),
        (false, false, false, false, false)
    );
}
