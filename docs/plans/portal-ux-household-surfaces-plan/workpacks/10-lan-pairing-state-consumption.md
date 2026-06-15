# 10 LAN Pairing State Consumption

<!-- agent-capsule -->

> Agent Capsule
> Plan: `portal-ux-household-surfaces-plan`
> Doc: `10 LAN Pairing State Consumption`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [20-step plan](../portal-ux-household-surfaces-20-step-plan.md),
[test blueprint](../portal-ux-household-surfaces-test-blueprint.md),
[UI/UX guide](../ui-ux-requirements-guide.md), and [folder README](../README.md).

## Where We Are

B owns the full LAN plan. C must consume those states without duplicating LAN
truth. For the current B lane, this means B owns the LAN-specific service-to-UI
wiring while C remains responsible for broad portal visual polish.

Current screenshots show the existing Devices/LAN surface can render the
service-backed add-device grid, local agent row, passive LAN rows,
router/infrastructure row, selected-device detail tabs, Activity/Network
network evidence fields, and Network policy family/per-device target controls.
Branch `codex/v0-9-lan-signed-discovery-relay-spine` adds more typed LAN state
for signed discovery, route custody, relay/cache unavailable, revoked/offline,
and manual-required proof. The unfinished work is making those states actionable
and inspectable for the parent.

## Where We Want To Be

LAN states appear coherently in Devices and related routes: discovered,
assigned, confirmed, trusted, ignored, stale, offline, router, infrastructure,
and manual-required.

## Requirement Checklist

- [ ] Use B/service read models for LAN state.
- [ ] Keep routers/infrastructure visible but non-enrollable.
- [ ] Keep signed discovery, relay/cache, revoked/offline, and
      manual-required proof states in B-owned typed contracts/service state
      rather than a portal-only truth.
- [ ] Show assignment/ignore/revocation state where available from
      `householdDeviceDecisions` and signed relay route-safety audit rows.
- [ ] Show manual-required physical proof labels.
- [ ] Test LAN fixture states without claiming real LAN proof.
- [ ] Wire the existing add-device parent action to
      `agent.lan-pairing.add-device.request` when a selected device has a
      controllable LAN route.
- [ ] Wire first-class parent controls for add, route select, rename, trust,
      ignore, restore, revoke, and route-custody decisions through existing LAN
      add-device decision fields plus route select/revoke commands. Portal
      transport routes those commands to the selected local-network child
      target.
- [ ] Render Activity/Network LAN diagnostics for service-backed selected
      route, evidence, signed proof, revocation/route rejection state,
      relay/cache unavailable, parent decisions, route requirements, audit
      checks, and manual/unproved proof state.
- [ ] Add richer Activity/Network diagnostics for scan/evidence first-seen and
      last-seen timing, evidence expiry, signed adapter proof state, and
      policy-target history.
- [ ] Add live browser proof screenshots. Captured on B lane ports in
      `output/playwright/lan-ux-proof/devices-lan-controls.png`,
      `output/playwright/lan-ux-proof/activity-network-diagnostics.png`, and
      `output/playwright/lan-ux-proof/policy-network-targets.png`.

## Acceptance And Proof

Portal LAN UI is a consumer of B-owned state and does not create a second source
of truth.

## Parallel Ownership Notes

B owns LAN implementation and proof. C owns presentation and ergonomics.
