# 40 Detector Prompt Packs And Schema Tests

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
