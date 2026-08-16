# 37 - Tracking Location Safety Analysis Lane

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `37 - Tracking Location Safety Analysis Lane`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Tracking AI explains expected-place, nearby-place ambiguity, stale/offline
state, unusual movement, and parent acknowledgement using typed location
evidence and parent rules.

## Where We Are

Tracking plan exists separately. AI should support explanation and ambiguity
handling, not replace location policy.

## Checklist

- [ ] Consume tracking/location evidence refs.
- [ ] Include expected-place and schedule context.
- [ ] Include stale/offline states.
- [ ] Include nearby-place ambiguity.
- [ ] Return explanation support result.
- [ ] Feed deterministic tracking policy.

## Proof

- Location AI alert support test.
- Stale/offline degrade test.
- Parent acknowledgement explanation proof.
