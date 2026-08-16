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

Sources: [20-step plan](../portal-ux-household-surfaces-20-step-plan.md), [test blueprint](../portal-ux-household-surfaces-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), and [folder README](../README.md).

## Ownership boundary

```text
portal UX owns visible fixture/runtime labels, invalid payload display, schema-decode display, and no-product-claim boundary.
domain/schema/protocol owners own the actual service read-model contracts.
data-custody owns custody/retention/export/delete semantics.
logging/evidence owners own proof artifact and diagnostic storage.
```

## Where We Are

Design and test states sometimes need fixtures. Product routes must not mistake fixtures for runtime state.

## Where We Want To Be

Fixtures, demos, and runtime service payloads are separated and labeled; runtime payloads are decoded before rendering.

## Required proof fields

The selected proof must name, at minimum:

```text
route
payload_source
fixture_state
runtime_state
schema_decode_state
invalid_payload_state
missing_payload_state
portal_local_replacement_state
source_label_state
custody_label_state
product_claim_state
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Requirement Checklist

- [ ] Decode service payloads with domain/protocol schemas.
- [ ] Label demo/fixture states explicitly.
- [ ] Avoid portal-local replacements for service read models.
- [ ] Add tests for invalid/missing payload states.
- [ ] Keep product claims tied to real-service proof.

## Acceptance And Proof

UI tests can tell whether they are exercising service state or explicit fixtures.

Proof must include fixture-mode, service-mode, invalid-payload, missing-payload, and no-product-claim cases for the selected route.

## Failure conditions

- Do not render fixtures as runtime state.
- Do not replace service read models with portal-local truth.
- Do not claim product readiness from demo/fixture state.
- Do not hide source/custody labels when they are required for the selected view.

## Parallel Ownership Notes

This protects C from accidentally making visual-only work look product-complete.
