# 07 - Deletion Retention And Custody Gate

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-ai-pipeline-plan`
> Doc: `07 - Deletion Retention And Custody Gate`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Raw image custody is explicit. Default behavior deletes temporary images and
does not upload raw screenshots remotely.

## Checklist

- [x] Queue image encrypted.
- [x] Raw path redacted outside child agent.
- [x] Delete after success.
- [x] Delete after TTL.
- [x] Delete failure visible.
- [x] Remote/cloud screenshot upload disabled.
- [x] Retention requires explicit opt-in if used.

## Proof

- Queue encryption artifact.
- Deletion proof artifact.
- Remote disabled proof.
- Retention non-claim or opt-in proof.
- Current proof artifacts:
  `output/screen-ai-pipeline-proof/service-cadence/proof-summary.json`,
  `output/screen-ai-pipeline-proof/service-analysis/proof-summary.json`,
  `output/screen-ai-pipeline-proof/service-retention-sweeper/proof-summary.json`,
  `output/ai-plan-proof/real-analysis/proof-summary.json`, and
  `output/screen-ai-pipeline-proof/live-operator/proof-summary.json`.
- TTL/delete-failure/retention-mode contract proof:
  `output/screen-ai-pipeline-proof/deletion-retention-custody/proof-summary.json`.
- The service retention sweeper proof removes an expired encrypted queue record
  and records an `expiredDeleted` Activity Screen row through the real Rust
  service/WebSocket path. The retention proof rejects unsupported raw
  screenshot retention. Production parent UI retention controls remain a
  non-claim.
