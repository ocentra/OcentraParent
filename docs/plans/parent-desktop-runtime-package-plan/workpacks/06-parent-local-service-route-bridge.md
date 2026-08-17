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
- Status: active. The refreshed Rust-owned route/health production source is independently accepted; expected-test refresh, focused execution, and proof revalidation remain open.
- Production reachability audit: the shipped Tauri command now checks the configured local agent through the existing correlated, schema/peer-validated typed `AgentHealthCheck` / `AgentHealthReported` WebSocket path before reporting connected/readiness; the reported timeout uses the same parent-runtime-core command timeout, and route read-model loaders/action dispatch already use that same service-owned transport. The retained raw TCP helper is compatibility/test support only. This source correction does not re-affirm prior validation or proof in the current production-code phase.
- Focused validation: `cargo test -p ocentra-parent-runtime-core --test unit parent_ui_bridge::snapshot_and_dispatch_tests::devices_route`, `cargo test -p ocentra-parent-runtime-core --test unit parent_ui_bridge::snapshot_and_dispatch_tests::start_route_snapshot_attaches_setup_first_run_panel`, and `cargo test -p ocentra-schema --test contract parent_ui_bridge` all passed.
- No-claim boundary: this workpack does not claim setup readiness, child runtime distribution ownership, or desktop/mobile/web package readiness.

## 2026-08-17 source-wave truth

All service command responses now use protocol-owned response-kind and identity
binding instead of accepting the first plausible text event. Required route
dependency and LAN-query failures are aggregated into typed unavailable or
degraded snapshots and rejected actions, so a post-health failure cannot retain
stale `ready` or `connected` state. Tauri and dev-web polling consume the same
typed health contract with bounded/serialized polling.

This source acceptance does not revalidate the older focused tests or proof.
The later expected-test wave must cover wrong event kind/id/nonce/digest/peer,
stale response time, fragmented-frame deadline, post-health dependency failure,
LAN-query failure, and no-stale-connected behavior before closure.
