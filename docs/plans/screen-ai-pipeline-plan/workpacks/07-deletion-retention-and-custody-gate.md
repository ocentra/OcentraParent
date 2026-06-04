# 07 - Deletion Retention And Custody Gate

## Target State

Raw image custody is explicit. Default behavior deletes temporary images and
does not upload raw screenshots remotely.

## Checklist

- [x] Queue image encrypted.
- [x] Raw path redacted outside child agent.
- [x] Delete after success.
- [ ] Delete after TTL.
- [ ] Delete failure visible.
- [x] Remote/cloud screenshot upload disabled.
- [ ] Retention requires explicit opt-in if used.

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
- Remaining unchecked rows require explicit TTL/delete-failure/retention-mode
  proof in the combined pipeline branch, not just contract-level expectation
  text.
