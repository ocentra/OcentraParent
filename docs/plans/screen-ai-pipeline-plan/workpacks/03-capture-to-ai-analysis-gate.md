# 03 - Capture To AI Analysis Gate

## Target State

Captured evidence flows into OCR, guided VLM, local text model, or deterministic
analysis through the AI queue/router.

## Checklist

- [ ] Capture ref enters AI context builder.
- [ ] Provider route recorded.
- [ ] OCR runs when text can answer.
- [ ] VLM runs only for guided visual classification.
- [ ] Text model consumes typed context only.
- [ ] Deterministic route skips model when structured evidence is enough.

## Proof

- AI context artifact.
- Route/runtime artifact.
- AI result artifact.
- Degraded/unknown proof where expected.
