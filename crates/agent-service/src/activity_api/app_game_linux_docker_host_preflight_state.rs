use ocentra_parent_agent_protocol::{
    app_game::APP_GAME_SCHEMA_VERSION,
    app_game_platform_proof_status::{
        APP_GAME_LINUX_DOCKER_PREFLIGHT_DAEMON_UNAVAILABLE,
        APP_GAME_LINUX_DOCKER_PREFLIGHT_PARTIAL, APP_GAME_LINUX_DOCKER_PREFLIGHT_PROBE_UNAVAILABLE,
        APP_GAME_LINUX_DOCKER_PREFLIGHT_READY, APP_GAME_PLATFORM_GAP_LINUX_DOCKER_CLI,
        APP_GAME_PLATFORM_GAP_LINUX_DOCKER_CONTEXT_INVENTORY,
        APP_GAME_PLATFORM_GAP_LINUX_DOCKER_DAEMON,
        APP_GAME_PLATFORM_GAP_LINUX_DOCKER_OBJECT_INVENTORY, AppGameLinuxDockerHostPreflight,
    },
    constants::v08_supported_adapter_runtime_proof as proof,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct DockerPreflightState(&'static str);

impl DockerPreflightState {
    pub(super) const READY: Self = Self(APP_GAME_LINUX_DOCKER_PREFLIGHT_READY);
    pub(super) const PARTIAL: Self = Self(APP_GAME_LINUX_DOCKER_PREFLIGHT_PARTIAL);
    pub(super) const DAEMON_UNAVAILABLE: Self =
        Self(APP_GAME_LINUX_DOCKER_PREFLIGHT_DAEMON_UNAVAILABLE);
    pub(super) const PROBE_UNAVAILABLE: Self =
        Self(APP_GAME_LINUX_DOCKER_PREFLIGHT_PROBE_UNAVAILABLE);
}

pub(super) fn build_preflight(
    state: DockerPreflightState,
    cli_visible: bool,
    daemon_visible: bool,
    context_count: Option<u64>,
    inventory: Option<(u64, u64)>,
) -> AppGameLinuxDockerHostPreflight {
    let ready = state == DockerPreflightState::READY;
    let mut open_gaps = Vec::new();
    if !cli_visible {
        open_gaps.push(APP_GAME_PLATFORM_GAP_LINUX_DOCKER_CLI.to_string());
    }
    if !daemon_visible {
        open_gaps.push(APP_GAME_PLATFORM_GAP_LINUX_DOCKER_DAEMON.to_string());
    }
    if context_count.is_none() {
        open_gaps.push(APP_GAME_PLATFORM_GAP_LINUX_DOCKER_CONTEXT_INVENTORY.to_string());
    }
    if inventory.is_none() {
        open_gaps.push(APP_GAME_PLATFORM_GAP_LINUX_DOCKER_OBJECT_INVENTORY.to_string());
    }
    let inventory_visible = inventory.is_some();
    let (image_count, container_count) = inventory.unwrap_or_default();

    AppGameLinuxDockerHostPreflight {
        schema_version: APP_GAME_SCHEMA_VERSION,
        state: state.0.to_string(),
        cli_visible,
        daemon_visible,
        context_inventory_visible: context_count.is_some(),
        context_count: context_count.unwrap_or_default(),
        image_inventory_visible: inventory_visible,
        image_count,
        container_inventory_visible: inventory_visible,
        container_count,
        identifiers_redacted: true,
        proof_refs: ready
            .then_some(proof::REF_LINUX_DOCKER_HOST_PREFLIGHT.to_string())
            .into_iter()
            .collect(),
        open_gaps,
        adapter_dispatch_claimed: false,
        platform_enforcement_claimed: false,
        provider_delivery_claimed: false,
        child_device_delivery_claimed: false,
        private_diagnostics_claimed: false,
    }
}
