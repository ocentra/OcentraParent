# 04 - AI Result To Policy Gate

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-ai-pipeline-plan`
> Doc: `04 - AI Result To Policy Gate`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Only schema-valid AI results reach deterministic parent policy.

## Checklist

- [x] AI result cites evidence refs.
- [x] AI result cites parent-rule refs.
- [x] Confidence/degraded state valid.
- [x] Invalid output rejected before policy.
- [x] Stricter parent rule wins.

## Proof

- AI result artifact.
- Policy decision artifact.
- Invalid output rejection log.
- Parent-rule conflict proof.
- Block action handoff source artifact:
  `output/screen-ai-pipeline-proof/block-action-dispatch/00-screen-block-source.json`.
