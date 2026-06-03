# 38 - Screen OCR VLM Router Lane

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
