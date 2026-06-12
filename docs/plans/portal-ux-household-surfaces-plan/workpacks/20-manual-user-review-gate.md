# 20 Manual User Review Gate

<!-- agent-capsule -->

> Agent Capsule
> Plan: `portal-ux-household-surfaces-plan`
> Doc: `20 Manual User Review Gate`
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

The user owns UI/UX look, feel, layout, interaction, and ergonomics. Automated
tests cannot replace that review.

## Where We Want To Be

Each major C slice reaches a manual review gate with screenshots, validation,
known runtime gaps, and exact route URLs for the user to inspect.

## Requirement Checklist

- [ ] Provide route list and local URL.
- [ ] Provide screenshots or browser proof artifacts.
- [ ] State validation commands and results.
- [ ] State runtime gaps and manual-required states.
- [ ] Wait for user/C visual decision before UX-ready claims.

## Acceptance And Proof

The user can inspect the actual route and give focused UX direction without
needing to reconstruct what changed.

## Parallel Ownership Notes

Primary should not override C/user visual judgment unless there is a
merge-safety, CI, or product-truth issue.
