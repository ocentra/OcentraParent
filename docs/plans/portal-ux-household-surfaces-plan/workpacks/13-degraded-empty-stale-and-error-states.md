# 13 Degraded, Empty, Stale, And Error States

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
