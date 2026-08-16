# 14 - Local Text LLM Adapter Boundary

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `14 - Local Text LLM Adapter Boundary`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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
