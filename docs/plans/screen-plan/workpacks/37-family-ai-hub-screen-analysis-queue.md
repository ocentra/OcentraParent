# 37 Family AI Hub Screen Analysis Queue

## Target State

Heavier analysis moves to a local/family hub queue before any remote/API path.

## MVP Boundary

This is AI-pass and architecture-alignment work. Capture MVP should expose route state for family-hub-required cases.

## Checklist

- [x] Define family hub availability state.
- [x] Define local-network custody boundary.
- [x] Define summary/image transfer rules if any.
- [x] Prefer redacted/cropped input.
- [x] Record parent approval requirements.
- [x] Add fallback to manual-required when hub unavailable.

## Proof

- Family hub route contract.
- Custody and no-remote-default proof.

Proof command:

```powershell
node scripts/test/screen-family-ai-hub-routing-proof.mjs
```

Proof artifact:

```text
output/screen-plan-proof/37-family-ai-hub-screen-analysis-queue/proof-summary.json
```

## Non-Claims

- No real LAN family hub runtime, discovery protocol, or relay is implemented.
- No production OCR/VLM model quality is claimed.
- No remote/API child-safety route, policy decision, portal UI, or enforcement
  adapter is claimed.
