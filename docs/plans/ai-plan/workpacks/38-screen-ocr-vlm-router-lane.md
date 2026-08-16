# 38 - Screen OCR VLM Router Lane

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `38 - Screen OCR VLM Router Lane`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Screen intelligence chooses deterministic, OCR, VLM, or local text model routes
by capture scope, policy need, permission, and available provider.

## Where We Are

Screen plan has router workpacks. This AI workpack owns shared AI route/result
contract alignment.

## Checklist

- [ ] Add screen task route matrix.
- [ ] Prefer OCR for visible text.
- [ ] Use VLM only for guided visual questions.
- [ ] Use text LLM only over OCR/screen summary JSON.
- [ ] Add unsupported/protected/permission states.
- [ ] Add deletion proof refs.
- [ ] Route real browser-use capture artifacts through structured, OCR, VLM, or text fallback.
- [ ] Route real app-use capture artifacts through active-window screen summary.
- [ ] Route timed cadence capture sequences without queue flood.

## Proof

- Screen OCR AI summary test.
- VLM guided router test.
- Raw screenshot API guard test.
- Real capture plus AI route proof.
