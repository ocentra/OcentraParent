use ocentra_parent_agent_protocol::app_game_platform_proof_status::APP_GAME_LINUX_DOCKER_PREFLIGHT_PROBE_UNAVAILABLE;

use super::app_game_linux_docker_host_preflight::{
    detect_linux_docker_host_preflight, unavailable_linux_docker_host_preflight,
};
use super::app_game_linux_docker_host_preflight_cleanup_owner::CleanupWorkerRegistry;

#[test]
fn mark_degraded_is_sticky_and_shared_with_clones() {
    let registry = CleanupWorkerRegistry::new();
    let clone = registry.clone();

    assert!(!registry.is_degraded());
    assert!(!clone.is_degraded());

    registry.mark_degraded();

    assert!(registry.is_degraded());
    assert!(clone.is_degraded());

    clone.mark_degraded();

    assert!(registry.is_degraded());
    assert!(clone.is_degraded());
}

#[test]
fn degraded_registry_makes_detection_fail_before_any_probe() {
    let registry = CleanupWorkerRegistry::new();
    registry.mark_degraded();

    let detected = detect_linux_docker_host_preflight(registry);

    assert_eq!(detected, unavailable_linux_docker_host_preflight());
    assert_eq!(
        detected.state,
        APP_GAME_LINUX_DOCKER_PREFLIGHT_PROBE_UNAVAILABLE
    );
}
