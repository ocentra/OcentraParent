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
- Status: blocked in the production-source phase. The bridge has useful typed response identity, freshness, deadline, and degradation checks, but the live WebSocket session is not authenticated and therefore is not accepted as a parent-authorized route.
- Production reachability audit: agent-service accepts a missing `Origin`, emits `AgentConnectionReady` before authenticating the peer, and exposes health as `Ready` while transport authentication is `Unauthenticated`. Parent runtime connects over `ws://` and supplies its own origin string. These caller-controlled or unauthenticated transport facts cannot establish parent authority even when later response envelopes are schema-, peer-, nonce-, correlation-, digest-, freshness-, and deadline-checked.
- First legal source dependency: Account currently routes session authority through WP03, which must provide an opaque, owner-issued, current, and revocable parent-local bridge session capability plus an owner-bound handshake. That provider-independent authority slice must be decomposed from WP03's downstream Cloudflare/Protected/Parent dependency cycle before a valid graph hard edge can be added. WP06 must then consume the capability before listener readiness, health readiness, route reads, or action dispatch. No WP06-only patch may infer authority from `Origin`, loopback, PID, same-user state, or typed response fields.
- Historical focused validation: `cargo test -p ocentra-parent-runtime-core --test unit parent_ui_bridge::snapshot_and_dispatch_tests::devices_route`, `cargo test -p ocentra-parent-runtime-core --test unit parent_ui_bridge::snapshot_and_dispatch_tests::start_route_snapshot_attaches_setup_first_run_panel`, and `cargo test -p ocentra-schema --test contract parent_ui_bridge` previously passed, but those tests do not cover authenticated transport admission and do not accept the current production source.
- No-claim boundary: this workpack does not claim setup readiness, child runtime distribution ownership, or desktop/mobile/web package readiness.

## 2026-08-17 source-wave truth

All service command responses use protocol-owned response-kind and structural
identity binding instead of accepting the first plausible text event. Required
route dependency and LAN-query failures are aggregated into typed unavailable
or degraded snapshots and rejected actions, so a post-health failure cannot
retain stale `ready` or `connected` state. Tauri and dev-web polling consume the
same typed health contract with bounded/serialized polling. These are positive
source properties, but they occur over an unauthenticated session and therefore
do not establish a trusted parent-service bridge.

After Account WP03 supplies the owner capability and WP06 composes authenticated
admission, the expected-test wave must cover missing/forged/revoked/stale
capability, missing/forged origin, readiness-before-authentication, wrong event
kind/id/nonce/digest/peer, stale response time, fragmented-frame deadline,
post-health dependency failure, LAN-query failure, and no-stale-connected
behavior before focused execution or proof.
