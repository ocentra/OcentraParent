# AI UI/UX Requirements Guide

## Goal

The parent must be able to see what AI is doing, what it cannot do, and why a
decision or explanation exists. AI UI must not hide uncertainty.

## Required Surfaces

### AI Runtime

- Local runtime configured/unconfigured.
- Model artifact status.
- Provider availability.
- Hardware fit.
- Current load state.
- Generation state.
- Queue/backpressure state.
- Last checked time.
- Degraded/unavailable reason.

### AI Jobs

- Active jobs.
- Pending jobs.
- Cancelled jobs.
- Timed-out jobs.
- Failed/degraded jobs.
- Source evidence refs.
- Task scope.
- Provider route.
- Result ref.

### AI Decisions

- Decision action from policy, not model.
- AI evidence result.
- Parent rule refs.
- Evidence refs.
- Model/runtime ref.
- Prompt/template version.
- Confidence and degraded state.
- Human-readable reason.

### Memory And Graph

- Recent activity memory state.
- Semantic memory state.
- Graph index state.
- Source evidence refs.
- Expiry/invalidation state.
- Rebuild state.
- Unsourced memory rejected state.

### Remote Boundary

- Remote disabled by default.
- Parent authorization state.
- Data custody/retention state.
- Redaction state.
- Remote unavailable/degraded state.

## UI Rules

- Do not show raw screenshots by default.
- Do not show unbounded child activity in assistant prompts.
- Do not imply AI is the policy authority.
- Show unavailable, unsupported, low-confidence, and manual-required states as
  first-class product states.
- Keep detailed diagnostic data available for parent/developer proof, but keep
  the default UI readable.
- Every explanation must be traceable to evidence and parent rules.

## Snapshot Requirements

Before claiming UI done, capture screenshots for:

- runtime unconfigured;
- runtime configured but unavailable;
- local model ready;
- job queued/running;
- timeout/degraded;
- AI result with explanation;
- memory source-citation state;
- remote disabled state;
- screen OCR/VLM permission-required state if screen UI changes.
