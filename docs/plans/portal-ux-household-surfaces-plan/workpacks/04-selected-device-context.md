# 04 Selected Device Context

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
