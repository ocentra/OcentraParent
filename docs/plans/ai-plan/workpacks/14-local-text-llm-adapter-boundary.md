# 14 - Local Text LLM Adapter Boundary

## Target State

The current local text model lane reasons over typed evidence and returns
schema-valid AI evidence. It cannot scan sources directly.

## Where We Are

Local AI chat generation proof exists. The safety path needs a strict adapter
that consumes context-builder output and produces parseable result candidates.

## Checklist

- [ ] Define text model adapter request/result.
- [ ] Consume context-builder output only.
- [ ] Reject raw OS/browser/network/screen input.
- [ ] Include model/runtime refs.
- [ ] Include prompt/template version.
- [ ] Return raw model output only inside parser boundary.

## Proof

- Adapter contract tests.
- No direct scan security test.
- Local chat proof adapted to safety dry-run path.
