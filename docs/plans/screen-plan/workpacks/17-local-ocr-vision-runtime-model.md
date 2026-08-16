# 17 Local OCR Vision Runtime Model

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `17 Local OCR Vision Runtime Model`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Local model runtime status, OCR/vision tasks, no OS/file/network scanning, and structured JSON only are enforced.

## Current State

The local OCR/vision runtime model is contract/runtime proved by existing
screen artifacts and the aggregate WP17 proof. WinRT OCR has real Windows
capture/service proof, guided VLM has schema-bound worker/readiness/read-model
proof, and the local AI resource scheduler proves priority, caps, local-only
custody, no remote AI, no raw retention, and degraded/manual-required states.
This closes the runtime model gate without claiming production OCR/VLM quality,
live VLM inference, cross-platform OCR/VLM parity, authenticated-account
coverage, or enforcement.

## Checklist

- [ ] Define local runtime status.
- [ ] Define worker input boundary.
- [ ] Define worker output boundary.
- [ ] Reject cloud/API upload by default.
- [ ] Add low-confidence and unavailable states.
- [ ] Add model/version metadata.
- [ ] Prove local-only processing.

## Proof

- `output/screen-plan-proof/17-local-ocr-vision-runtime-model/proof-summary.json`.
- `output/ai-plan-proof/screen-winrt-ocr-worker/proof-summary.json`.
- `output/screen-ai-pipeline-proof/service-winrt-ocr/proof-summary.json`.
- `output/ai-plan-proof/screen-vlm-worker-contract-proof/proof-summary.json`.
- `output/ai-plan-proof/screen-vlm-execution-readiness-proof/proof-summary.json`.
- `output/ai-plan-proof/screen-vlm-journal-read-model-proof/proof-summary.json`.
- `output/screen-plan-proof/local-ai-resource-scheduler/proof-summary.json`.
