# 01 - Prerequisite Merge And Branch Gate

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-ai-pipeline-plan`
> Doc: `01 - Prerequisite Merge And Branch Gate`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Pipeline work starts only after screen and AI prerequisite implementations are
merged to `main` or explicitly approved as stacked heads.

## Checklist

- [ ] Record screen prerequisite branch/commit/PR.
- [ ] Record AI prerequisite branch/commit/PR.
- [ ] Confirm pipeline branch contains both implementations.
- [ ] Confirm no stale capture or AI proof assumptions.
- [ ] Run lane/hub guards before edits.

## Proof

- `output/screen-ai-pipeline-proof/prerequisite-merge/proof-summary.json`.
- Git status and branch base recorded in the prerequisite merge proof.
- The implementation checklist and PR-ready hub reports name the stacked proof
  commits and non-claims.
