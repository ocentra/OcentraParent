# 31 Screen Intelligence Router

## Target State

Router chooses the cheapest safe route before capture, OCR, VLM, family hub, or remote redacted-only fallback.

## MVP Boundary

This workpack is required for the capture MVP. It does not select the final OCR/VLM model.

## Checklist

- [x] Define route enum.
- [x] Define route input contract.
- [x] Check browser/app/game/network/session evidence before capture.
- [x] Route managed browser to structured extraction first.
- [x] Route native app/game/launcher/unknown process to active-window or selected-window path when allowed.
- [x] Return manual-required/unavailable when capture is not allowed.
- [x] Add policy question and sensitivity fields.

## Proof

- Contract tests for route decisions.
- Router examples for browser, native game, launcher, unknown process, and no-screen-needed.

## Current Proof

Command:

```powershell
node scripts/test/screen-intelligence-router-proof.mjs
```

Artifact:

```text
output/screen-plan-proof/31-screen-intelligence-router/proof-summary.json
```

Non-claims:

- No live browser CDP capture is performed by this proof.
- No browser provider fallback, network runtime, OCR/VLM quality, policy action, or enforcement action is claimed.
- No raw screenshot retention or remote screenshot upload is permitted by these contracts.
