# 31 Screen Intelligence Router

## Target State

Router chooses the cheapest safe route before capture, OCR, VLM, family hub, or remote redacted-only fallback.

## MVP Boundary

This workpack is required for the capture MVP. It does not select the final OCR/VLM model.

## Checklist

- [ ] Define route enum.
- [ ] Define route input contract.
- [ ] Check browser/app/game/network/session evidence before capture.
- [ ] Route managed browser to structured extraction first.
- [ ] Route native app/game/launcher/unknown process to active-window or selected-window path when allowed.
- [ ] Return manual-required/unavailable when capture is not allowed.
- [ ] Add policy question and sensitivity fields.

## Proof

- Contract tests for route decisions.
- Router examples for browser, native game, launcher, unknown process, and no-screen-needed.
