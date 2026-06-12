# AI Plan Next Actions

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `AI Plan Next Actions`
> Kind: resume queue and highest-open work.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

This file is the short resume list for the next worker. It is derived from open workpack/checklist status and does not replace the assigned workpack.

## How to use

1. Confirm the hub assignment and lane.
2. Pick only the assigned workpack from the list below.
3. Open that workpack and exact checklist rows only.
4. Do not claim `DONE` unless the assigned workpack's acceptance/proof rows are updated and validation is listed.

## Highest-open workpacks by unchecked boxes

- [48 - Rollout Checklist And PR Gate](workpacks/48-rollout-checklist-and-pr-gate.md): 25 open of 25 boxes.
- [32 - Household AI Provider Mesh And Remote Assistant Boundary](workpacks/32-family-ai-hub-and-remote-assistant-boundary.md): 11 open of 11 boxes.
- [07 - AI Job Queue Contract](workpacks/07-ai-job-queue-contract.md): 10 open of 11 boxes.
- [08 - AI Provider Routing Contract](workpacks/08-ai-provider-routing-contract.md): 10 open of 10 boxes.
- [38 - Screen OCR VLM Router Lane](workpacks/38-screen-ocr-vlm-router-lane.md): 9 open of 9 boxes.
- [31 - Guided VLM Worker Lane](workpacks/31-guided-vlm-worker-lane.md): 8 open of 8 boxes.
- [46 - Security Privacy Negative Gates Lane](workpacks/46-security-privacy-negative-gates-lane.md): 8 open of 8 boxes.
- [47 - Performance Resource Battery Proof Lane](workpacks/47-performance-resource-battery-proof-lane.md): 8 open of 8 boxes.
- [03 - Contract Boundary And Effect Schemas](workpacks/03-contract-boundary-and-effect-schemas.md): 6 open of 6 boxes.
- [09 - Local Evidence Context Builder V1](workpacks/09-local-evidence-context-builder-v1.md): 6 open of 6 boxes.
- [13 - Deterministic No-Model Classifier Lane](workpacks/13-deterministic-no-model-classifier-lane.md): 6 open of 6 boxes.
- [14 - Local Text LLM Adapter Boundary](workpacks/14-local-text-llm-adapter-boundary.md): 6 open of 6 boxes.
- [15 - Local Text LLM Execution Dry-Run Adapter](workpacks/15-local-text-llm-execution-dry-run-adapter.md): 6 open of 6 boxes.
- [16 - Output Parser And Schema Validator](workpacks/16-output-parser-and-schema-validator.md): 6 open of 6 boxes.
- [17 - Degraded Timeout Invalid-Output Handling](workpacks/17-degraded-timeout-invalid-output-handling.md): 6 open of 6 boxes.
- [20 - Parent Explanation Read Model](workpacks/20-parent-explanation-read-model.md): 6 open of 6 boxes.
- [23 - Evidence-Backed Semantic Memory](workpacks/23-evidence-backed-semantic-memory.md): 6 open of 6 boxes.
- [25 - Minimal Graph Edges For Safety Context](workpacks/25-minimal-graph-edges-for-safety-context.md): 6 open of 6 boxes.
- [33 - Browser URL Video AI Lane](workpacks/33-browser-url-video-ai-lane.md): 6 open of 6 boxes.
- [35 - Browser Game Cloud Game AI Lane](workpacks/35-browser-game-cloud-game-ai-lane.md): 6 open of 6 boxes.

## PR readiness guard

A PR-ready slice should close a named workpack or clearly explain the exact remaining rows. Do not create a tiny PR that only updates one proof note while leaving the assigned workpack/checklist state stale.

Before reporting `DONE` or `PR_READY`, update the workpack, checklist row(s), proof reference(s), and feature/product docs if product status changed.

## Actioned completion tracker

- [ ] Re-check this plan route from AGENTS/PLAN_STATE and confirm the assigned workpack path.
- [ ] Update one assigned workpack and matching checklist/proof rows before reporting progress.
- [ ] Record failure conditions, skipped checks, and evidence path in PLAN_STATE/TEST_PROOF_EXPECTATIONS for every claimed progress.
