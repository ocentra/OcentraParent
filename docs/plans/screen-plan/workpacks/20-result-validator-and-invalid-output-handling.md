# 20 Result Validator And Invalid Output Handling

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `20 Result Validator And Invalid Output Handling`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Invalid JSON, missing refs, invalid confidence, unsupported categories, raw text overflow, and malformed deletion state are rejected.

## Current State

Validation direction exists; complete malformed-output proof is open.

## Checklist

- [ ] Reject invalid JSON.
- [ ] Reject missing source refs.
- [ ] Reject invalid confidence.
- [ ] Reject unsupported categories.
- [ ] Reject raw text overflow.
- [ ] Reject missing/malformed deletion state.
- [ ] Record invalid/unknown state.

## Proof

- Negative contract tests.
- Service tests showing invalid output cannot drive policy.
