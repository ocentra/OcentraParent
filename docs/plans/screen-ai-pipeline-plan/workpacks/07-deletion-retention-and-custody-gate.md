# 07 - Deletion Retention And Custody Gate

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
  `output/ai-plan-proof/real-analysis/proof-summary.json`, and
  `output/screen-ai-pipeline-proof/live-operator/proof-summary.json`.
- TTL/delete-failure/retention-mode contract proof:
  `output/screen-ai-pipeline-proof/deletion-retention-custody/proof-summary.json`.
- The retention proof rejects unsupported raw screenshot retention. Production
  parent UI retention controls and a background TTL sweeper remain non-claims.
