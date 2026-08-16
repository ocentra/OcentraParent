# 12 Child-Facing Status And Reasons

<!-- agent-capsule -->

> Agent Capsule
> Plan: `v0-8-enforcement-control-plan`
> Doc: `12 Child-Facing Status And Reasons`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [20-step plan](../v0-8-enforcement-control-20-step-plan.md),
[test blueprint](../v0-8-enforcement-control-test-blueprint.md), and
[folder README](../README.md).

## Where We Are

Parent-facing proof states exist. Child-facing status remains incomplete as a
product experience.

## Where We Want To Be

When a child is warned, time-limited, blocked, or asked to request permission,
the child device can show a stable reason and next action without portal logic.

## Requirement Checklist

- [ ] Define reason codes and text-token references.
- [ ] Support time limit reached, ask parent, allowed, warning, unavailable, and
      blocked by parent rule.
- [ ] Include target and schedule context where safe.
- [ ] Keep generated/AI text out of enforcement status.
- [ ] Preserve local/offline behavior.

## Acceptance And Proof

Contract and service tests prove child-facing reason state is returned with
action results and survives audit.

## Parallel Ownership Notes

This is runtime/product state, not visual polish. UI lanes may render it after
service state exists.
