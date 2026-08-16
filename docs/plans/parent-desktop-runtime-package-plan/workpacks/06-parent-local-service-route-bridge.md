# Workpack 06 - Parent Local-Service Route Bridge

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-client-runtime-distribution-plan`
> Workpack: `06-parent-local-service-route-bridge`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: prove the typed route bridge between parent client launch/readiness and local-service state.

## Must prove

- bridge inputs and outputs use the canonical contract shape
- route state stays separate from setup ownership
- service reachability and degradation are explicit
- the bridge does not absorb child runtime distribution claims

## Failure conditions

- bridge state becomes a setup completion claim
- route bridge and package claims are merged
- missing service state is reported as healthy

## Completion

- Proof root: `output/parent-client-runtime-distribution-plan-proof/06-parent-local-service-route-bridge/`
- Status: complete. The Rust-owned route snapshot contract, Devices-route local-service boundary, and checklist/index/state truth are aligned.
- Production reachability audit: the shipped Tauri command now checks the configured local agent through the existing typed `AgentHealthCheck` / `AgentHealthReported` WebSocket path before reporting connected/readiness; route read-model loaders and action dispatch already use that same service-owned transport. This source correction does not re-affirm prior validation or proof in the current production-code phase.
- Focused validation: `cargo test -p ocentra-parent-runtime-core --test unit parent_ui_bridge::snapshot_and_dispatch_tests::devices_route`, `cargo test -p ocentra-parent-runtime-core --test unit parent_ui_bridge::snapshot_and_dispatch_tests::start_route_snapshot_attaches_setup_first_run_panel`, and `cargo test -p ocentra-schema --test contract parent_ui_bridge` all passed.
- No-claim boundary: this workpack does not claim setup readiness, child runtime distribution ownership, or desktop/mobile/web package readiness.
