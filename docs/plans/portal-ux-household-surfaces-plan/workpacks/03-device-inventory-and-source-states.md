# 03 Device Inventory And Source States

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

- [ ] Distinguish local agent, LAN agent, passive neighbor, router, ignored,
      stale, offline, and revoked states.
- [ ] Show controllable versus visible-only.
- [ ] Show source/custody labels.
- [ ] Show `Not reported` instead of inventing hardware details.
- [ ] Test long hostnames and many-device layouts.

## Acceptance And Proof

Device UI reads service state and never treats passive LAN neighbors as policy
targets.

## Parallel Ownership Notes

B owns LAN implementation. C owns how those states are presented.
