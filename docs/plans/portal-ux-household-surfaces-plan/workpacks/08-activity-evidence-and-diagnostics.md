# 08 Activity Evidence And Diagnostics

<!-- agent-capsule -->

> Agent Capsule
> Plan: `portal-ux-household-surfaces-plan`
> Doc: `08 Activity Evidence And Diagnostics`
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

Evidence paths exist across browser, app/game, network, and screen summaries.
The UI needs consistent diagnostic and parent-facing patterns.

## Where We Want To Be

Activity surfaces show evidence source, confidence, freshness, custody, and
unknown/degraded state before summaries or assistant actions.

## Requirement Checklist

- [ ] Show source evidence refs where available.
- [ ] Separate confidence from certainty.
- [ ] Label local-only, parent-cache, and unavailable custody.
- [ ] Show stale/degraded/unknown states.
- [ ] Add copy/debug output with redaction.

## Acceptance And Proof

Activity views never invent evidence or collapse unavailable state into normal
data.

## Parallel Ownership Notes

Evidence storage/runtime ownership remains outside C.
