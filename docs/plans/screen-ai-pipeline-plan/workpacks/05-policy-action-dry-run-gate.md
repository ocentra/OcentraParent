# 05 - Policy Action Dry-Run Gate

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-ai-pipeline-plan`
> Doc: `05 - Policy Action Dry-Run Gate`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Pipeline proves policy actions or dry-run actions without letting AI enforce
directly.

## Checklist

- [ ] Observe action proof.
- [ ] Allow action proof.
- [ ] Warn action proof.
- [ ] Ask-parent action proof.
- [ ] Time-limit action proof with timer/expiry refs.
- [ ] Block dry-run proof and owned-process Windows adapter proof.
- [ ] Unknown/manual-required proof.

## Proof

- Action or dry-run artifact.
- Time-limit adapter artifact:
  `output/screen-ai-pipeline-proof/action-dispatch/proof-summary.json`.
- Block adapter artifact:
  `output/screen-ai-pipeline-proof/block-action-dispatch/proof-summary.json`.
- Enforcement-adapter non-claim for browser, category, network/domain, mobile,
  and broad block paths until those adapters have real proof.
- Audit event artifact.
