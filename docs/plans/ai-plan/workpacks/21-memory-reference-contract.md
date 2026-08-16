# 21 - Memory Reference Contract

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `21 - Memory Reference Contract`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Memory refs are typed, local, source-cited, confidence-scored where needed, and
invalidatable.

## Where We Are

Local AI memory graph contracts and core activity memory graph pieces exist.
The source-citation guard must be enforced before memory influences decisions.

## Checklist

- [ ] Define memory reference kinds.
- [ ] Require source evidence refs.
- [ ] Include policy/action refs where applicable.
- [ ] Include generated time, expiry, confidence, and index version.
- [ ] Reject unsourced memory for decisioning.

## Proof

- Memory contract tests.
- Unsourced memory rejected test.
- Expired memory ignored test.
