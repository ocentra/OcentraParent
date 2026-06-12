# 11 Audit And Journal Events

<!-- agent-capsule -->

> Agent Capsule
> Plan: `v0-8-enforcement-control-plan`
> Doc: `11 Audit And Journal Events`
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

Enforcement action states exist, but product trust requires durable audit for
actions, failures, previews, timer transitions, and approvals.

## Where We Want To Be

Every product-control transition has a durable event that can be queried by
portal, reports, and proof scripts with evidence and policy references.

## Requirement Checklist

- [ ] Journal action accepted, action rejected, adapter result, and no-op.
- [ ] Journal timer and rollback transitions.
- [ ] Journal approvals, denials, expiry, and overrides.
- [ ] Include evidence, policy, actor, route, and target references.
- [ ] Add read-model/query coverage for recent action history.

## Acceptance And Proof

Audit tests can reconstruct what happened, why, who requested it, and whether it
changed device behavior.

## Parallel Ownership Notes

Reports and assistant surfaces should consume this history instead of inventing
summaries.
