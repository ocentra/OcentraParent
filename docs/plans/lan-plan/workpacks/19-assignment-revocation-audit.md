# 19 Assignment, Revocation, And Audit

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `19 Assignment, Revocation, And Audit`
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

Current proof includes trusted registry, route-control direction, restart
readback, and rescan behavior, but the full parent assignment, rename, trust,
ignore, revocation, and audit behavior is still not product complete across
physical two-device and manual-artifact flows.

Branch `codex/v0-9-lan-signed-discovery-relay-spine` strengthens the typed
registry, route-custody, revoked-route, relay/cache unavailable, and rejected
decision states. Parent-facing read-model consumption now shows parent
  decision/audit/route-custody state and sends the existing add-device request
  command for controllable LAN slots. First-class rename, trust, ignore,
  restore, and revoke controls now reuse those existing LAN command surfaces,
  so the remaining gap is proof breadth rather than missing Rust or portal
  wiring.

Current B-lane follow-up adds the missing neighbor-only rename/type routing
case: canonical household decisions without a child-agent id now route through
the local-network LAN service path, return add-device reported events, and
survive portal refresh. Current local Rust validation proves selected-route
restart recovery, route-select control gating, rename evidence, ignore/restore
state changes, revoke audit evidence, manual-required provider-selection
fallbacks, and explicit contract rejection and route-selection shapes. Main
lane has now rerun portal LAN-target routing with the route-context bridge fix:
selected-route LAN commands reach the real local-network child target in Rust
tests and the refreshed source-matrix/browser proof. The installer/PIN
handshake and the remaining assignment/rescan/weak-evidence contradiction proof
  remain open.

## Where We Want To Be

Parent decisions are durable and explicit. Parent assignment links a device to a
child only through manual parent action or signed child-agent confirmation.
Rename, trust, ignore, revocation, stale, offline, and route rejection are
audited and survive restart.

## Requirement Checklist

- [x] Current branch proves route-custody and trusted-registry safety states for
      accepted and rejected signed discovery/relay decisions.
- [x] Current branch models revoked, unavailable, manual-required, and
      wrong-target route states without allowing weak LAN evidence to become
      authority.
- [x] Portal selected-device details and Activity/Network diagnostics now show
      parent decision/audit rows, route custody, route rejection state, and
      relay/cache custody from the typed LAN read model.
- [x] Portal add-to-parent flow now calls the existing
      `agent.lan-pairing.add-device.request` command only for selected LAN
      slots with a controllable route.
- [x] Expose first-class portal controls for add, route select, rename, trust,
      ignore, restore, and revoke by reusing existing LAN command surfaces:
      `agent.lan-pairing.add-device.request` for household decisions plus route
      select/revoke commands for route custody. Portal command routing now sends
      LAN commands to the selected local-network child target.
- [x] Portal household rename/type for a LAN-discovered neighbor uses canonical
      device identity, routes over `local-network` even without a child-agent id,
      receives `agent.lan-pairing.add-device.reported`, and survives portal
      refresh from service readback.
- [x] Reject anonymous, wrong-origin, wrong-route, wrong-device, replayed, and
      expired pairing/control requests.
- [x] Preserve assignment and rename through rescan and weak evidence changes.
  Rename/type now survives portal refresh through service readback, neighbor-only
  same-MAC rescan with changed IP or weaker label, and persistent restart
  readback of the selected child route from the real Rust browser-runtime
  snapshot path.
- [x] Apply revocation before any new rule, query, approval, or heartbeat
      authority is accepted.
- [x] Show selected-device and route status clearly in parent-visible state.
  Current `/devices`, Browser Settings, AI Runtime, Entitlements, Policy
  Network, Activity, and proof-panel LAN surfaces are green from real Rust
  snapshots under the rerun `portal-ui.spec.ts` path.

## Acceptance And Proof

- Service tests cover anonymous rejection, accepted pairing, wrong origin,
  wrong device, revoked device, service restart, safe unpaired fallback, and
  audited command rejection.
- Portal tests cover LAN slot metadata for parent decisions/proof fields and
  portal transport routing for LAN commands to local-network child targets.
  Focused browser proof covers editable rename/type input, refresh persistence,
  and Update/Capability gating for an unpaired LAN neighbor. Full first-run
  setup, recovery, and degraded UX tests remain tracked in the family setup
  feature gap.
- Focused browser proof also reran green for selected-route Trust, Ignore,
  Restore, and Revoke controls plus Browser Settings, AI Runtime,
  Entitlements, Policy Network, and Activity target persistence from the real
  Rust snapshot path.
- Focused Rust restart/readback proof now captures the actual canonical device
  id emitted by the live pre-restart scan model, then proves the selected
  route and renamed child device survive persistent service restart without a
  synthetic fallback id.
- Audit records include source, route, actor, target device, reason, and
  outcome.

Current proof:

- `output/lan-plan-proof/19-assignment-revocation-audit/01-local-validation.md`
- Focused Rust proof:
  `cargo test -p ocentra-parent-agent-service route_trust_state_reports_pairing_selected_target -- --nocapture`
- Focused Rust proof:
  `cargo test -p ocentra-parent-agent-service parent_ignore_and_restore_decisions_change_enrollment_state -- --nocapture`
- Focused Rust proof:
  `cargo test -p ocentra-parent-agent-service parent_rename_decision_updates_canonical_display_name_with_evidence -- --nocapture`
- Focused Rust proof:
  `cargo test -p ocentra-parent-agent-service lan_pairing_household_device_spine -- --nocapture`
- Focused Rust proof:
  `cargo test -p ocentra-parent-agent-service provider_selection_read_model -- --nocapture`
- Focused Rust proof:
  `cargo test -p ocentra-parent-agent-service lan_pairing_persistent_registry_recovers_selected_route_after_restart -- --nocapture`
- Focused Rust proof:
  `cargo test -p ocentra-parent-agent-service persistent_runtime_restores_selected_route_and_household_rename_into_read_model -- --nocapture`
- Focused Rust proof:
  `cargo test -p ocentra-parent-agent-service lan_pairing_browser_runtime -- --nocapture`
- Focused Rust proof:
  `cargo test -p ocentra-parent-agent-service lan_pairing_route_select_allows_selected_child_control -- --nocapture`
- Focused Rust proof:
  `cargo test -p ocentra-parent-agent-service lan_pairing_route_select_makes_multi_device_control_explicit -- --nocapture`
- Focused Rust proof:
  `cargo test -p ocentra-lan-core household_revoke_decision_records_audit_evidence_and_blocks_control -- --nocapture`
- Focused Rust proof:
  `cargo test -p ocentra-parent-agent-protocol lan_pairing -- --nocapture`

## Parallel Ownership Notes

This work crosses domain, service, and portal state. Keep portal actions typed
and route-checked; do not let UI commands bypass the child-agent authority.
