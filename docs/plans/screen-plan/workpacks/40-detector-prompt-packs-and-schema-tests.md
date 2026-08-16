# 40 Detector Prompt Packs And Schema Tests

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `40 Detector Prompt Packs And Schema Tests`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Target State

Detector-specific JSON prompts replace open-ended screen descriptions.

## MVP Boundary

Capture MVP should reserve detector IDs and result contracts. Prompt quality proof belongs to AI-pass work.

## Checklist

- [ ] Define detector IDs.
- [ ] Define output schema.
- [ ] Add social/video/chat/game/school/bypass/adult/violence/payment/signup detectors.
- [ ] Forbid private messages, names, credentials, and full OCR text by default.
- [ ] Add malformed output tests.
- [ ] Add confidence and uncertainty reason tests.

## Proof

- Prompt pack tests.
- Schema validation tests.
- Negative privacy output tests.

Proof command:

```powershell
node scripts/test/screen-detector-prompt-pack-proof.mjs
```

Proof artifact:

```text
output/screen-plan-proof/40-detector-prompt-packs-and-schema-tests/proof-summary.json
```

## Non-Claims

- No production OCR/VLM model quality is claimed.
- No live model inference is claimed.
- No policy/action/enforcement execution is claimed.
