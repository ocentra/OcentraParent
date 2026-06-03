# 30 - OCR Worker Lane

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
- Real capture OCR proof artifacts.
- Portal screenshot if screen UI changes.
