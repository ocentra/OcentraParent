# WP197 App/game Linux Docker host preflight

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP197 App/game Linux Docker host preflight`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

Represent Docker host visibility as parent-safe Linux platform evidence in the
shared app/game platform proof spine.

This workpack records whether the local Windows host can see Docker CLI,
daemon, context, image, and container inventory boundaries, but it stores only
counts and readiness states. It does not store Docker context names, image
names, container ids, raw paths, or private daemon diagnostics.

## Non-Goals

- No Docker container policy execution.
- No Linux AppArmor, SELinux, package, Flatpak, Snap, or container restriction
  proof.
- No launch blocking, rollback, audit, adapter dispatch, platform enforcement,
  provider delivery, or child-device delivery claim.
- No raw Docker context, image, or container identifier custody.

## Files

- `crates/agent-protocol/src/app_game_platform_proof_status.rs`
- `crates/agent-protocol/src/constants/v08_supported_adapter_runtime_proof.rs`
- `crates/agent-service/src/activity_api/app_game_linux_docker_host_preflight.rs`
- `crates/agent-service/src/activity_api/app_game_linux_docker_host_preflight_output.rs`
- `crates/agent-service/src/activity_api/app_game_linux_docker_host_preflight_process.rs`
- `crates/agent-service/src/activity_api/app_game_linux_docker_host_preflight_state.rs`
- `crates/agent-service/src/activity_api/app_game_linux_docker_host_preflight_wait.rs`
- `crates/agent-service/src/activity_api/app_game_platform_proof_status_payload.rs`
- `crates/parent-runtime-core/src/parent_ui_bridge/app_game_platform.rs`

Expected test roots for the later test-writing wave:

- `crates/agent-protocol/tests/contract/app_game_platform_proof_status_tests.rs`
- `crates/agent-service/tests/unit/app_game_linux_docker_host_preflight.rs`
- `crates/agent-service/tests/unit/app_game_platform_proof_status_payload_tests.rs`
- `crates/parent-runtime-core/tests/integration/parent_ui_bridge.rs`

## Validation

- Focused protocol compatibility and no-claim contract tests.
- Focused Docker process timeout, bounded-output, malformed-output, partial,
  unavailable, and ready-state tests using a real child-process fixture rather
  than an in-memory adapter.
- Focused service payload and parent-rendering tests proving count-only custody,
  no raw identifiers/diagnostics, and explicit false execution/delivery claims.
- Focused Rust formatting, library checks, architecture, source-shape,
  no-test-doubles, validation-bypass, and coordination guards.

## Current source state - 2026-08-24

Production source is drafted in the Rust protocol, agent-service, and
parent-runtime owners. The service resolves an exact Docker executable without
a shell, probes CLI/daemon/context/image/container visibility on a bounded
blocking worker, caps output and inventory counts, kills timed-out processes,
discards stderr, and emits only readiness/count state. Parent rendering exposes
the redacted state without raw paths, context names, image names, container
identifiers, or private diagnostics. Protocol rows remain backward compatible,
and all adapter/enforcement/delivery claims remain false.

The three affected production libraries compile together. Expected tests have
not been written or run, existing struct-literal tests require the later
protocol-field refresh, and no retained proof, pre-commit, CI, PR, READY, or
DONE claim is made.

## Done Criteria

- Docker CLI/daemon/context/image/container visibility is represented as typed
  parent-domain readiness.
- Context, image, and container details are redacted to counts only.
- Linux platform proof status can carry `linux-docker-host-preflight-ref`.
- Container policy, platform enforcement, adapter dispatch, and child delivery
  remain false.
