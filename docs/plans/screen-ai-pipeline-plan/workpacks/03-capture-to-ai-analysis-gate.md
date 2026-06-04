# 03 - Capture To AI Analysis Gate

## Target State

Captured evidence flows into OCR, guided VLM, local text model, or deterministic
analysis through the AI queue/router.

## Checklist

- [x] Capture ref enters AI context builder.
- [x] Provider route recorded.
- [x] OCR runs when text can answer.
- [x] VLM runs only for guided visual classification.
- [x] Text model consumes typed context only.
- [x] Deterministic route skips model when structured evidence is enough.

## Proof

- AI context artifact.
- Route/runtime artifact.
- AI result artifact.
- Degraded/unknown proof where expected.
- `output/screen-ai-pipeline-proof/service-analysis/proof-summary.json`
  proves the service-owned encrypted queue job, adapter runtime route,
  `localVision` read-model row, evidence digest, policy eligibility, and queue
  deletion path for one captured active-window job.
- `output/screen-ai-pipeline-proof/service-native-game-analysis/proof-summary.json`
  proves a service-owned native foreground active-window capture can flow
  through the service-owned local adapter analysis runtime into a
  `localVision` game read-model row, preserving the queue job, capture reason,
  active-window scope, digest, policy eligibility, and queue deletion path.
- `output/screen-ai-pipeline-proof/ocr-route/proof-summary.json`
- `output/screen-ai-pipeline-proof/local-text-route/proof-summary.json`
- `output/screen-ai-pipeline-proof/deterministic-route/proof-summary.json`
- `output/ai-plan-proof/real-analysis/proof-summary.json`
