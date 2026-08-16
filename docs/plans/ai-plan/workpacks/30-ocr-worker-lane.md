# 30 - OCR Worker Lane

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `30 - OCR Worker Lane`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

OCR extracts visible text from approved local screen jobs and produces a typed,
source-cited screen text summary.

## Where We Are

Screen plan defines capture scopes and screen intelligence routing. OCR execution
needs worker contracts, model/tool selection, and deletion proof.

## Checklist

- [ ] Define OCR job contract.
- [ ] Define OCR result contract.
- [ ] Link to screen evidence refs and image digest.
- [ ] Prove temporary image deletion.
- [ ] Route OCR summary into context builder.
- [ ] Add unavailable/permission-required states.
- [ ] Prove OCR on a real browser-use capture artifact.
- [ ] Prove OCR on a real app-use capture artifact.
- [ ] Prove OCR on a timed cadence capture sequence artifact.

## Proof

- OCR worker tests.
- Screen summary integration test.
- Raw image deletion proof.
- Real capture OCR proof artifacts under
  `output/ai-plan-proof/screen-winrt-ocr-worker/proof-summary.json`.
- Context-builder replay proof artifacts under
  `output/ai-plan-proof/screen-summary-ai-context/proof-summary.json`.
- Portal screenshot if screen UI changes.
