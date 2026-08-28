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

use super::app_game_platform_probe_cache::PlatformProbeCache;

#[test]
fn initial_and_cloned_cache_snapshots_are_fully_unavailable() {
    let cache = PlatformProbeCache::new();
    let cloned_cache = cache.clone();

    let initial_snapshot = cache.snapshot();
    let cloned_snapshot = cloned_cache.snapshot();

    assert_eq!(initial_snapshot, cloned_snapshot);
    assert_unavailable_snapshot(&initial_snapshot);
    assert_unavailable_snapshot(&cloned_snapshot);
}

fn assert_unavailable_snapshot(snapshot: &AppGameLinuxDockerHostPreflight) {
    assert_eq!(snapshot.schema_version, APP_GAME_SCHEMA_VERSION);
    assert_eq!(
        snapshot.state,
        APP_GAME_LINUX_DOCKER_PREFLIGHT_PROBE_UNAVAILABLE
    );
    assert_eq!(snapshot.cli_visible, false);
    assert_eq!(snapshot.daemon_visible, false);
    assert_eq!(snapshot.context_inventory_visible, false);
    assert_eq!(snapshot.context_count, 0);
    assert_eq!(snapshot.image_inventory_visible, false);
    assert_eq!(snapshot.image_count, 0);
    assert_eq!(snapshot.container_inventory_visible, false);
    assert_eq!(snapshot.container_count, 0);
    assert_eq!(snapshot.identifiers_redacted, true);
    assert_eq!(snapshot.proof_refs, Vec::<String>::new());
    assert_eq!(
        snapshot.open_gaps,
        vec![
            APP_GAME_PLATFORM_GAP_LINUX_DOCKER_CLI.to_string(),
            APP_GAME_PLATFORM_GAP_LINUX_DOCKER_DAEMON.to_string(),
            APP_GAME_PLATFORM_GAP_LINUX_DOCKER_CONTEXT_INVENTORY.to_string(),
            APP_GAME_PLATFORM_GAP_LINUX_DOCKER_OBJECT_INVENTORY.to_string(),
        ]
    );
    assert_eq!(
        (
            snapshot.adapter_dispatch_claimed,
            snapshot.platform_enforcement_claimed,
            snapshot.provider_delivery_claimed,
            snapshot.child_device_delivery_claimed,
            snapshot.private_diagnostics_claimed,
        ),
        (false, false, false, false, false)
    );
}
