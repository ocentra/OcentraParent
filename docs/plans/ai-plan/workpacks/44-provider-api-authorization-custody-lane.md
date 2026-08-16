# 44 - Provider API Authorization Custody Lane

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `44 - Provider API Authorization Custody Lane`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

External/API providers are parent-authorized, custody-labeled, retention-labeled,
and unavailable for normal child safety.

## Where We Are

API AI provider authorization proof exists. The route must remain separate from
child-device safety and visible in UI.

## Checklist

- [ ] Define API provider authorization state.
- [ ] Require parent action ref.
- [ ] Require data custody and retention state.
- [ ] Reject safety-path remote provider use.
- [ ] Add provider unavailable/degraded states.

## Proof

- API authorization proof.
- Remote disabled-by-default test.
- Custody label guard test.
