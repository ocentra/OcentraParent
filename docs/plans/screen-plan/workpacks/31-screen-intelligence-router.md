# 31 Screen Intelligence Router

## Target State

Router chooses the cheapest safe route before capture, OCR, VLM, trusted household mesh provider, or remote redacted-only fallback.

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

Current proof: `output/screen-plan-proof/31-32-screen-router-structured-extraction/proof-summary.json`.

Non-claims: this is a contract/proof route planner only. It does not claim live managed-browser producer integration,
real DOM/accessibility capture, screenshot capture, portal UI, policy execution, enforcement, or product-complete
pipeline closure.
