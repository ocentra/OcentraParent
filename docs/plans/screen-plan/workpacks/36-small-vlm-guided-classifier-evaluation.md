# 36 Small VLM Guided Classifier Evaluation

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `36 Small VLM Guided Classifier Evaluation`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Target State

Small local VLM is used only for guided classification on safe crops when structured/OCR evidence is insufficient.

## MVP Boundary

This is AI-pass work. Capture MVP should define the route/result contracts and defer model choice.

## Checklist

- [ ] Verify configured local image capability for the current Windows proof route.
- [ ] Evaluate Qwen2.5-VL/Qwen2-VL candidates when the default runtime is insufficient.
- [ ] Use detector-specific JSON prompts.
- [ ] Limit image pixels and crop regions.
- [ ] Reject open-ended descriptions.
- [ ] Measure runtime and public-live crop quality.
- [ ] Record uncertainty/manual-required and fallback behavior.

## Proof

- Guided detector test set.
- Model capability proof.
- Resource proof.

Expected readiness proof (missing in this checkout):

```powershell
node scripts/test/screen-vlm-guided-classifier-readiness-proof.mjs
```

Expected artifact (not present):

```text
output/screen-plan-proof/36-small-vlm-guided-classifier-evaluation/proof-summary.json
```

The expected readiness proof script is absent, so this workpack currently has
no retained readiness-proof execution or expected proof artifact. The retained
local VLM execution-readiness contract proof does not substitute for the missing
script; it cannot establish the guided worker template/version, local-only
custody, bounded image-pixel budget, deleted query-store requirement before
completed status, or manual-required behavior for this expected root.

The retained Windows-lane local VLM matrix and live crop quality proofs now go
beyond readiness:

```powershell
node scripts/test/screen-ai-local-vlm-proof.mjs
node scripts/test/screen-vlm-live-crop-quality-proof.mjs
node scripts/test/screen-vlm-runtime-resource-measurement-proof.mjs
node scripts/test/screen-vlm-resource-crop-readiness-proof.mjs
node scripts/test/screen-vlm-model-selection-proof.mjs
node scripts/test/screen-vlm-rollout-fallback-gate-proof.mjs
```

Artifacts:

```text
output/screen-ai-pipeline-proof/proof-summary.json
output/screen-plan-proof/36-vlm-live-crop-quality/proof-summary.json
output/screen-plan-proof/36-vlm-runtime-resource-measurement/proof-summary.json
output/screen-plan-proof/36-vlm-resource-crop-readiness/proof-summary.json
output/screen-plan-proof/36-vlm-model-selection/proof-summary.json
output/screen-plan-proof/36-vlm-rollout-fallback-gate/proof-summary.json
```

The retained local matrix runs Qwen2-VL over real captured Windows/browser
proof images and schema-validates the guided classifier output. The retained
live crop proof loads public live pages for video, school/productivity, browser
game, shopping, and public social/feed categories, captures managed-browser
crops, runs local Qwen2-VL, records expected visible term/category matches, and
deletes raw crop files after analysis. The retained resource proof records
per-sample wall time, CPU time, peak working set, model/mmproj paths, bounded
crop dimensions, and no-raw-retention custody. These are local Windows proof
artifacts, not broad production rollout claims.

These retained artifacts do not close WP36's expected proof contract because the
readiness proof script and expected proof artifact are absent. The selected
local llama.cpp/Qwen2-VL path remains a documented candidate boundary only; it
does not close product-quality rollout across additional hardware profiles,
authenticated-account social/feed quality, cross-platform model/runtime parity,
portal runtime rendering, enforcement, or the full screen-AI pipeline.

## Current Decision

- Keep the guided local VLM route as the Windows proof-leading VLM path when
  OCR/structured evidence is insufficient.
- Keep open-ended descriptions rejected; detector-specific JSON prompts and
  bounded crops remain required.
- Do not claim authenticated-account social coverage from the public-feed
  proof.
- Do not claim cross-platform VLM parity, hardware rollout thresholds, or
  production model-quality completion until separate device/profile proof
  artifacts exist.
