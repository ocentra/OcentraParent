# 38 Local AI Resource Scheduler Priority Queue

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

- Scheduler tests.
- Load/resource proof.
