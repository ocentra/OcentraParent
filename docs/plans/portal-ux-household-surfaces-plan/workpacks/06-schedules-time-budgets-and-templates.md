# 06 Schedules, Time Budgets, And Templates

<!-- agent-capsule -->

> Agent Capsule
> Plan: `portal-ux-household-surfaces-plan`
> Doc: `06 Schedules, Time Budgets, And Templates`
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

Schedules and budgets are part of the product gap. Existing proof is not a
complete parent scheduling UX.

## Where We Want To Be

Parents can reason about school time, bedtime, free time, app/game limits,
exceptions, bonus time, and conflicts in one predictable workflow.

## Requirement Checklist

- [ ] Use schedule/time-budget contracts where available.
- [ ] Show active, inactive, upcoming, expired, and conflict states.
- [ ] Provide parent-friendly templates without hiding details.
- [ ] Keep timers service-owned.
- [ ] Test long labels and compact widths.

## Acceptance And Proof

Schedule UI makes current/effective state understandable without claiming
unsupported enforcement.

## Parallel Ownership Notes

Runtime timer proof belongs to A/service work. C renders it.
