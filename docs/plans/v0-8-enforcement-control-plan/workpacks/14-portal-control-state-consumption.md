# 14 Portal Control State Consumption

<!-- agent-capsule -->

> Agent Capsule
> Plan: `v0-8-enforcement-control-plan`
> Doc: `14 Portal Control State Consumption`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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
