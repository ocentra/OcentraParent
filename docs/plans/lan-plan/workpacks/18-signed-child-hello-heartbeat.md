# 18 Signed Child Hello And Heartbeat

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `18 Signed Child Hello And Heartbeat`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [20-step plan](../v0-9-lan-discovery-20-step-plan.md),
[test blueprint](../v0-9-lan-discovery-test-blueprint.md),
[UI/UX guide](../ui-ux-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

Current V0.9 proof includes local/LAN route and command validation, but physical
household readiness still lacks a signed LAN child hello and heartbeat from a
second child-agent device.

Branch `codex/v0-9-lan-signed-discovery-relay-spine` adds signed discovery
spine rows and rejection states for the branch proof harness. That is a
verified protocol/service proof layer for signed discovery readiness, but it
does not replace a real paired child-agent hello and heartbeat from a second
physical device.

## Where We Want To Be

After pairing, the child agent connects outward to the parent when possible and
sends a signed hello with protocol version, device id, install id, family hash,
optional child profile hash, platform, hostname, agent version, local IPs, MACs
where available, capabilities, timestamp, nonce, and signature. Heartbeats
update last confirmed, online, stale, and offline state.

## Requirement Checklist

- [x] Current branch records signed-discovery proof rows and rejection outcomes
      for invalid, revoked, wrong-target, unavailable, and manual-required
      states in typed contracts and Rust protocol/service parity.
- [x] Current branch keeps physical second-device household proof
      manual-required instead of claiming CI can prove it.
- [x] Portal LAN selected-device details and Activity/Network diagnostics now
      show signed-hello/signed-heartbeat manual-required labels from the typed
      read model.
- [x] Reject unsigned, invalid-signature, expired, replayed, wrong-family,
      wrong-device, unpaired, and wrong-protocol hello payloads.
- [x] Accept unknown future capability only when the contract says it is safe.
- [x] Record child-agent capabilities without mixing in browser/app/screen-time
      control claims.
- [x] Keep Android and iOS MAC/background limits explicit and manual-required
      until real device proof exists.
- [x] Transition online to stale to offline without deleting the device record.

## Acceptance And Proof

- Child hello tests cover valid signed hello, missing device id, missing nonce,
  invalid signature, wrong family, expired timestamp, replay, unknown future
  version, and unknown capability.
- Heartbeat explicit-timestamp tests cover valid heartbeat, wrong signature, timeout,
  late heartbeat recovery, stale, and offline transitions.
- Manual proof captures second physical child-agent hello and heartbeat before
  production household LAN readiness is claimed.

## Parallel Ownership Notes

This is security-sensitive. Pairing keys, route checks, nonce stores, and
heartbeat state must stay shared with assignment/revocation work.

## Local Rust/Core Status

As of 2026-06-28, the scoped Rust/core slice is locally validated and recorded
in `output/lan-plan-proof/18-signed-child-hello-heartbeat/01-local-validation.md`.

- `crates/lan-core/src/lan_pairing.rs` rejects unsigned or malformed signature
  fields, invalid signatures, expired timestamps, replayed nonces, wrong
  family, wrong route/parent/child bindings, and fail-closed schema/message
  drift. The current Rust contract does not expose a separate
  `protocolVersion` field, so local "wrong-protocol" proof is the existing
  fail-closed schema/message-kind boundary rather than a second version field.
- `crates/lan-core/tests/unit/lan_flow.rs` proves valid signed hello/heartbeat,
  negative verifier cases, future-safe capability passthrough, and no-churn
  capability recording in the LAN core slice.
- `crates/lan-core/tests/unit/read_model.rs` proves the signed-discovery
  read-model spine stays visible across stale/offline/manual-required states
  without deleting the durable device record.
- `crates/agent-service/tests/unit/lan_pairing.rs` now proves the runtime
  boundary accepts real signed hello/heartbeat envelopes, reports
  manual-required signed-child status, rejects missing parent/child signed
  context, and keeps stale/offline selected-device state explicit.
- `crates/agent-service/tests/unit/lan_pairing/device_roles.rs` now proves the
  default child-mobile runtime surfaces stay scaffold/manual-required on
  Android and iOS instead of inventing localhost-ready route state before real
  device proof exists.
- This packet reran green:
  `cargo test -p ocentra-lan-core lan_flow -- --nocapture`,
  `cargo test -p ocentra-lan-core read_model -- --nocapture`,
  `cargo test -p ocentra-parent-agent-service lan_pairing -- --nocapture`,
  `cargo test -p ocentra-parent-agent-protocol lan_pairing -- --nocapture`,
  and `cargo lint-architecture` for the owned Rust files.
- Remaining open rows are still real and unchanged: explicit physical Android
  and iOS device evidence plus second physical child-device hello/heartbeat
  proof are not claimed by this local Rust/core pass.
