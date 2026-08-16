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

Current checkpoint truth on this branch/worktree (2026-06-18):

- Focused proof is now recorded under `output/portal-ux-household-surfaces-plan-proof/03-device-inventory-and-source-states/`.
- The current packet proves the LAN inventory owner seam on current source: local child-agent, LAN child-agent, passive neighbor, router, ignored, stale, offline, and revoked states are all distinguished in the service-backed slot model instead of collapsing into fake green readiness.
- Passive LAN neighbors remain visible inventory rows but do not become canonical policy targets; ignored and revoked neighbors stay excluded from `createParentPortalCanonicalDeviceSlots(...)`.
- The selected-device surface now carries explicit source/custody/control-state consumption for the service-backed LAN slots, including distinct `Ignored`, `Revoked`, `Stale`, and `Offline` control labels.
- Missing LAN hardware details stay on the `Not reported` fallback path instead of inventing CPU/GPU/memory values, and the dense grid render proof keeps long selected labels and many-device layouts visible without synthetic hardware fill-in.
- This checklist row is now locally closed on this branch/worktree because the owner seam, focused portal tests, and portal app typecheck all reran green after the WP03 state and detail-surface corrections.

## Parallel Ownership Notes

B owns LAN implementation. C owns how those states are presented.
