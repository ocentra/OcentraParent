# 38 Local AI Resource Scheduler Priority Queue

## Target State

One-heavy-job scheduling, OCR/VLM rate limits, and policy-priority behavior are defined.

## MVP Boundary

Capture MVP should define queue priority fields. Full scheduler proof belongs to AI-pass work.

## Checklist

- [x] Define AI job type and priority.
- [x] Enforce one heavy job at a time on normal PCs.
- [x] Prioritize policy-blocking jobs over background summaries.
- [x] Add timeout and skipped/degraded states.
- [x] Add max image pixels and OCR snippet limits.
- [x] Report model/runtime queue status.

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
