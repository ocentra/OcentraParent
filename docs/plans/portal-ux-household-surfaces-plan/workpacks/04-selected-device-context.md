# 04 Selected Device Context

<!-- agent-capsule -->

> Agent Capsule
> Plan: `portal-ux-household-surfaces-plan`
> Doc: `04 Selected Device Context`
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

Some portal surfaces can show device state, but cross-route selected-device
context still needs product-level consistency.

## Where We Want To Be

Policy, Activity, AI, Reports, and Account surfaces clearly show which
child/device/route they apply to before any action.

## Requirement Checklist

- [ ] Show selected child/device/route source on every relevant route.
- [ ] Preserve context across navigation.
- [ ] Fail visibly when no valid device is selected.
- [ ] Prevent wrong-device action confusion.
- [ ] Test context persistence.

## Acceptance And Proof

Playwright navigates across routes and verifies selected-device state is visible.

## Parallel Ownership Notes

This workpack is UX/state consumption, not route authority implementation.
