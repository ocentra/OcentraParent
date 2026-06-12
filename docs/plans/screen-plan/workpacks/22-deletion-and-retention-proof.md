# 22 Deletion And Retention Proof

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `22 Deletion And Retention Proof`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Delete after success, delete after expiry, delete-failed visible state, deletion proof refs, and no silent long-term raw image behavior are implemented.

## Current State

Retention/deletion is specified but not fully proved.

## Checklist

- [ ] Delete raw image after success.
- [ ] Delete raw image after expiry.
- [ ] Record delete-failed state.
- [ ] Record deletion proof ref.
- [ ] Show deletion state in portal.
- [ ] Prove no default long-term raw image retention.

## Proof

- Queue directory before/after proof.
- Store record deletion proof.
- Portal screenshot.
