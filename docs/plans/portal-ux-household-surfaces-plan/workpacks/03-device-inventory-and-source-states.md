# 03 Device Inventory And Source States

<!-- agent-capsule -->

> Agent Capsule
> Plan: `portal-ux-household-surfaces-plan`
> Doc: `03 Device Inventory And Source States`
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

Devices are visible in development form. LAN discovery and product-control states
are still being wired by other lanes.

## Where We Want To Be

Device inventory is the product center and shows role, source, evidence,
confidence, controllability, and next action without duplicate truths.

## Requirement Checklist

- [x] Distinguish local agent, LAN agent, passive neighbor, router, ignored,
      stale, offline, and revoked states.
- [x] Show controllable versus visible-only.
- [x] Show source/custody labels.
- [x] Show `Not reported` instead of inventing hardware details.
- [x] Test long hostnames and many-device layouts.

## Acceptance And Proof

Device UI reads service state and never treats passive LAN neighbors as policy
targets.

## Current Source-Truth Packet (2026-08-25)

The accepted production source is present at canonical `b6d8d12ead5965cea80625431fdc67e518702403`, including source commits `aa5f4579e` and `adba0c9c1` in `vendor/ocentra-parent-core-ui/AppPages/ParentPortal/activity-ui-intent.ts`. This packet records that source state only; it makes no production-source change.

- The Rust owner/read-model seam is `crates/agent-service/src/lan_pairing_browser_add_device_state.rs`, which supplies the typed LAN snapshot, trusted registry, and revoked-device identifiers. The shipped Portal caller path is `apps/portal/src/ParentPortalRoute.tsx` -> `apps/portal/src/vendor-parent-portal-surface.js` -> `vendor/ocentra-parent-core-ui/AppPages/ParentPortal/ParentPortalSvgSurface.tsx` -> `activity-ui-intent.ts`.
- `lanDeviceSlots` collects canonical, discovered, trusted, pairing, and selected sources in that order. The accepted `upsertLanDeviceSlot` behavior is absorbing for a matched slot: an incoming `revoked` state stays revoked, and a matching existing revoked slot cannot be overwritten by a later ready/online selected update. A genuine identity/slot change uses the new slot and does not inherit the prior revoked badge.
- The existing `apps/portal/tests/e2e/lan-command-center-proof.spec.ts`, `apps/portal/tests/e2e/portal-route-scaffold-lan.ts`, and `apps/portal/tests/live-activity` surfaces cover service-backed LAN/source-state presentation, but no focused test directly exercises these projection helpers for both revocation update directions and the identity/slot boundary. The accepted source commits changed no test or proof path.
- The expected proof root `output/portal-ux-household-surfaces-plan-proof/03-device-inventory-and-source-states/` is absent in this checkout. No test, proof, CI, or pre-commit command was run or recorded in this source-only packet.

The requirement boxes above remain checked as the intended device-inventory coverage; they are not completion evidence for this packet.

## Remaining Validation And Proof Gaps

- Add the focused projection coverage expected by this workpack: canonical online state followed by matched revoked state, incoming revoked state followed by pairing/selected ready state, and a true identity/slot change that must not inherit revocation. The assertions must retain source/custody and no-fake-data boundaries.
- Run the workpack-selected service-backed projection/route checks and record the required negative states in the named proof manifest; those checks were intentionally not run here.
- Physical/provider pairing, freshness, and device-authority validation remain outside the Portal owner boundary. This workpack is source accepted and validation open; it is not `READY`, `PR_READY`, or `DONE`.

## Parallel Ownership Notes

B owns LAN implementation. C owns how those states are presented.
