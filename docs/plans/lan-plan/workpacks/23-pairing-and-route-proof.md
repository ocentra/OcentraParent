# 23 Pairing And Route Proof

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `23 Pairing And Route Proof`
> Kind: assigned active workpack; read only when this exact workpack is selected.
> Read when: Only when this exact workpack is explicitly selected from `WORKPACK_INDEX.md`.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack's own proof rows and tests support the claim.
> Proves: only this workpack's current route-proof boundary and progress explicitly recorded here.
> Does not prove: current completion of sibling workpacks or broad LAN readiness.
> Proof rule: Rewrite or discard any stale historical assumptions before using this file for execution claims.

<!-- /agent-capsule -->

Sources: [folder README](../README.md), [feature doc](../../../features/family-setup-device-roles.md),
[LAN pairing expectations](../../../expectations/lan-pairing.md).
Assumes the earlier Rust-owned contract and state workpacks exist before this
proof slice claims parity.

## Active scope status

This workpack is part of the authoritative `01-25` LAN execution model. It is
active and still open.

Historical TS-first household notes, portal-owned recovery assumptions, and
exact implementation file recipes from older copies of this draft are stale.
Current direction for this workpack is:

- Rust owns pairing, route-custody, recovery, revocation, audit, stale/offline,
  and rejection logic.
- Rust-owned shared schema/protocol boundaries own public command, event,
  rejection, and read-model shapes.
- Rust-owned runtime/read-model proof remains authoritative.
- TS may render Rust-backed state and dispatch typed bridge commands only. TS
  does not own route semantics, recovery state, audit truth, or proof closure.

## Where We Are

The current LAN spine already exposes Rust-owned selected-route, stale/offline,
revoked, signed-discovery, and audit-oriented state across the LAN read model.

Current verified local proof on 2026-06-28:

- `cargo test -p ocentra-parent-agent-service lan_pairing -- --nocapture` is
  green for the current Rust-owned pairing/runtime surface, including
  selected-route restart recovery, revoke-before-control, stale/offline status,
  replay or expiry rejection, wrong-origin or wrong-device rejection, selected
  route command dispatch, and audit continuity.
- `cargo test -p ocentra-parent-runtime-core
  lan_agent_command_requested_for_devices_route_forwards_signed_child_observe_payload_and_replay_fields -- --nocapture`,
  `cargo test -p ocentra-parent-runtime-core
  lan_scan_action_returns_bounded_error_when_response_times_out -- --nocapture`,
  and `cargo test -p ocentra-parent-runtime-core
  product_bridge_actions_return_route_snapshots_without_invented_overlay_data -- --nocapture`
  are green, so the parent runtime and `/devices` route consume Rust-backed
  route/rejection truth without UI-side invention.
- `cargo lint-architecture crates/agent-service/tests/unit/lan_pairing.rs
  crates/agent-service/tests/unit/lan_pairing_household_device_spine.rs
  crates/parent-runtime-core/tests/unit/parent_ui_bridge/snapshot_and_dispatch_tests.rs`
  is green for the touched route-proof test surfaces.

The remaining open work is no longer basic TS wiring. The remaining open proof
is:

- real two-device route/revoke/re-pair proof across a live household LAN
- router/firewall/manual topology proof where the real network path matters
- any broader replay/restart/event-stream artifact set beyond the currently
  green Rust service/runtime tests

## Where We Want To Be

This workpack should eventually prove that the Rust-owned LAN pairing/runtime
path:

1. applies revocation before any later control or recovery intent;
2. keeps stale/offline/recovery-required state visible without false reachability
   claims;
3. rejects wrong-origin, wrong-route, wrong-device, replayed, expired, and
   anonymous control paths with typed reasons;
4. preserves audit rows that identify actor, target, route, reason, and
   outcome; and
5. carries manual-required boundaries honestly when CI cannot prove the real
   network topology.

## Ownership boundary

- Rust shared schema/protocol crates own public pairing, route, rejection,
  recovery, and audit shapes.
- Rust service/runtime/read-model crates own lease checks, recovery transitions,
  stale/offline TTL evaluation, rejection handling, persistence, and proof
  generation.
- Supporting UI or browser artifacts may show those states, but they consume
  Rust-backed truth only and must not become the authority for pairing or route
  behavior.
- This workpack must not reintroduce TS business logic, TS-owned contracts,
  portal-owned rejection logic, or documentation that tells future agents to
  rebuild LAN ownership in presentation code.

## Scope

- Prove Rust-owned pairing, route-selection, recovery, revoke, and audit
  behavior against the current LAN runtime/read-model path.
- Keep this slice focused on route/recovery proof and state honesty.
- Use typed handoffs when household/setup UI consumes these states later.
- Do not turn this workpack into a portal implementation recipe or a household
  feature design packet.

## Tests And Proof

- Real tests must live in organized Rust test groups such as `tests/contract`,
  `tests/unit`, `tests/integration`, `tests/security`, or
  `tests/version-skew`, depending on the claimed risk.
- Route-proof closure must include real negative coverage for anonymous,
  wrong-origin, wrong-route, wrong-device, replayed, expired, revoked, stale,
  and offline states where applicable.
- Restart/rescan persistence proof must show that selected-route, trust, audit,
  and recovery-required state remain honest after reloads.
- Audit/read-model proof must show typed route status, typed rejection reason,
  and typed recovery or manual-required state without UI-side invention.
- Inline source-owned tests, placeholder directories, `.gitkeep` trees, fake
  coverage, mock-only readiness, or screenshot-only closure do not count.
- Supporting UI/browser artifacts may be attached only as presentation evidence
  for already-proved Rust states; they do not substitute for Rust-owned proof.
- Proof artifact: `output/lan-plan-proof/23-pairing-and-route-proof/`

Supporting presentation evidence already green for this Rust-backed route truth:

- `$env:OCENTRA_PARENT_PORTAL_PLAYWRIGHT_SPEC='portal-ui.spec.ts'; node
  scripts/test/portal-playwright-runner.mjs`
  passes on the current Windows `/devices` path after the Rust-backed route
  snapshot reaches the portal.

## AI Worker Checklist

- [ ] Confirm WP23 is the assigned active workpack.
- [ ] Rewrite any stale TS-owned route/recovery wording still present in this
      file before code moves.
- [ ] Confirm the consumed command/event/read-model shapes are Rust-owned before
      runtime or proof work starts.
- [ ] Keep route-custody, recovery, rejection, and audit truth in Rust-owned
      runtime/read-model proof.
- [ ] Do not let portal/UI wording become the authority for route or recovery
      state.
- [ ] All claimed tests live in real organized Rust test folders/groups; no
      inline source-owned, placeholder, `.gitkeep`, fake, or mock-only test
      surfaces count.
- [ ] Manual-required physical topology or second-device gaps remain explicit.

## Manual-Required Gaps

Real two-device route/revoke/re-pair proof across a real household LAN remains
manual-required until an actual artifact set exists. Router/firewall behavior,
local permission constraints, and second-device signed proof remain outside CI
closure unless a later proof packet regenerates them honestly.
