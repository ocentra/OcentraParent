# 14 Portal Control State Consumption

Sources: [20-step plan](../v0-8-enforcement-control-20-step-plan.md),
[test blueprint](../v0-8-enforcement-control-test-blueprint.md), and
[folder README](../README.md).

## Where We Are

Portal routes can display service state, but product-control UI must not become
the authority for policy evaluation or enforcement.

## Where We Want To Be

Portal shows action state, capability, proof level, pending approvals, active
timers, and known gaps from the service with no fake success path.

## Requirement Checklist

- [ ] Render service-backed action states only.
- [ ] Label dry-run, observe-only, active, unavailable, degraded, and
      manual-required.
- [ ] Use typed intents for parent actions.
- [ ] Show result/audit output after action.
- [ ] Add Playwright coverage for touched routes.

## Acceptance And Proof

Playwright proves parent controls talk to the real service and render returned
state without browser console/page errors.

## Parallel Ownership Notes

C may own look and ergonomics. A owns product-control state and service proof.
