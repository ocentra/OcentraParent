# 09 Browser, App, And Network Surfaces

<!-- agent-capsule -->

> Agent Capsule
> Plan: `portal-ux-household-surfaces-plan`
> Doc: `09 Browser, App, And Network Surfaces`
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

Browser, app/game, and network evidence each have different trust boundaries.
The UI must keep those distinctions visible.

## Where We Want To Be

Parents can see exact managed browser evidence, unmanaged browser detection,
app/game sessions, and network flow summaries without confusing one for another.

## Requirement Checklist

- [ ] Label exact URL only for managed browser evidence.
- [ ] Label unmanaged browser as process-only/possible bypass.
- [ ] Show app/game session duration from stored evidence.
- [ ] Show network flow as metadata, not page content.
- [ ] Test unknown and degraded states.

## Acceptance And Proof

UI tests fail if weaker evidence is presented as a stronger claim.

## Parallel Ownership Notes

This is a UX truthfulness workpack; runtime evidence producers remain separate.
