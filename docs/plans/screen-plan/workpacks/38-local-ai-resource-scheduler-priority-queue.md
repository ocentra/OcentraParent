# 38 Local AI Resource Scheduler Priority Queue

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `38 Local AI Resource Scheduler Priority Queue`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

One-heavy-job scheduling, OCR/VLM rate limits, and policy-priority behavior are defined.

## MVP Boundary

Capture MVP should define queue priority fields. Full scheduler proof belongs to AI-pass work.

## Checklist

- [ ] Define AI job type and priority.
- [ ] Enforce one heavy job at a time on normal PCs.
- [ ] Prioritize policy-blocking jobs over background summaries.
- [ ] Add timeout and skipped/degraded states.
- [ ] Add max image pixels and OCR snippet limits.
- [ ] Report model/runtime queue status.

## Proof

- `packages/activity-domain/src/screen-evidence-resource-scheduler.ts`
  defines screen OCR/VLM/deterministic job kind, priority, heavy/light/no-model
  resource weight, queue state, timeout/skipped/degraded states, pixel caps, OCR
  snippet caps, local-only custody, no-remote-AI, and no-raw-retention fields.
- `packages/activity-domain/src/screen-evidence-resource-scheduler-proof.ts`
  records the proof matrix: one running policy-blocking heavy VLM job, queued
  cadence/background heavy VLM work, completed OCR, timed-out VLM, skipped
  protected surface, and deterministic no-model completion.
- `packages/activity-domain/tests/screen-evidence-resource-scheduler.test.ts`
  rejects duplicate heavy jobs, bad admission order, queued jobs without
  singleton blocking, raw retention, remote AI, and cap violations.
- `scripts/test/screen-local-ai-resource-scheduler-proof.mjs` writes
  `output/screen-plan-proof/local-ai-resource-scheduler/proof-summary.json` and
  invokes the existing `scripts/test/local-ai-provider-scheduler-proof.mjs`
  runtime proof for the shared local provider singleton scheduler.
