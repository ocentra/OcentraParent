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

The old `packages/parent-domain` list is historical; the accepted source owner
is Rust and is integrated through `5bfb2f6f3` from source packet `23c08da016`.
The production surface is `crates/agent-protocol/src/app_game_platform_proof_status.rs`,
`crates/agent-protocol/src/constants/v08_supported_adapter_runtime_proof.rs`,
and `crates/agent-service/src/activity_api/` (the preflight, cleanup, process,
path, output, state, supervisor, and wait modules), with service-runtime and
websocket report/admission wiring.

The following expected focused test roots are not present and must be written
as one test wave before execution:

- `crates/agent-service/tests/unit/app_game_linux_docker_host_preflight.rs`
- `crates/agent-service/tests/unit/app_game_linux_docker_host_preflight_parser_tests.rs`
- `crates/agent-service/tests/unit/app_game_linux_docker_host_preflight_path_security_tests.rs`
- `crates/agent-service/tests/unit/app_game_linux_docker_host_preflight_cleanup_tests.rs`
- `crates/agent-service/tests/unit/app_game_platform_probe_cache_tests.rs`
- `crates/agent-service/tests/unit/app_game_platform_proof_status_route_rejection_tests.rs`

## Validation

Focused validation is deferred until the six expected Rust test roots exist;
then select the smallest `cargo test` targets for `agent-service` and
`agent-protocol`, plus architecture and Enforcer checks for the touched source.
No test, proof, pre-commit, CI, or PR result is claimed by this source
checkpoint.

## Done Criteria

- Docker CLI/daemon/context/image/container visibility is represented as typed
  parent-domain readiness.
- Context, image, and container details are redacted to counts only.
- Linux platform proof status can carry `linux-docker-host-preflight-ref`.
- Container policy, platform enforcement, adapter dispatch, and child delivery
  remain false.
