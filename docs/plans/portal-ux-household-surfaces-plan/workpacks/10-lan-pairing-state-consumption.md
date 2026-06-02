# 10 LAN Pairing State Consumption

Sources: [20-step plan](../portal-ux-household-surfaces-20-step-plan.md),
[test blueprint](../portal-ux-household-surfaces-test-blueprint.md),
[UI/UX guide](../ui-ux-requirements-guide.md), and [folder README](../README.md).

## Where We Are

B owns the full LAN plan. C must consume those states without duplicating LAN
truth.

## Where We Want To Be

LAN states appear coherently in Devices and related routes: discovered,
assigned, confirmed, trusted, ignored, stale, offline, router, infrastructure,
and manual-required.

## Requirement Checklist

- [ ] Use B/service read models for LAN state.
- [ ] Keep routers/infrastructure visible but non-enrollable.
- [ ] Show assignment/ignore/revocation state where available.
- [ ] Show manual-required physical proof labels.
- [ ] Test LAN fixture states without claiming real LAN proof.

## Acceptance And Proof

Portal LAN UI is a consumer of B-owned state and does not create a second source
of truth.

## Parallel Ownership Notes

B owns LAN implementation and proof. C owns presentation and ergonomics.
