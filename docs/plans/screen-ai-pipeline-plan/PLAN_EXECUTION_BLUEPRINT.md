# Screen AI Pipeline Plan � HID Execution Blueprint

## Execution objective

Close the final capture-trigger and rollout gates, proving advisory AI output quality and safety boundaries.

## Slice 01 � Trigger-to-Capture Gate

### Acceptance

- Capture starts only on authenticated policy-permitted trigger with replay protection.

### Tests

- `screen-ai.ocr-output.invariants`
- `screen-ai.trigger-authn-authz`

### Proof

- `docs/proof/screen-ai-pipeline-plan/slice-01-trigger-capture.md`

## Slice 02 � OCR/VLM Output Quality and Invariants

### Acceptance

- Model outputs remain schema-valid, bounded, and non-assertive on unsupported contexts.

### Tests

- `screen-ai.vlm-output.invariants`
- `screen-ai.hallucination.regression`

### Proof

- `docs/proof/screen-ai-pipeline-plan/slice-02-output-quality.md`

## Slice 03 � PR and Rollout Gate

### Acceptance

- Evidence and operator handoff matrix includes failure and manual-required cases.

### Tests

- `screen-ai.redaction.custody`
- `screen-ai.safety-boundary`

### Proof

- `docs/proof/screen-ai-pipeline-plan/slice-03-rollout-pr-gate.md`

## Workpacks (execution lane)

### Slice-to-workpack binding

- Slice 01: docs/plans/screen-ai-pipeline-plan/workpacks/01-prerequisite-merge-and-branch-gate.md
- Slice 02: docs/plans/screen-ai-pipeline-plan/workpacks/02-real-trigger-to-capture-gate.md
- Slice 03: docs/plans/screen-ai-pipeline-plan/workpacks/03-capture-to-ai-analysis-gate.md

## PR-ready gate

- No final PR claim without rollout proof and explicit no-policy-authority boundary.

## HID test floor (this plan)

### Required test families for closed slice

- Unit: capture/policy parser invariants
- Integration: model routing and queue-to-policy handoff
- E2E: real-trigger to policy signal path
- Security: prompt injection, PII boundary, output invariants
- Non-functional: queue latency and throughput

### Mandatory slice evidence checks

- negative cases documented (at least one per slice)
- rollback/teardown proof recorded
- proof manifest references command output, artifacts, and manual review notes
