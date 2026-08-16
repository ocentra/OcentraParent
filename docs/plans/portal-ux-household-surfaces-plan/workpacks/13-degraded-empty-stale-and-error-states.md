# 13 Degraded, Empty, Stale, And Error States

<!-- agent-capsule -->

> Agent Capsule
> Plan: `portal-ux-household-surfaces-plan`
> Doc: `13 Degraded, Empty, Stale, And Error States`
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

Several routes show current data, but complete empty/degraded/error behavior is
not guaranteed.

## Where We Want To Be

Every route has useful states for empty, loading, stale, offline, service
unavailable, permission-limited, unsupported, manual-required, and error cases.

## Requirement Checklist

- [ ] Avoid fake success in empty states.
- [ ] Show next valid action.
- [ ] Keep error text inside containers without overlap.
- [ ] Include retry/copy/debug where useful.
- [ ] Add tests for representative degraded states.

## Acceptance And Proof

Screens remain understandable when the service is disconnected or data is
unavailable.

## Parallel Ownership Notes

This is a C-owned UX quality gate for every route it touches.
