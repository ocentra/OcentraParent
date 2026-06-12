# 39 - Device Hardware Model Fit Lane

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `39 - Device Hardware Model Fit Lane`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Parents and developers can see whether the child device can run the selected
local model tasks.

## Where We Are

AI UI notes mention hardware fit. Runtime acceleration config and device details
exist, but model fit must be explicit for text/OCR/VLM/embedding tasks.

## Checklist

- [ ] Capture CPU/RAM/GPU capability refs.
- [ ] Map model/task requirements.
- [ ] Add fit states: fits, maybe, too large, unsupported, unknown.
- [ ] Include acceleration settings.
- [ ] Expose status in portal.

## Proof

- Hardware/model fit tests.
- Runtime status proof.
- Portal screenshot if UI changes.
