# 05 - Policy Action Dry-Run Gate

## Target State

Pipeline proves policy actions or dry-run actions without letting AI enforce
directly.

## Checklist

- [x] Observe action proof.
- [x] Allow action proof.
- [x] Warn action proof.
- [x] Ask-parent action proof.
- [x] Time-limit action proof with timer/expiry refs.
- [x] Block dry-run proof and owned-process Windows adapter proof.
- [x] Unknown/manual-required proof.

## Proof

- Action or dry-run artifact.
- Time-limit adapter artifact:
  `output/screen-ai-pipeline-proof/action-dispatch/proof-summary.json`.
- Block adapter artifact:
  `output/screen-ai-pipeline-proof/block-action-dispatch/proof-summary.json`.
- Enforcement-adapter non-claim for browser, category, network/domain, mobile,
  and broad block paths until those adapters have real proof.
- Audit event artifact.
