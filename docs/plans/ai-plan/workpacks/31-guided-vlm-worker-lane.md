# 31 - Guided VLM Worker Lane

## Target State

Guided VLM answers narrowly scoped visual safety questions from approved local
screen jobs and returns schema-valid visual evidence.

## Where We Are

VLM is planning-only until OCR baseline and screen queue/deletion proof exist.

## Checklist

- [ ] Define guided question set.
- [ ] Define VLM job/result contracts.
- [ ] Limit input to approved capture scope.
- [ ] Record confidence and unknown/degraded reasons.
- [ ] Delete raw image after result.
- [ ] Feed typed summary into context builder.
- [ ] Prove guided VLM on a real browser-use capture when OCR/structured evidence is insufficient.
- [ ] Prove guided VLM on a real app/game capture when OCR/structured evidence is insufficient.

## Proof

- VLM parser tests.
- Permission/deletion tests.
- Guided question fixture proof.
- Real capture guided VLM proof artifacts.
