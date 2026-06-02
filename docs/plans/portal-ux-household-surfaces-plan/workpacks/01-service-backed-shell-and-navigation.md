# 01 Service-Backed Shell And Navigation

Sources: [20-step plan](../portal-ux-household-surfaces-20-step-plan.md),
[test blueprint](../portal-ux-household-surfaces-test-blueprint.md),
[UI/UX guide](../ui-ux-requirements-guide.md), and [folder README](../README.md).

## Where We Are

The portal has route surfaces, but the product still needs one coherent shell
that makes connection, source, and selected-device state visible.

## Where We Want To Be

Navigation feels like one parent product and every route starts from validated
service/read-model state.

## Requirement Checklist

- [ ] Show connection and source state consistently.
- [ ] Keep route ids, DOM ids, and display tokens domain-owned.
- [ ] Avoid fake route-local success.
- [ ] Support desktop and mobile navigation without overlap.
- [ ] Add Playwright checks for shell state.

## Acceptance And Proof

Playwright verifies shell connection/source labels against the real service.

## Parallel Ownership Notes

C owns visual structure. Runtime truth remains with the service and domain
contracts.
