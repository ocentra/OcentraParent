# 40 Detector Prompt Packs And Schema Tests

## Target State

Detector-specific JSON prompts replace open-ended screen descriptions.

## MVP Boundary

Capture MVP should reserve detector IDs and result contracts. Prompt quality proof belongs to AI-pass work.

## Checklist

- [x] Define detector IDs.
- [x] Define output schema.
- [x] Add social/video/chat/game/school/bypass/adult/violence/payment/signup detectors.
- [x] Forbid private messages, names, credentials, and full OCR text by default.
- [x] Add malformed output tests.
- [x] Add confidence and uncertainty reason tests.

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
