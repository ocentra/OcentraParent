# 16 No-Fake-Data Contract Adapter

<!-- agent-capsule -->

> Agent Capsule
> Plan: `portal-ux-household-surfaces-plan`
> Doc: `16 No-Fake-Data Contract Adapter`
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
