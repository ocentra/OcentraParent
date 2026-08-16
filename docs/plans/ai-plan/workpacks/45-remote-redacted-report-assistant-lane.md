# 45 - Remote Redacted Report Assistant Lane

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `45 - Remote Redacted Report Assistant Lane`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Remote parent assistant can explain parent-approved reports with citations,
redaction, retention state, and uncertainty, while local safety remains
authoritative.

## Where We Are

Parent assistant routing proof exists. Report assistant must wait for
parent-owned source bundle/custody proof.

## Checklist

- [ ] Define report bundle source refs.
- [ ] Add redaction/minimization state.
- [ ] Require parent approval.
- [ ] Require cited answer.
- [ ] Degrade to local-only on remote failure.

## Proof

- Remote report request tests.
- Citation required tests.
- Retention state portal proof.
