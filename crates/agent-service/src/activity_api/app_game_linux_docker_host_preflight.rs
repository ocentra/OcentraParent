use ocentra_parent_agent_protocol::{
    app_game_platform_proof_status::AppGameLinuxDockerHostPreflight,
    constants::v08_supported_adapter_runtime_proof as proof,
};

use std::time::Instant;

use super::{
    app_game_linux_docker_host_preflight_paths::resolve_trusted_docker_executable,
    app_game_linux_docker_host_preflight_process::{
        run_docker_probe, DockerProbeArguments, DockerProbeOutput,
    },
    app_game_linux_docker_host_preflight_state::{build_preflight, DockerPreflightState},
    app_game_linux_docker_host_preflight_wait::DOCKER_PREFLIGHT_TIMEOUT,
};

const MAX_DOCKER_INVENTORY_COUNT: u64 = 10_000_000;

pub(super) fn detect_linux_docker_host_preflight() -> AppGameLinuxDockerHostPreflight {
    let deadline = Instant::now() + DOCKER_PREFLIGHT_TIMEOUT;
    let Some(executable) = resolve_trusted_docker_executable() else {
        return build_preflight(
            DockerPreflightState::PROBE_UNAVAILABLE,
            false,
            false,
            None,
            None,
        );
    };

    let daemon_visible = probe_has_nonempty_text(run_docker_probe(
        &executable,
        DockerProbeArguments(&proof::DOCKER_VERSION_ARGUMENTS),
        deadline,
    ));
    let context_count = parse_context_count(run_docker_probe(
        &executable,
        DockerProbeArguments(&proof::DOCKER_CONTEXT_ARGUMENTS),
        deadline,
    ));
    let inventory = daemon_visible
        .then(|| {
            parse_inventory_counts(run_docker_probe(
                &executable,
                DockerProbeArguments(&proof::DOCKER_INVENTORY_ARGUMENTS),
                deadline,
            ))
        })
        .flatten();
    let state = if !daemon_visible {
        DockerPreflightState::DAEMON_UNAVAILABLE
    } else if context_count.is_some() && inventory.is_some() {
        DockerPreflightState::READY
    } else {
        DockerPreflightState::PARTIAL
    };

    build_preflight(state, true, daemon_visible, context_count, inventory)
}

pub(super) fn unavailable_linux_docker_host_preflight() -> AppGameLinuxDockerHostPreflight {
    build_preflight(
        DockerPreflightState::PROBE_UNAVAILABLE,
        false,
        false,
        None,
        None,
    )
}

fn probe_has_nonempty_text(output: DockerProbeOutput) -> bool {
    output.success
        && std::str::from_utf8(&output.stdout).is_ok_and(|value| !value.trim().is_empty())
}

fn parse_context_count(output: DockerProbeOutput) -> Option<u64> {
    if !output.success {
        return None;
    }
    let value = std::str::from_utf8(&output.stdout).ok()?;
    let count = value.lines().try_fold(0_u64, |count, line| {
        (line == proof::DOCKER_CONTEXT_COUNT_MARKER && count < MAX_DOCKER_INVENTORY_COUNT)
            .then_some(count + 1)
    })?;
    (count > 0).then_some(count)
}

fn parse_inventory_counts(output: DockerProbeOutput) -> Option<(u64, u64)> {
    if !output.success {
        return None;
    }
    let value = std::str::from_utf8(&output.stdout).ok()?;
    let mut values = value.split_whitespace();
    let image_count = values.next()?.parse::<u64>().ok()?;
    let container_count = values.next()?.parse::<u64>().ok()?;
    (values.next().is_none()
        && image_count <= MAX_DOCKER_INVENTORY_COUNT
        && container_count <= MAX_DOCKER_INVENTORY_COUNT)
        .then_some((image_count, container_count))
}
