# 16 No-Fake-Data Contract Adapter

Sources: [20-step plan](../portal-ux-household-surfaces-20-step-plan.md),
[test blueprint](../portal-ux-household-surfaces-test-blueprint.md),
[UI/UX guide](../ui-ux-requirements-guide.md), and [folder README](../README.md).

## Where We Are

Design and test states sometimes need fixtures. Product routes must not mistake
fixtures for runtime state.

## Where We Want To Be

Fixtures, demos, and runtime service payloads are separated and labeled; runtime
payloads are decoded before rendering.

## Requirement Checklist

- [ ] Decode service payloads with domain/protocol schemas.
- [ ] Label demo/fixture states explicitly.
- [ ] Avoid portal-local replacements for service read models.
- [ ] Add tests for invalid/missing payload states.
- [ ] Keep product claims tied to real-service proof.

## Acceptance And Proof

UI tests can tell whether they are exercising service state or explicit fixtures.

## Parallel Ownership Notes

This protects C from accidentally making visual-only work look product-complete.
