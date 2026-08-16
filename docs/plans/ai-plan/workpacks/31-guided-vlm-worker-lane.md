# 31 - Guided VLM Worker Lane

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `31 - Guided VLM Worker Lane`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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
